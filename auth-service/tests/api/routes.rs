use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// localhost:3000/login
#[tokio::test]
async fn login_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.post_login().await;
    assert_eq!(response.status().as_u16(), 200);
}
// localhost:3000/logout
#[tokio::test]
async fn logout_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.delete_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}
// localhost:3000/signup
#[tokio::test]
async fn signup_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;
    assert_eq!(response.status().as_u16(), 200);
}
// localhost:3000/verify-token