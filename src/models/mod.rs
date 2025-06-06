use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserItem {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct Page {
    pub number: u32,
}

#[derive(Deserialize, ToSchema)]
pub struct BodyItem {
    pub title: String,
}


#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}