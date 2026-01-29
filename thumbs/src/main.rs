use std::net::SocketAddr;
use std::path::Path;
use axum::{Extension, Router};
use axum::response::Html;
use axum::routing::get;
use sqlx::{Row, SqlitePool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //Read the .env and build env variables
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    //Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Build Axum with an "extension" to hold the database connection pool
    let app = Router::new()
        .route("/", get(index_page))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}

// async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
//     let result = sqlx::query("SELECT COUNT(id) FROM images")
//         .fetch_one(&pool)
//         .await
//         .unwrap();
//     let count = result.get::<i64, _>(0);
//     format!("{count} images in the database")
// }

async fn index_page() -> Html<String> {
    let path = Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
