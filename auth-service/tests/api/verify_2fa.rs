use crate::helpers::TestApp;

// localhost:3000/verify-2fa
#[tokio::test]
async fn verify_2fa_returns_ok_status() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
}