use crate::users::server::{get_current_user, CreateUser, LoginUser, LogoutUser};
use leptos::prelude::*;

#[component]
pub fn CreateUser() -> impl IntoView {
    let action: ServerAction<CreateUser> = ServerAction::new();
    let value = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col px-4 py-2 justify-center">
                <label>"Username" <input type="text" name="user[username]" required /></label>
                <label>"Full name" <input type="text" name="user[full_name]" required /></label>
                <label>"Password" <input type="password" name="password" required /></label>
                <label>
                    "Role" <select name="user[role]" class="border-2 border-b-slate-700">
                        <option value="Operator">"Operador"</option>
                        <option value="Admin">"Admin"</option>
                    </select>
                </label>
                <input type="hidden" name="user[active]" value="true" />
                <button type="submit">"Crear usuario"</button>
            </div>
        </ActionForm>
        <p>
            {move || match value.get() {
                None => "".to_string(),
                Some(Ok(user)) => format!("Usuario: {:#?} loguineado", user),
                Some(Err(e)) => format!("Error: {e}"),
            }}
        </p>
    }
}

#[component]
pub fn LoginUser() -> impl IntoView {
    let action: ServerAction<LoginUser> = ServerAction::new();
    let value = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col px-4 py-2 justify-center">
                <label>"Username" <input type="text" name="user_name" required /></label>
                <label>"Password" <input type="password" name="password" required /></label>
                <button
                    class="max-w-40 bg-blue-500 hover:bg-blue-600 active:bg-blue-800 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "login"
                </button>
            </div>
        </ActionForm>
        <p>
            {move || match value.get() {
                None => "".to_string(),
                Some(Ok(user)) => format!("Usuario: {:#?} logeado", user),
                Some(Err(e)) => format!("Error: {e:#?}"),
            }}
        </p>
    }
}

#[component]
pub fn HandleLogin() -> impl IntoView {
    view! {
        <Await
        future=get_current_user()
         let:data
        >
    {match data.clone().unwrap() {
    None => view! {<LoginUser/>}.into_any(),
    Some(user) =>view! {<p>{user.username}</p> <Logout/>}.into_any()
    }}
        </Await>
    }
}

#[component]
fn Logout() -> impl IntoView {
    let logout = ServerAction::<LogoutUser>::new();
    let value = logout.value();
    view! {
        <ActionForm action=logout>
            <button type="submit">"Logout"</button>
        </ActionForm>
        <p>
            {
                move || match value.get() {
                    None => "Summer Card SDK".to_string(),
                    Some(Ok(val)) => format!("{:?}", val),
                    Some(Err(error)) => format!("Error {:?}", error)
                }
            }
        </p>
    }
}

#[component]
pub fn CurrentUser() -> impl IntoView {
    let resource = OnceResource::new(get_current_user());
    view! {
        <Suspense fallback=|| view! { <p>"…"</p> }>
            {move || Suspend::new(async move {
                match resource.await {
                    Ok(Some(user)) => view! { <p>{user.full_name}</p> }.into_any(),
                    Ok(None) => view! { <a href="/login">"Login"</a> }.into_any(),
                    Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
                }
            })}
        </Suspense>
    }
}
