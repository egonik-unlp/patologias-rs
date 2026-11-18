use crate::users::server::{CreateUser, LoginUser};
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
                Some(Ok(user)) => format!("Usuario: {:#?} creado", user),
                Some(Err(e)) => format!("Error: {e:#?}"),
            }}
        </p>
    }
}
