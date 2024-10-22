#![allow(unused)]

use axum::{middleware, response::{Html, IntoResponse, Response}, routing::get, Router};
use model::ModelController;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

// with this you can import it in other modules
pub use self::error::{Error, Result};

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new()
    .merge(routes_root())
    .merge(web::routes_login::routes())
    .nest("/api", routes_api)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new());

    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("LISTENING ON ADDR {}\n", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

fn routes_root() -> Router {
    Router::new()
    .route("/", get(handler_root))
}

async fn handler_root() -> impl IntoResponse {
    println!("->> {:<12} - handler_root", "HANDLER");
    Html("Hello World!")
}