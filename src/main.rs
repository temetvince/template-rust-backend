use axum::Router;
use sqlx::sqlite::SqlitePool;
use tokio::sync::OnceCell;

mod handlers;
mod models;
mod db;

use handlers::user_routes;

type SharedConnection = SqlitePool;
static DATABASE_URL: &str = "sqlite://DB.db";
static DB_POOL: OnceCell<SharedConnection> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(DATABASE_URL).await.expect("Failed to connect to database");
    sqlx::migrate!().run(&pool).await.expect("Failed to apply migrations");

    DB_POOL.set(pool.clone()).expect("Failed to set DB pool");

    let app = Router::new()
        .merge(user_routes(pool.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
