use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::User;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup_route(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> impl IntoResponse {
    let mut store = state.user_store.write().await;
    let user = User::new(request.email, request.password, request.requires_2fa);

    store.add_user(user).expect("Failed to add user");

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}