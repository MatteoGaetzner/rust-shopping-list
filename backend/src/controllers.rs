use crate::database::ShoppingItem;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use model::{CreateListResponse, PostShopItem, ShoppingListItem};
use uuid::Uuid;

use crate::Database;

pub async fn get_items(
    State(state): State<Database>,
    Path(list_uuid): Path<Uuid>,
) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state.read().unwrap().as_vec(&list_uuid.to_string());

    Json(items)
}

pub async fn add_item(
    State(state): State<Database>,
    Path(list_uuid): Path<Uuid>,
    Json(post_request): Json<PostShopItem>,
) -> impl IntoResponse {
    let item = ShoppingItem {
        title: post_request.title.clone(),
        creator: post_request.posted_by.clone(),
    };
    let uuid = Uuid::new_v4().to_string();

    let Ok(mut db) = state.write() else {
        return (StatusCode::SERVICE_UNAVAILABLE).into_response();
    };

    db.insert_item(&list_uuid.to_string(), &uuid, item);

    (
        StatusCode::OK,
        Json(ShoppingListItem {
            title: post_request.title,
            posted_by: post_request.posted_by,
            uuid,
        }),
    )
        .into_response()
}

pub async fn delete_item(
    State(state): State<Database>,
    Path((list_uuid, item_uuid)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    db.delete_item(&list_uuid.to_string(), &item_uuid.to_string());
    StatusCode::OK
}

pub async fn create_shopping_list(State(state): State<Database>) -> impl IntoResponse {
    let uuid = Uuid::new_v4().to_string();
    state.write().unwrap().create_list(&uuid);
    Json(CreateListResponse { uuid })
}
