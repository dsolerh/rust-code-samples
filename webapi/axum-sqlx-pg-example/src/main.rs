use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod api;
mod error;
mod models;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // NOTE: middlewares (layers) are excecuted from bottom to top
    let router_hello = Router::new()
        .merge(handlers_hello())
        .merge(api::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .merge(handlers_static());

    // region: --- Start server
    let addr = "0.0.0.0:8091";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("->> Listening on: {addr}\n");
    axum::serve(listener, router_hello).await.unwrap();
    // region: --- Start server
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

// region: --- Handlers hello
fn handlers_hello() -> Router {
    Router::new()
        .route(
            "/hello",
            get(|| async { Html("Hello <strong>World!!<strong/>") }),
        )
        .route("/hello_query", get(handler_hello_query))
        .route("/hello_path/{name}", get(handler_hello_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello_query", "HANDLER");
    println!("  --> params: {:?}", params);
    Html(format!(
        "Hello <strong>{}<strong/>",
        params.name.as_deref().unwrap_or("World!!")
    ))
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello_path", "HANDLER");
    println!("  --> path:name: {:?}", name);
    Html(format!("Hello <strong>{}<strong/>", name))
}
// region: --- Handlers hello

// region: --- Handlers static
fn handlers_static() -> Router {
    Router::new().nest_service("/content", get_service(ServeDir::new("./")))
}
// region: --- Handlers static
