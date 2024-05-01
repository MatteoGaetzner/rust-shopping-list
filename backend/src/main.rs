use axum::{
    routing::{delete, get},
    Router,
};
use database::InMemoryDatabase;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

mod controllers;
mod database;

type Database = Arc<RwLock<InMemoryDatabase>>;

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new()
        .route("/list", get(controllers::create_shopping_list))
        .route(
            "/list/:list_uuid/items",
            get(controllers::get_items).post(controllers::add_item),
        )
        .route(
            "/list/:list_uuid/items/:item_uuid",
            delete(controllers::delete_item),
        )
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
