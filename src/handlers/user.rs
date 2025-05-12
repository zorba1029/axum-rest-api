use axum::{
    body::Body,
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::{mysql::MySqlPool, Row};
use crate::models::{User, UserItem, CreateUserRequest};
use crate::AppState;
use std::sync::Arc;

//-- 테스트 코드 ----------------
pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User Created Successfully"))
        .unwrap()
}

//-- DB 연동 테스트 코드 ----------------
pub async fn create_user_db(
    Extension(db_pool): Extension<MySqlPool>,
    Json(user_data): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match sqlx::query("INSERT INTO axum_users (name, email) VALUES (?, ?)")
        .bind(&user_data.name)
        .bind(&user_data.email)
        .execute(&db_pool)
        .await {
            Ok(_) => (
                StatusCode::CREATED,
                Json(json!({
                    "message": "User Created Successfully",
                    "user" : {
                        "name": user_data.name,
                        "email": user_data.email,
                    }
                }))
            ).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Failed to create user: {}", e)
                }))
            ).into_response(),
        }
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

//-- DB 연동 테스트 코드 ----------------
pub async fn list_users_db(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
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

pub async fn get_app_state(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "db_name": app_state.db_name.clone(),
            "db_user": app_state.db_user.clone(),
            "db_host": app_state.db_host.clone(),
            "db_port": app_state.db_port.clone(),
            "db_user": app_state.db_user.clone(),
            "server_host": app_state.server_host.clone(),
            "server_port": app_state.server_port.clone(),
        }))
    )
}

// [
//  {
//     "email": "alice.smith@example.com",
//     "id": 1,
//     "name": "Alice Smith"