use auth_service::routes::SignupResponse;
use crate::get_random_email::get_random_email;
use crate::helpers::TestApp;

// localhost:3000/signup
#[tokio::test]
async fn signup_returns_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123"
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
                   test_case)
        ;
    }

}

#[tokio::test]
async fn signup_returns_created_status_when_inputs_are_valid() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201, "Failed for input: {:?}", test_case);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}