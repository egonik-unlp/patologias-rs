use leptos::logging::log;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use patologias_rs::entrypoint::http::Application;

    let application = Application::init();
    let address = application.get_address();
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app = application.get_app();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
