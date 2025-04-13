pub mod routes;
mod domain;
pub mod services;
pub mod app_state;

use crate::routes::{
    login_route, logout_route, signup_route, verify_2fa_route, verify_token_route,
};
use axum::{
    routing::{delete, post},
    serve::Serve,
    Router,
};
use std::error::Error;
use tower_http::services::ServeDir;
use crate::app_state::AppState;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/login", post(login_route))
            .route("/logout", delete(logout_route))
            .route("/signup", post(signup_route))
            .route("/verify-token", post(verify_token_route))
            .route("/verify-2fa", post(verify_2fa_route))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
