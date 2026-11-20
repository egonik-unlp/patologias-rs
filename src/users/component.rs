use crate::users::{
    dto::UserDto,
    server::{get_current_user, CreateUser, LoginUser, LogoutUser},
};
use leptos::prelude::*;

#[component]
pub fn CreateUser() -> impl IntoView {
    let action: ServerAction<CreateUser> = ServerAction::new();
    let value: MappedSignal<Option<Result<UserDto, ServerFnError>>> = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col px-4 py-2 gap-4 mt-10 max-w-96 mx-auto border border-slate-500 bg-fuchsia-50 rounded-lg shadow">
                <h1 class="text-2xl">"Create user"</h1>
                <label class="gap-1 text-sm flex flex-col ">
                    "Username"
                    <input
                        class="px-2 py-2 border border-slate-400"
                        type="text"
                        name="user[username]"
                        required
                    />
                </label>
                <label class="gap-1 text-sm flex flex-col ">
                    "Full name"
                    <input
                        class="px-2 py-3 border border-slate-400"
                        type="text"
                        name="user[full_name]"
                        required
                    />
                </label>
                <label class="gap-1 text-sm flex flex-col ">
                    "Password"
                    <input
                        class="px-2 py-3 border border-slate-400"
                        type="password"
                        name="password"
                        required
                    />
                </label>
                <label class="gap-1 text-sm flex flex-col ">
                    "Role"
                    <select
                        class="px-2 py-3 border border-slate-400"
                        name="user[role]"
                    >
                        <option value="Operator">"Operador"</option>
                        <option value="Admin">"Admin"</option>
                    </select>
                </label>
                <input type="hidden" name="user[active]" value="true" />
                <button
                    class="rounded-lg px-1 py-1 bg-fuchsia-200 shadow-md hover:bg-fuchsia-400 max-w-32 mx-auto mb-4 "
                    type="submit"
                >
                    "Crear usuario"
                </button>
                <div>
                    {move || match value.get() {
                        None => view! {}.into_any(),
                        Some(Ok(user)) => {
                            view! {
                                <div class="max-w-80 mx-auto bg-green-400">
                                    <h1 class="text-2xl bg-green-200 max-w-40 rounded-xl shadow">
                                        "Login"
                                    </h1>
                                    <p>{format!("Usuario {} creado", user.username)}</p>
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="max-w-80 mx-auto bg-red-400">
                                    <h1 class="text-2xl bg-red-200 max-w-40 rounded-xl shadow">
                                        "Error"
                                    </h1>
                                    <p>{format!("Creando usuario:{} ", e)}</p>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn LoginUser() -> impl IntoView {
    let action: ServerAction<LoginUser> = ServerAction::new();
    let value = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col gap-4 max-w-96 mx-auto p-6 shadow-md border border-slate-400 mt-10 rounded-lg bg-white">
                <h1 class="text-2xl">"Iniciar sesión"</h1>
                <label class="flex flex-col gap-1 text-sm font-semibold text-gray-700">
                    "Username"
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="text"
                        name="user_name"
                        required
                    />
                </label>
                <label class="flex flex-col gap-1 text-sm font-semibold text-gray-700">
                    "Password"
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="password"
                        name="password"
                        required
                    />
                </label>
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
        <Await future=get_current_user() let:data>
            {match data.clone().unwrap() {
                None => view! { <LoginUser /> }.into_any(),
                Some(user) => {
                    view! {
                        <p>{user.username}</p>
                        <Logout />
                    }
                        .into_any()
                }
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
            {move || match value.get() {
                None => "Summer Card SDK".to_string(),
                Some(Ok(val)) => format!("{:?}", val),
                Some(Err(error)) => format!("Error {:?}", error),
            }}
        </p>
    }
}

#[component]
pub fn CurrentUser() -> impl IntoView {
    let resource = OnceResource::new(get_current_user());
    view! {
        <Suspense fallback=|| {
            view! { <p>"…"</p> }
        }>
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
