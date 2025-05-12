use axum::extract::{Json, Path, Query};
use crate::models::{Page, BodyItem};

pub async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item ID: {}, Page Number: {}", id, page.number)
}

pub async fn add_item(Json(body_item): Json<BodyItem>) -> String {
    format!("Item added: {}", body_item.title)
}