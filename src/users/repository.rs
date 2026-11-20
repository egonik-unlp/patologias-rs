use anyhow::Context;

use crate::{
    core::database::{
        connection::interact,
        schema::{sessions, users},
    },
    impl_repository,
    users::model::{NewSession, NewUser, Session, User},
};
use leptos::logging::log;

impl_repository!(async, User, users, i32, NewUser);
impl_repository!(
    async,
    Session,
    sessions,
    i32,
    NewSession,
    belongs_to(user_id: i32)
);

impl UserRepository {
    /// `None` si no existe un usuario con ese nombre.
    pub async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let username = username.to_owned();
        interact(&self.pool, move |conn| {
            use diesel::prelude::*;
            users::table
                .filter(users::username.eq(username))
                .select(User::as_select())
                .first(conn)
                .optional()
                .context("finding user by username")
        })
        .await
    }
}
impl SessionRepository {
    pub async fn get_session_by_user(&self, username: &str) -> anyhow::Result<Option<Session>> {
        let username = username.to_owned();
        log!("Get session by user called with = {}", username);
        interact(&self.pool, move |conn| {
            use diesel::prelude::*;
            let user = users::table
                .filter(users::username.eq(username))
                .select(User::as_select())
                .first(conn)
                .context("finding user by username")?;
            Session::belonging_to(&user)
                .select(Session::as_select())
                .get_result(conn)
                .optional()
                .context("finding sesion by user")
        })
        .await
    }

    // src/users/repository.rs, in impl SessionRepository
    /// Usuario dueño de una sesión vigente. `None` si el token no existe,
    /// la sesión venció o el usuario está inactivo.
    pub async fn get_user_by_session(&self, token: &str) -> anyhow::Result<Option<User>> {
        let token = token.to_owned();
        interact(&self.pool, move |conn| {
            use diesel::prelude::*;
            sessions::table
                .inner_join(users::table)
                .filter(sessions::token.eq(token))
                .filter(sessions::expires_at.gt(chrono::Utc::now()))
                .filter(users::active.eq(true))
                .select(User::as_select())
                .first(conn)
                .optional()
                .context("finding user by session token")
        })
        .await
    }
    pub async fn get_session_by_token(&self, token: &str) -> anyhow::Result<Option<Session>> {
        let token = token.to_owned();
        interact(&self.pool, move |conn| {
            use diesel::prelude::*;
            sessions::table
                .filter(sessions::token.eq(&token))
                .select(Session::as_select())
                .first(conn)
                .optional()
                .context("Could not find session")
        })
        .await
    }
}
