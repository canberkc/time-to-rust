pub async fn root() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Rust AXUM".to_string())
}


pub async fn healthcheck() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Hi I am great, don't worry :)".to_string())
}