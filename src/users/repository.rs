use anyhow::Context;

use crate::core::database::connection::interact;
use crate::core::database::schema::{sessions, users};
use crate::impl_repository;
use crate::users::model::{NewSession, NewUser, Session, User};

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
}
