use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserItem {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Page {
    pub number: u32,
}

#[derive(Deserialize)]
pub struct BodyItem {
    pub title: String,
}


#[derive(Deserialize)]  // JSON 요청을 받기 위해 필요
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}