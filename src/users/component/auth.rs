//! Session UI: login form, logout button, current-user badge.
use crate::frontend::icons::IconLogOut;
use crate::users::{
    dto::UserDto,
    server::{get_current_user, LoginUser, LogoutUser},
};
use leptos::prelude::*;

#[component]
pub fn LoginUser() -> impl IntoView {
    let action: ServerAction<LoginUser> = expect_context();
    let value: MappedSignal<Option<Result<UserDto, _>>> = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col gap-4 max-w-96 mx-auto px-4 py-2 shadow border border-slate-500 rounded-lg bg-white">
                <h1 class="text-2xl font-semibold">"Iniciar sesión"</h1>
                <label class="flex flex-col gap-1">
                    <span class="text-sm font-semibold text-slate-700">"Username"</span>
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="text"
                        name="user_name"
                        required
                    />
                </label>
                <label class="flex flex-col gap-1  ">
                    <span class="text-sm font-semibold text-slate-700">"Password"</span>
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="password"
                        name="password"
                        required
                    />
                </label>
                <button
                    class="max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "login"
                </button>
                {move || match value.get() {
                    None => view! {}.into_any(),
                    Some(Ok(user)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-green-50 border-green-200 text-green-700">
                                {format!("Sesión iniciada como {}", user.username)}
                            </p>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class="rounded-md border px-3 py-2 text-sm bg-red-50 border-red-200 text-red-700">
                                {format!("Error: {}", e)}
                            </p>
                        }
                            .into_any()
                    }
                }}
            </div>
        </ActionForm>
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
                        <LogoutUser />
                    }
                        .into_any()
                }
            }}
        </Await>
    }
}

#[component]
pub fn LogoutUser() -> impl IntoView {
    let action: ServerAction<LogoutUser> = expect_context();
    let value: MappedSignal<Option<Result<(), _>>> = action.value();
    view! {
        <div>
            <ActionForm action>
                <button
                    class="inline-flex items-center gap-1.5 px-2 py-1 rounded font-semibold border border-slate-300 bg-white text-slate-700 hover:bg-slate-50"
                    type="submit"
                >
                    <IconLogOut class="size-4" />
                    "Logout"
                </button>
            </ActionForm>
            <p>
                {move || match value.get() {
                    Some(Err(err)) => {
                        view! {
                            <p class="rounded-md border px-3 py-2 text-sm bg-red-50 border-red-200 text-red-700">
                                {format!("Error {}", err)}
                            </p>
                        }
                            .into_any()
                    }
                    _ => view! {}.into_any(),
                }}
            </p>
        </div>
    }
}

#[component]
pub fn CurrentUser() -> impl IntoView {
    let login: ServerAction<LoginUser> = expect_context();
    let logout: ServerAction<LogoutUser> = expect_context();
    let resource: Resource<Result<Option<UserDto>, _>> = Resource::new(
        move || (login.version().get(), logout.version().get()),
        |_| get_current_user(),
    );
    view! {
        <Suspense fallback=|| {
            view! { <p>"…"</p> }
        }>
            {move || Suspend::new(async move {
                match resource.await {
                    Ok(Some(user)) => {
                        view! {
                            <div class="flex items-center justify-between">
                                <div class="flex flex-col">
                                    <p class="font-semibold text-sm">{user.username}</p>
                                    <p class="text-xs">{user.full_name}</p>
                                </div>
                                <LogoutUser />
                            </div>
                        }
                            .into_any()
                    }
                    Ok(None) => view! { <p>"Not logged in"</p> }.into_any(),
                    Err(e) => view! { <p>{format!("Error: {e}")}</p> }.into_any(),
                }
            })}
        </Suspense>
    }
}
