use actix_cors::Cors;

mod db;
mod error;
mod events;
mod model;
mod routes;

pub use db::{init_db_pool, migrate, DbError};
pub use error::HttpError;
pub use events::EventService;
pub use routes::init_routes;

pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| {
        panic!("ERROR: Missing environment variable '{var}'")
    })
}

pub fn get_cors(cors_allowed_origins: String) -> Cors {
    if cors_allowed_origins.is_empty() {
        Cors::permissive()
    } else {
        let mut cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        let allowed_origins: Vec<&str> =
            cors_allowed_origins.split(',').collect();

        for origin in allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        cors
    }
}
