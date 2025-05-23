use axum::extract::{Json, Path, Query};
use crate::models::{Page, BodyItem};

#[utoipa::path(
    get,
    path = "/item/{id}",
    params(
        ("id" = i32, Path, description = "Item id"),
        ("number" = Option<i32>, Query, description = "Optional number")
    ),
    responses(
        (status = 200, description = "Show item details", body = String)
    )
    // tags = ["Item"] // 주석 처리
)]
pub async fn show_item(Path(id): Path<i32>, Query(params): Query<Page>) -> String {
    format!("Item ID: {}, Page Number: {}", id, params.number)
}

#[utoipa::path(
    post,
    path = "/add-item",
    request_body = BodyItem,
    responses(
        (status = 201, description = "Item added successfully", body = String)
    )
    // tags = ["Item"] // 주석 처리
)]
pub async fn add_item(Json(item): Json<BodyItem>) -> String {
    format!("Item added: {}", item.title)
}
