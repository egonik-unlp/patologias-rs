//! Guards for protected server fns: call one at the start of the body.
use leptos::prelude::*;

use crate::users::{dto::UserDto, server::get_current_user};

/// Para el inicio de cada server fn protegida: devuelve el usuario logueado,
/// o redirige a `/login` y corta con error.
pub async fn require_user() -> Result<UserDto, ServerFnError> {
    match get_current_user().await? {
        Some(user) => Ok(user),
        None => {
            leptos_axum::redirect("/login");
            Err(ServerFnError::new("No autenticado"))
        }
    }
}

pub async fn require_admin() -> Result<UserDto, ServerFnError> {
    use crate::users::dto::RoleDto;

    match get_current_user().await? {
        Some(user) if matches!(user.role, RoleDto::Admin) => Ok(user),
        Some(_) => Err(ServerFnError::new("Not admin")),
        None => {
            leptos_axum::redirect("/login");
            Err(ServerFnError::new("No autenticado"))
        }
    }
}
