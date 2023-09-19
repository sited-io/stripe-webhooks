use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use stripe_webhooks::{
    get_cors, get_env_var, init_db_pool, init_routes, migrate,
    CredentialsService, EventService, MediaService,
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
    )?;
    migrate(&db_pool).await?;

    // initialize credentials service
    let credentials_service = CredentialsService::new(
        get_env_var("OAUTH_URL"),
        get_env_var("OAUTH_HOST"),
        get_env_var("SERVICE_USER_CLIENT_ID"),
        get_env_var("SERVICE_USER_CLIENT_SECRET"),
    );

    // initialize media service client
    let media_service = MediaService::init(
        get_env_var("MEDIA_SERVICE_URL"),
        credentials_service,
    )
    .await?;

    // initialize event service
    let event_service = EventService::new(db_pool, media_service);

    let cors_allowed_origins = get_env_var("CORS_ALLOWED_ORIGINS");

    tracing::log::info!("web server listening on {}", host);

    HttpServer::new(move || {
        let cors = get_cors(cors_allowed_origins.clone());

        App::new()
            .wrap(cors)
            .wrap(Logger::new(
                "from: %{r}a %r %s %b %{Referer}i %{User-Agent}i",
            ))
            .app_data(web::Data::new(event_service.clone()))
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
