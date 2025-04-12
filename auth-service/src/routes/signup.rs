use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn signup_route() -> impl IntoResponse {
    StatusCode::OK.into_response()
}