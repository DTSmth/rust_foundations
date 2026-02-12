// use std::net::SocketAddr;
// use axum::Router;
// use axum::routing::get;
// use axum::response::Html;
// use serde::Serialize;
//
// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(say_hello_text));
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
//
// #[derive(Serialize)]
// struct MessageJson {
//     message: String,
// }
//
// async fn say_hello_text() ->Html<String> {
//     // const HTML: &str = include_str!("hello.html");
//     let path = std::path::Path::new("src/hello.html");
//     let content = tokio::fs::read_to_string(path).await.unwrap();
//     Html(content)
// }
//
// async fn hello() -> axum::Json<MessageJson> {
//     let message = MessageJson { message: "Hello, Json!".to_string() };
//     axum::Json(message)
// }

fn main() {

}
