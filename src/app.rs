use crate::entities::component::Entities;
use crate::frontend::icons::{
    BrandMark, IconFilePlus, IconFiles, IconHouse, IconSpecimenJar, IconUserPlus,
};
use crate::frontend::notfound::NotFound;
use crate::reports::component::{CreateReport, Report, Reports, UpdateReport};
use crate::reports::server::{DeleteStudy, UpdateStudy};
use crate::users::component::{CreateUser, CurrentUser, HandleLogin, LoginUser, UserList};
use crate::users::server::{LoginUser, LogoutUser};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::path;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="/favicon.ico" sizes="32x32" />
                <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
                <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
                <link rel="manifest" href="/site.webmanifest" />
                <meta name="theme-color" content="#4B2E83" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-slate-100 text-slate-800">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // TODO: Si estos son los unicos, se puede dejar asi, si no capaz convenga en pensar en alguna
    // resolución un poquito más elegante
    let action: ServerAction<LoginUser> = ServerAction::new();
    let logout: ServerAction<LogoutUser> = ServerAction::new();
    let delete_action: ServerAction<DeleteStudy> = ServerAction::new();
    let update_action: ServerAction<UpdateStudy> = ServerAction::new();
    provide_context(action);
    provide_context(logout);
    provide_context(delete_action);
    provide_context(update_action);
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/patologias-rs.css" />

        // sets the document title
        <Title text="Patologias" />
        <div class="grid grid-cols-1 md:grid-cols-[20rem_1fr] gap-6 min-h-dvh">
            <aside class="flex flex-col md:sticky md:top-0 md:h-dvh py-10 gap-1 px-6 bg-white border-b md:border-r md:border-b-0 border-slate-200">
                <div class="flex items-center gap-2 mb-6 text-lg font-semibold">
                    <BrandMark class="size-8" />
                    "Patologías"
                </div>
                <a href="/" class="flex items-center gap-2">
                    <IconHouse class="size-5" />
                    "Home"
                </a>
                <a href="/create_report" class="flex items-center gap-2">
                    <IconFilePlus class="size-5" />
                    "New Report"
                </a>
                <a href="/reports" class="flex items-center gap-2">
                    <IconFiles class="size-5" />
                    "Reports"
                </a>
                <a href="/entities" class="flex items-center gap-2">
                    <IconSpecimenJar class="size-5" />
                    "Entities"
                </a>
                <a href="/create_user" class="flex items-center gap-2">
                    <IconUserPlus class="size-5" />
                    "Create User"
                </a>
                <a href="/users" class="flex items-center gap-2">
                    <IconFiles class="size-5" />
                    "Users"
                </a>
                <div class="flex-1"></div>
                <div class="py-4 border-t border-slate-200">

                    <CurrentUser />
                </div>
            </aside>
            // content for this welcome page
            <Router>
                <main class="mt-10">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=StaticSegment("") view=HandleLogin />
                        <Route path=path!("/reports") view=Reports />
                        <Route path=path!("/reports/:id") view=Report />
                        <Route path=path!("/reports/:id/edit") view=UpdateReport />
                        <Route path=path!("/create_report") view=CreateReport />
                        <Route path=path!("/entities") view=Entities />
                        <Route path=path!("/create_user") view=CreateUser />
                        <Route path=path!("/login") view=LoginUser />
                        <Route path=path!("/users") view=UserList />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
