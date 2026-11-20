use crate::entities::component::Entities;
use crate::reports::component::{CreateReport, Report, Reports};
use crate::users::component::{CreateUser, CurrentUser, HandleLogin, LoginUser, LogoutUser};

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
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-slate-100 text-slate-800 h-dvh">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/patologias-rs.css" />

        // sets the document title
        <Title text="Patologias" />
        <div class="grid grid-cols-1  md:grid-cols-[20rem_1fr] gap-6 min-h-dvh">
            <aside class="flex flex-col sticky top-0 py-10 gap-1 px-6 bg-white border-b md:border-l md:border-b-0 border-slate-200">
                <a href="/">"Home"</a>
                <a href="/create_report">"New Report"</a>
                <a href="/reports">"Reports"</a>
                <a href="/entities">"Entities"</a>
                <a href="/create_user">"Create User"</a>
                <div class="flex-1"></div>
                <div class="grid grid-cols-[2fr_1fr] py-4 border-t ">
                    <CurrentUser />
                    <LogoutUser/>
                </div>
            </aside>
            // content for this welcome page
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HandleLogin />
                        <Route path=path!("/reports") view=Reports />
                        <Route path=path!("/reports/:id") view=Report />
                        <Route path=path!("/create_report") view=CreateReport />
                        <Route path=path!("/entities") view=Entities />
                        <Route path=path!("/create_user") view=CreateUser />
                        <Route path=path!("/login") view=LoginUser />
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
        <!-- <HandleLogin /> -->
    }
}
