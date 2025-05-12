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
