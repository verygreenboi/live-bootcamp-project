use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{Application, services::HashmapUserStore, app_state::AppState};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::new()));
    let app_state = AppState::new(user_store);
    let app = Application::build(app_state, "0.0.0.0:3000").await.expect("failed to build server");
    app.run().await.expect("failed to run server");
}
