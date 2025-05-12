use axum::{
    body::Body,
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::{mysql::MySqlPool, Row};
use crate::models::{User, UserItem};

pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User Created Successfully"))
        .unwrap()
}

pub async fn list_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];

    Json(users)
}

pub async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<UserItem>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(UserItem { 
            id: user_id,
            name: "".to_string(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("User Can NOT be deleted".to_string())
    } else {
        Ok(())
    }
}

pub async fn get_axum_users(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM axum_users")
        .fetch_all(&db_pool)
        .await {
            Ok(rows) => rows,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to fetch users from DB",
                ).into_response();
            }
        };

    let axum_users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (StatusCode::OK, Json(axum_users)).into_response()
}

// [
//  {
//     "email": "alice.smith@example.com",
//     "id": 1,
//     "name": "Alice Smith"