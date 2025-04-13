use crate::app_state::AppState;
use crate::domain::{AuthAPIError, User};
use crate::services::UserStoreError;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl SignupRequest {
    pub fn is_valid(&self) -> bool {
        match self {
            user if user.email.contains('@')
                && user.password.len() >= 8
                && !user.email.is_empty() =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn to_user(&self) -> User {
        User::new(self.email.clone(), self.password.clone(), self.requires_2fa)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

pub async fn signup_route(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let mut store = state.user_store.write().await;

    if !request.is_valid() {
        return Ok(AuthAPIError::InvalidCredentials.into_response());
    }

    let user = request.to_user();
    let result = store.add_user(user);

    if result.is_ok() {
        let response = Json(SignupResponse {
            message: "User created successfully!".to_string(),
        });
        Ok((StatusCode::CREATED, response).into_response())
    } else {
        let err = result.err().unwrap();
        match err {
            UserStoreError::UserAlreadyExists => Ok(AuthAPIError::UserAlreadyExists.into_response()),
            _ => Ok(AuthAPIError::UnexpectedError.into_response()),
        }
    }
}
