use std::net::SocketAddr;

use crate::{
    app::*,
    core::{config::model::Config, database::connection::get_connection_pool},
    entrypoint::application_state::AppState,
};
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub struct Application {
    app: Router<()>,
    addr: SocketAddr,
}

impl Application {
    pub fn init() -> Self {
        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let config = Config::from_env();
        println!("{:#?}", config.clone());
        let connection = get_connection_pool(config.database_url);
        let state = AppState::new(connection, leptos_options);

        let app = Router::new()
            .leptos_routes_with_context(
                &state,
                routes,
                {
                    let state = state.clone();
                    move || provide_context(state.clone())
                },
                {
                    let leptos_options = state.clone().leptos_options.clone();
                    move || shell(leptos_options.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
            .with_state(state);
        Application { app, addr }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn get_app(self) -> Router<()> {
        self.app
    }
}
