use leptos::prelude::*;

use crate::users::dto::NewUserDto;

//TODO: Que la password no viaje por el cable
#[server]
pub async fn create_user(user: NewUserDto, password: String) -> Result<(), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    ctx.users_service
        .create_user(user, password)
        .await
        .map_err(ServerFnError::new)
        .map(|_| ())
}

#[server]
pub async fn login_user(user: NewUserDto, password: String) -> Result<(), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("Getting ctx"))?;
    ctx.users_service
        //TODO: Limpiar acá
        .login_user(user.username, password.as_str())
        .await
        .map_err(ServerFnError::new)
        .map(|_| ())
}
