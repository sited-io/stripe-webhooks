use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use stripe_webhooks::{
    get_cors, get_env_var, init_db_pool, init_routes, migrate, AppSettings,
    EventService, Publisher,
};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging
    tracing_subscriber::fmt::init();

    // get required environment variables
    let host = get_env_var("HOST");

    // initialize database connection and migrate
    let db_pool = init_db_pool(
        get_env_var("DB_HOST"),
        get_env_var("DB_PORT").parse().unwrap(),
        get_env_var("DB_USER"),
        get_env_var("DB_PASSWORD"),
        get_env_var("DB_DBNAME"),
        std::env::var("DB_ROOT_CERT").ok(),
    )?;

    migrate(&db_pool).await?;

    // get AppSettings
    let app_settings = AppSettings::new(get_env_var("STRIPE_ENDPOINT_SECRET"));

    // initialize NATS publisher
    let publisher = Publisher::new(
        async_nats::ConnectOptions::new()
            .user_and_password(
                get_env_var("NATS_USER"),
                get_env_var("NATS_PASSWORD"),
            )
            .connect(get_env_var("NATS_HOST"))
            .await?,
    );

    let cors_allowed_origins = get_env_var("CORS_ALLOWED_ORIGINS");

    tracing::info!("web server listening on {}", host);

    HttpServer::new(move || {
        let cors = get_cors(cors_allowed_origins.clone());

        // initialize event service
        let event_service =
            EventService::new(db_pool.clone(), publisher.clone());

        App::new()
            .wrap(cors)
            .wrap(Logger::new(
                "from: %{r}a %r %s %b %{Referer}i %{User-Agent}i",
            ))
            .app_data(web::Data::new(app_settings.clone()))
            .app_data(web::Data::new(event_service))
            .configure(init_routes)
    })
    .workers(2)
    .bind(host)
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
