use crate::users::server::CreateUser;
use leptos::prelude::*;

#[component]
pub fn CreateUser() -> impl IntoView {
    let action: ServerAction<CreateUser> = ServerAction::new();
    view! {
       <ActionForm action>
            <label>"Username" <input type="text" name="user[username]" required/></label>
            <label>"Full name" <input type="text" name="user[full_name]" required/></label>
            <label>"Password" <input type="password" name="password" required/></label>
            <label>
                "Role"
                <select name="user[role]" class="border-2 border-b-slate-700">
                    <option value="Operator">"Operador"</option>
                    <option value="Admin">"Admin"</option>
                </select>
            </label>
            <input type="hidden" name="user[active]" value="true"/>
            <button type="submit">"Crear usuario"</button>
        </ActionForm>
    }
}

#[component]
pub fn LoginUser() -> impl IntoView {}
