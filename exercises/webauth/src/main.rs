use axum::{
    Json,
    Router,
    routing::get,
    response::IntoResponse,
    http::{StatusCode, header::HeaderMap},
};
use std::net::SocketAddr;
use base64::{engine::general_purpose, Engine as _};

#[tokio::main]
async fn main() {
    // Spawn Level 1 Server (HTTP - Port 8080)
        let app = Router::new()
            .route("/patient-data", get(level_1_handler));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        println!("🚨 Level 1 (HTTP/Basic) running on http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
}

// ==========================================
// LEVEL 1: HTTP BASIC AUTH (The Sniffing Lab)
// ==========================================
async fn level_1_handler(headers: HeaderMap) -> impl IntoResponse {
    match headers.get("Authorization") {
        Some(auth_header) => {
            let auth_str = auth_header.to_str().unwrap_or("");
            if auth_str.starts_with("Basic ") {
                let code = &auth_str[6..]; // Strip "Basic "
                if let Ok(decoded) = general_purpose::STANDARD.decode(code) {
                    let creds = String::from_utf8(decoded).unwrap_or_default();

                    // VULNERABILITY: We accept any login, but we print it to console!
                    println!("🕵️  [WIRESHARK TARGET] Captured Credentials: {}", creds);

                    return (StatusCode::OK, Json("Access Granted: Patient Data Details..."));
                }
            }
        }
        None => {}
    }

    // Trigger the browser popup
    (
        StatusCode::UNAUTHORIZED,
        axum::Json("[(\"WWW-Authenticate\", \"Basic realm=\"Healthcare System\"\")]")
    )
}

