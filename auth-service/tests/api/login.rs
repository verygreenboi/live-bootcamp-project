use crate::helpers::TestApp;

// localhost:3000/login
#[tokio::test]
async fn login_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.post_login().await;
    assert_eq!(response.status().as_u16(), 200);
}