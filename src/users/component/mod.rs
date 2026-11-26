//! User UI. Components are re-exported here so callers keep importing from
//! `users::component`.
mod admin;
mod auth;

pub use admin::{CreateUser, UserList};
pub use auth::{CurrentUser, HandleLogin, LoginUser, LogoutUser};
