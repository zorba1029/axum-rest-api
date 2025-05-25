use axum::{
    body::Body,
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
// use serde_json::{json, Value};
use serde_json::json;
use sqlx::{mysql::MySqlPool, Row};
use crate::models::{User, UserItem, CreateUserRequest};
use crate::AppState;
use std::sync::Arc;
use crate::AppError;

//-- 테스트 코드 ----------------
#[utoipa::path(
    post,
    path = "/create-user",
    responses(
        (status = 201, description = "User created successfully", body = UserItem)
    )
)]
pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User Created Successfully"))
        .unwrap()
}

//-- DB 연동 테스트 코드 ----------------
#[utoipa::path(
    post,
    path = "/create-user-db",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully in DB", body = serde_json::Value),
        (status = 500, description = "Failed to create user", body = serde_json::Value)
    )
)]
pub async fn create_user_db(
    Extension(db_pool): Extension<MySqlPool>,
    Json(user_data): Json<CreateUserRequest>,
// ) -> impl IntoResponse {
) -> Result<impl IntoResponse, AppError> {
    match sqlx::query("INSERT INTO axum_users (name, email) VALUES (?, ?)")
        .bind(&user_data.name)
        .bind(&user_data.email)
        .execute(&db_pool)
        .await {
            // Ok(_) => (
            //     StatusCode::CREATED,
            //     Json(json!({
            //         "message": "User Created Successfully",
            //         "user" : {
            //             "name": user_data.name,
            //             "email": user_data.email,
            //         }
            //     }))
            // ).into_response(),
            // Err(e) => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     Json(json!({
            //         "error": format!("Failed to create user: {}", e)
            //     }))
            // ).into_response(),
            Ok(_) => Ok((
                StatusCode::CREATED,
                Json(json!({
                    "message": "User Created Successfully",
                    "user" : {
                        "name": user_data.name,
                        "email": user_data.email,
                    }
                }))
            ).into_response()),
            Err(e) => Err(AppError::InternalServerError(format!(
                "Failed to create user - {}", 
                e
            ))),
        }
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
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

#[utoipa::path(
    delete,
    path = "/delete-user/{id}",
    params(
        ("id" = i32, Path, description = "User id to delete")
    ),
    responses(
        (status = 200, description = "User deleted (placeholder response)", body = UserItem),
        (status = 500, description = "Failed to delete user", body = String)
    )
    // tags = ["User (Test)"]
)]
// pub async fn delete_user(Path(user_id): Path<i32>) -> Result<Json<UserItem>, impl IntoResponse> {
pub async fn delete_user(Path(user_id): Path<i32>) -> Result<Json<UserItem>, AppError> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(UserItem { 
            id: user_id,
            name: "".to_string(),
        })),
        // Err(e) => Err((
        //     StatusCode::INTERNAL_SERVER_ERROR,
        //     format!("Failed to delete user: {}", e),
        // )),
        Err(e) => Err(AppError::UserNotFound(user_id, e.to_string())),
    }
}

async fn perform_delete_user(user_id: i32) -> Result<(), String> {
    if user_id == 1 {
        Err("User Can NOT be deleted".to_string())
    } else {
        Ok(())
    }
}

//-- DB 연동 테스트 코드 ----------------
#[utoipa::path(
    get,
    path = "/axum-users",
    responses(
        (status = 200, description = "List of users from DB", body = Vec<User>),
        (status = 500, description = "Failed to fetch users", body = String)
    )
)]
// pub async fn list_users_db(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
pub async fn list_users_db(Extension(db_pool): Extension<MySqlPool>) -> Result<impl IntoResponse, AppError> {
    // let rows = match sqlx::query("SELECT id, name, email FROM axum_users")
    //     .fetch_all(&db_pool)
    //     .await {
    //         Ok(rows) => rows,
    //         Err(_) => {
    //             return (
    //                 StatusCode::INTERNAL_SERVER_ERROR,
    //                 "Failed to fetch users from DB",
    //             ).into_response();
    //         }
    //     };

    // let axum_users: Vec<User> = rows
    //     .into_iter()
    //     .map(|row| {
    //         User {
    //             id: row.try_get::<i32, _>("id").unwrap_or_default(),
    //             name: row.try_get::<String, _>("name").unwrap_or_default(),
    //             email: row.try_get::<String, _>("email").unwrap_or_default(),
    //         }
    //     })
    //     .collect();
    
    // (StatusCode::OK, Json(axum_users)).into_response()

    let result = sqlx::query("SELECT id, name, email FROM axum_users")
        .fetch_all(&db_pool)
        .await;

    match result {
        Ok(rows) => {
            let axum_users: Vec<User> = rows
                .into_iter()
                .map(|row| {
                    User {
                        id: row.try_get::<i32, _>("id").unwrap_or_default(),
                        name: row.try_get::<String, _>("name").unwrap_or_default(),
                        email: row.try_get::<String, _>("email").unwrap_or_default(),
                    }
                })
                .collect();
            Ok((StatusCode::OK, Json(axum_users)).into_response())
        }
        Err(e) => Err(AppError::InternalServerError(format!(
            "Failed to fetch users from DB - {}",
            e
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/admin/get_app_state",
    responses(
        (status = 200, description = "Get App State", body = serde_json::Value)
    ),
    security(
        ("ApiKeyAuth" = [])
    )
    // tags = ["Admin"]
)]
pub async fn get_app_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "db_name": state.db_name.clone(),
            "db_user": state.db_user.clone(),
            "db_host": state.db_host.clone(),
            "db_port": state.db_port.clone(),
            "db_user": state.db_user.clone(),
            "server_host": state.server_host.clone(),
            "server_port": state.server_port.clone(),
        }))
    )
}

// [
//  {
//     "email": "alice.smith@example.com",
//     "id": 1,
//     "name": "Alice Smith"