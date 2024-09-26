use axum::{response::IntoResponse, routing::{get, post}, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login() -> impl IntoResponse {}
async fn logout() -> impl IntoResponse {}
