use crate::helpers::TestApp;

// localhost:3000/logout
#[tokio::test]
async fn logout_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.delete_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}