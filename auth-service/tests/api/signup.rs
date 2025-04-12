use crate::helpers::TestApp;

// localhost:3000/signup
#[tokio::test]
async fn signup_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;
    assert_eq!(response.status().as_u16(), 200);
}