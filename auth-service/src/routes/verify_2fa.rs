use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn verify_2fa_route() -> impl IntoResponse {
    StatusCode::OK.into_response()
}