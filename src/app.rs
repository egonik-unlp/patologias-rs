use crate::entities::component::Entities;
use crate::reports::component::{CreateReport, Report, Reports};
use crate::users::component::{CreateUser, CurrentUser, HandleLogin, LoginUser};

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
            <body>
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
        <Title text="Welcome to Leptos" />
        <nav class="flex p-10 space-x-20">
            <a href="/">"Home"</a>
            <a href="/create_report">"New Report"</a>
            <a href="/reports">"Reports"</a>
            <a href="/entities">"Entities"</a>
            <a href="/create_user">"Create User"</a>
        </nav>
        // content for this welcome page
                <CurrentUser/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/reports") view=Reports />
                    <Route path=path!("/reports/:id") view=Report />
                    <Route path=path!("/create_report") view=CreateReport />
                    <Route path=path!("/entities") view=Entities />
                    <Route path=path!("/create_user") view=CreateUser />
                    <Route path=path!("/login") view=LoginUser />
                </Routes>
            </main>
        </Router>
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
        <HandleLogin/>
    }
}
