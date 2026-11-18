use chrono::{DateTime, Utc};
use diesel::{
    associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset, Selectable,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::users::dto::RoleDto;

#[derive(Debug, Deserialize, Serialize, DbEnum, Clone)]
#[serde(rename_all = "lowercase")]
#[ExistingTypePath = "crate::core::database::schema::sql_types::Roles"]
pub enum Role {
    Admin,
    Operator,
}
#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Selectable)]
#[diesel(table_name= crate::core::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub role: Role,
    pub active: bool,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub full_name: String,
    pub role: Role,
    pub active: bool,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Selectable)]
#[diesel(table_name= crate::core::database::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSession {
    pub token: String,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}

impl From<RoleDto> for Role {
    fn from(value: RoleDto) -> Self {
        match value {
            RoleDto::Operator => Role::Operator,
            RoleDto::Admin => Role::Admin,
        }
    }
}
