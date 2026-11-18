use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUserDto {
    pub username: String,
    pub full_name: String,
    pub role: RoleDto,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub role: RoleDto,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RoleDto {
    Operator,
    Admin,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionDto {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewSessionDto {
    pub token: String,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}
