use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    async fn setup_test_db() -> sqlx::PgPool {
        dotenv().ok();

        let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&test_database_url)
            .await
            .expect("Failed to connect to the test database")
    }

    #[tokio::test]
    async fn test_get_db_pool() {
        let pool = setup_test_db().await;
        assert!(pool.acquire().await.is_ok(), "Failed to acquire a connection");
    }
}

pub async fn init() -> Result<(), sqlx::Error> {
    let pool = get_db_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
