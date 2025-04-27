use crate::get_random_email::get_random_email;
use crate::helpers::TestApp;
use auth_service::routes::{ErrorResponse, SignupResponse};

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
            test_case
        );
    }
}

#[tokio::test]
async fn signup_returns_created_status_when_inputs_are_valid() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "test_Passw0rd!",
        "requires2FA": true
    });
    let response = app.post_signup(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        test_case
    );

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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": "pass",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to UserBody")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_request = serde_json::json!({
        "email": random_email,
        "password": "passworD123!",
        "requires2FA": true
    });
    app.post_signup(&signup_request).await;
    let response = app.post_signup(&signup_request).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .error,
        "User already exists".to_owned()
    );
}
