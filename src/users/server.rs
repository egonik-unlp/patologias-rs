use crate::users::dto::{NewUserDto, UserDto};
use leptos::prelude::*;
//TODO: Que la password no viaje por el cable
#[server]
pub async fn create_user(user: NewUserDto, password: String) -> Result<UserDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use crate::users::dto::RoleDto;

    let logged_user = require_user().await?;
    if !matches!(logged_user.role, RoleDto::Admin) {
        return Err(ServerFnError::new("Solo un admin puede crear usuarios"));
    }
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    ctx.users_service
        .create_user(user, password)
        .await
        .map_err(ServerFnError::new)
        .map(UserDto::from)
}
#[server]
pub async fn get_all_users() -> Result<Vec<UserDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    require_user().await?;
    let ctx = expect_context::<AppState>();
    ctx.users_service
        .get_all()
        .await
        .map_err(ServerFnError::new)
}
#[server]
pub async fn login_user(user_name: String, password: String) -> Result<UserDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use axum::http::{header::SET_COOKIE, HeaderValue};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos::logging::log;
    use leptos_axum::ResponseOptions;

    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    let response = expect_context::<ResponseOptions>();
    let (user, session) = ctx
        .users_service
        .login_user(user_name, password.as_str())
        .await
        .inspect_err(|err| log!("Login fallido: {err:#}"))
        // Mismo mensaje si el usuario no existe o la contraseña está mal.
        .map_err(|_| ServerFnError::new("Usuario o contraseña incorrectos"))?;

    // La cookie vence cuando vence la sesión en la base.
    let max_age = (session.expires_at - chrono::Utc::now()).num_seconds();
    let cookie = Cookie::build(("session_token", session.token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(max_age))
        .build();
    response.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.encoded().to_string()).map_err(ServerFnError::new)?,
    );
    leptos_axum::redirect("/");
    Ok(user)
}

#[server]
pub async fn logout_user() -> Result<(), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use axum::http::{header::SET_COOKIE, HeaderValue};
    use axum_extra::extract::cookie::CookieJar;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos_axum::extract;
    use leptos_axum::ResponseOptions;
    let ctx =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Failed to get AppState"))?;
    let jar: CookieJar = extract().await?;
    if let Some(session) = jar.get("session_token") {
        ctx.users_service
            .logout_user(session.value())
            .await
            .map_err(ServerFnError::new)?;
    }
    let mut cookie = Cookie::build(("session_token", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .build();
    cookie.make_removal(); // empty value, Max-Age=0, Expires in the past

    let header = HeaderValue::from_str(&cookie.to_string()).map_err(ServerFnError::new)?;
    expect_context::<ResponseOptions>().append_header(SET_COOKIE, header);

    leptos_axum::redirect("/login");
    Ok(())
}

/// `None` si no hay cookie, la sesión venció o el usuario está inactivo.
#[server]
pub async fn get_current_user() -> Result<Option<UserDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use axum_extra::extract::cookie::CookieJar;
    use leptos_axum::extract;

    let ctx =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Failed to get AppState"))?;
    let jar: CookieJar = extract().await?;
    let Some(cookie) = jar.get("session_token") else {
        return Ok(None);
    };
    ctx.users_service
        .get_user_from_session(cookie.value())
        .await
        .map_err(ServerFnError::new)
}

/// Para el inicio de cada server fn protegida: devuelve el usuario logueado,
/// o redirige a `/login` y corta con error.
#[cfg(feature = "ssr")]
pub async fn require_user() -> Result<UserDto, ServerFnError> {
    match get_current_user().await? {
        Some(user) => Ok(user),
        None => {
            leptos_axum::redirect("/login");
            Err(ServerFnError::new("No autenticado"))
        }
    }
}

#[cfg(feature = "ssr")]
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
