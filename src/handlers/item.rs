use axum::extract::{Json, Path, Query};
use crate::models::{Page, BodyItem};
use crate::AppError;

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
pub async fn show_item(Path(id): Path<i32>, Query(params): Query<Page>) -> Result<String, AppError> {
    // format!("Item ID: {}, Page Number: {}", id, params.number)
    match find_item(id).await {
        Ok(_) => Ok(format!("Item ID: {}, Page Number: {}", id, params.number)),
        Err(e) => Err(AppError::InvalidInput(id, e.to_string())),
    }
}

async fn find_item(id: i32) -> Result<(), String> {
    if id == 1 {
        Err("Item Not Found".to_string())
    } else {
        Ok(())
    }
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
