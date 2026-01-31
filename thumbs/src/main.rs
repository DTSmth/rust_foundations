use std::net::SocketAddr;
use std::path::Path;
use axum::{Extension, Router};
use axum::extract::Multipart;
use axum::response::Html;
use axum::routing::{get, post};
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
        .route("/upload", post(uploader))
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

async fn insert_image_into_database(pool: &SqlitePool, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("INSERT INTO images (tags) VALUES(? RETURNING id)")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}



async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart
) -> String {
    let mut tags = None; // "None" means "no tags yet"
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), // Using Some means we can check we received it
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) { // Destructuring both Options at on
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
    } else {
        panic!("Missing field");
    }

    "Ok".to_string()
}
