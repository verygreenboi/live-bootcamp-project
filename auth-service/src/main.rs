use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000").await.expect("failed to build server");
    app.run().await.expect("failed to run server");
}
