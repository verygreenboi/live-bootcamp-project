use crate::routes::{
    login::login_route,
    logout::logout_route,
    signup::signup_route,
    verify_token::verify_token_route,
};
use axum::{
    routing::{post, delete},
    serve::Serve,
    Router
};
use std::error::Error;
use tower_http::services::ServeDir;

mod routes;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/login", post(login_route))
            .route("/logout", delete(logout_route))
            .route("/signup", post(signup_route))
            .route("/verify-token", post(verify_token_route));

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
