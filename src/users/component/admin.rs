//! Admin-only user management: create users, list users.
use crate::users::{
    dto::UserDto,
    server::{get_all_users, CreateUser},
};
use leptos::prelude::*;

#[component]
pub fn CreateUser() -> impl IntoView {
    let action: ServerAction<CreateUser> = ServerAction::new();
    let value: MappedSignal<Option<Result<UserDto, ServerFnError>>> = action.value();
    view! {
        <ActionForm action>
            <div class="flex flex-col px-4 py-2 gap-4 max-w-96 mx-auto border border-slate-500 bg-white rounded-lg shadow">
                <h1 class="text-2xl font-semibold">"Create user"</h1>
                <label class="gap-1 flex flex-col ">
                    <span class="text-sm font-semibold text-slate-700">"Username"</span>
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="text"
                        name="user[username]"
                        required
                    />
                </label>
                <label class="gap-1 flex flex-col ">
                    <span class="text-sm font-semibold text-slate-700">"Full name"</span>
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="text"
                        name="user[full_name]"
                        required
                    />
                </label>
                <label class="gap-1 flex flex-col ">
                    <span class="text-sm font-semibold text-slate-700">"Password"</span>
                    <input
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        type="password"
                        name="password"
                        required
                    />
                </label>
                <label class="gap-1 flex flex-col ">
                    <span class="text-sm font-semibold text-slate-700">"Role"</span>

                    <select
                        class="border border-slate-400 px-3 py-2 rounded-md w-full"
                        name="user[role]"
                    >
                        <option value="Operator">"Operador"</option>
                        <option value="Admin">"Admin"</option>
                    </select>
                </label>
                <input type="hidden" name="user[active]" value="true" />
                <button
                    class="max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "Crear usuario"
                </button>
                {move || match value.get() {
                    None => view! {}.into_any(),
                    Some(Ok(user)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-green-50 border-green-200 text-green-700">
                                {format!("Usuario {} creado", user.username)}
                            </p>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-red-50 border-red-200 text-red-700">
                                {format!("Error creando usuario. {} ", e)}
                            </p>
                        }
                            .into_any()
                    }
                }}
            </div>
        </ActionForm>
    }
}

//TODO: Finish component. Ojo que () satisface impl IntoView
#[component]
pub fn UserList() -> impl IntoView {
    #[allow(unused_variables)]
    let resource = OnceResource::new(get_all_users());
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="error">
                    <p>"Not a number! Errors: "</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect::<Vec<_>>()
                        }}
                    </ul>
                </div>
            }
        }>

            <Suspense fallback=|| view! { <p>Loading...</p> }>
                <table>
                    <thead>
                        <tr>
                            <th>"Username"</th>
                            <th>"Full Name"</th>
                            <th>"Role"</th>
                        </tr>
                    </thead>
                    <tbody>

                        {move || Suspend::new(async move {
                            let resource = resource.await;
                            resource
                                .map(|rowdata: Vec<UserDto>| {
                                    rowdata
                                        .into_iter()
                                        .map(|row| {

                                            view! {
                                                <tr>
                                                    <td>{row.username}</td>
                                                    <td>{row.full_name}</td>
                                                    <td>{format!("{:?}", row.role)}</td>
                                                </tr>
                                            }
                                        })
                                        .collect_view()
                                })
                        })}
                    </tbody>
                </table>
            </Suspense>
        </ErrorBoundary>
        <div class="flex flex-col gap-2 px-4 py-2">
        </div>
    }
}
