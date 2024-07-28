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

pub async fn init() -> Result<(), sqlx::Error> {
    let pool = get_db_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
