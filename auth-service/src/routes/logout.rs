use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn logout() -> impl IntoResponse {
    println!("processing logout route");
    StatusCode::OK.into_response()
}
