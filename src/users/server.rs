use anyhow::Context;
use axum_extra::extract::CookieJar;
use leptos::{logging::log, prelude::*};

use crate::users::dto::{NewUserDto, UserDto};

//TODO: Que la password no viaje por el cable
#[server]
pub async fn create_user(
    user: NewUserDto,
    password: String,
) -> Result<UserDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    ctx.users_service
        .create_user(user, password)
        .await
        .map_err(ServerFnError::new)
        .map(UserDto::from)
}

#[server]
pub async fn login_user(user_name: String, password: String) -> Result<UserDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;

    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    let user = ctx.users_service.get_session_by_user(user_name).await.context("Getting session by user")?;
    match user {
        Some(_) => todo!(),
        None => todo!(),
    }
    let token = ctx.users_service.create_session(O)
    ctx.users_service
        //TODO: Limpiar acá
        .login_user(user_name, password.as_str())
        .await
        .inspect_err(|err| log!("Aca vino en realidad: {}", err))
        .map_err(ServerFnError::new)
        // Acá se podría devolver la session y despues vemos que onda con el token
        .map(UserDto::from)
}
