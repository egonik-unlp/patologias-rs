use chrono::{DateTime, Utc};
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    query_builder::AsChangeset,
    Selectable,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::users::dto::{NewSessionDto, RoleDto, SessionDto, UserDto};

#[derive(Debug, Deserialize, Serialize, DbEnum, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[ExistingTypePath = "crate::core::database::schema::sql_types::Roles"]
pub enum Role {
    Admin,
    Operator,
}
#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Selectable, PartialEq)]
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

#[derive(
    Debug,
    Clone,
    Deserialize,
    Serialize,
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    PartialEq,
)]
#[diesel(belongs_to(User))]
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

impl From<Role> for RoleDto {
    fn from(value: Role) -> Self {
        match value {
            Role::Operator => RoleDto::Operator,
            Role::Admin => RoleDto::Admin,
        }
    }
}
impl From<User> for UserDto {
    fn from(value: User) -> Self {
        let User {
            id,
            username,
            full_name,
            role,
            active,
            ..
        } = value;
        let role = role.into();

        UserDto {
            username,
            full_name,
            id,
            role,
            active,
        }
    }
}

impl From<Session> for SessionDto {
    fn from(value: Session) -> Self {
        let Session {
            id,
            token,
            user_id,
            expires_at,
        } = value;
        SessionDto {
            id,
            token,
            user_id,
            expires_at,
        }
    }
}
impl From<NewSessionDto> for NewSession {
    fn from(value: NewSessionDto) -> Self {
        let NewSessionDto {
            token,
            user_id,
            expires_at,
        } = value;
        NewSession {
            token,
            expires_at,
            user_id,
        }
    }
}
