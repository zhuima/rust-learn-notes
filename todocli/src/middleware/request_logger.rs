use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn log_request(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let res = next.run(req).await;

    let duration = start.elapsed();
    let status = res.status();

    tracing::info!(
        "Request: {} {} - Status: {} - Duration: {:?} - IP: {}",
        method,
        uri,
        status,
        duration,
        ip
    );

    Ok(res)
}
