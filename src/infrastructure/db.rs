use crate::config::application::DbConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, PgPool, Pool, Postgres};

pub async fn connection(config: &DbConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_url())
        .await
        .expect(
            format!(
                "Failed to connect to database: {}",
                &config.connection_url()
            )
            .as_str(),
        )
}
