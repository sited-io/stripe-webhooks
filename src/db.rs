use std::ops::DerefMut;

use deadpool_postgres::tokio_postgres::error::SqlState;
use deadpool_postgres::{
    tokio_postgres::NoTls, Config, CreatePoolError, Pool, PoolError, Runtime,
    SslMode,
};
use refinery::Target;
use reqwest::StatusCode;

use crate::HttpError;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[derive(Debug)]
pub enum DbError {
    TokioPostgres(deadpool_postgres::tokio_postgres::Error),
    Pool(PoolError),
    CreatePool(CreatePoolError),
    SeaQuery(sea_query::error::Error),
}

impl DbError {
    pub fn ignore_to_ts_query<T>(self, default: T) -> Result<T, Self> {
        if let Self::TokioPostgres(err) = &self {
            if let Some(err) = err.as_db_error() {
                if *err.code() == SqlState::SYNTAX_ERROR
                    && err.routine() == Some("toTSQuery")
                {
                    tracing::log::warn!("{:?}", err);
                    return Ok(default);
                }
            }
        }

        Err(self)
    }
}

impl From<deadpool_postgres::tokio_postgres::Error> for DbError {
    fn from(err: deadpool_postgres::tokio_postgres::Error) -> Self {
        Self::TokioPostgres(err)
    }
}

impl From<PoolError> for DbError {
    fn from(err: PoolError) -> Self {
        Self::Pool(err)
    }
}

impl From<CreatePoolError> for DbError {
    fn from(err: CreatePoolError) -> Self {
        Self::CreatePool(err)
    }
}

impl From<sea_query::error::Error> for DbError {
    fn from(err: sea_query::error::Error) -> Self {
        Self::SeaQuery(err)
    }
}

impl From<DbError> for HttpError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::TokioPostgres(tp_err) => {
                if let Some(err) = tp_err.as_db_error() {
                    match *err.code() {
                        SqlState::UNIQUE_VIOLATION => HttpError::from_message(
                            StatusCode::CONFLICT,
                            err.message(),
                        ),
                        SqlState::SYNTAX_ERROR => {
                            tracing::log::error!("{err:?}");
                            HttpError::internal()
                        }
                        SqlState::FOREIGN_KEY_VIOLATION => {
                            HttpError::from_message(
                                StatusCode::CONFLICT,
                                err.message(),
                            )
                        }
                        _ => {
                            tracing::log::error!("{err:?}");
                            HttpError::internal()
                        }
                    }
                } else {
                    tracing::log::error!("{tp_err:?}");
                    HttpError::internal()
                }
            }
            DbError::Pool(pool_err) => {
                tracing::log::error!("{pool_err:?}");
                HttpError::internal()
            }
            DbError::CreatePool(create_pool_err) => {
                tracing::log::error!("{create_pool_err:?}");
                HttpError::internal()
            }
            DbError::SeaQuery(sea_query_err) => {
                tracing::log::error!("{sea_query_err:?}");
                HttpError::internal()
            }
        }
    }
}

pub fn init_db_pool(
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
) -> Result<Pool, CreatePoolError> {
    let mut config = Config::new();
    config.host = Some(host);
    config.port = Some(port);
    config.user = Some(user);
    config.password = Some(password);
    config.dbname = Some(dbname);

    config.ssl_mode = Some(SslMode::Prefer);

    config.create_pool(Some(Runtime::Tokio1), NoTls)
}

pub async fn migrate(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = pool.get().await?;

    let runner = embedded::migrations::runner();
    runner
        .set_target(Target::Latest)
        .run_async(client.deref_mut().deref_mut())
        .await?;

    Ok(())
}
