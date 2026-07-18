use axum::{routing::{get,post},Router,};

use handlers::{root,register};


mod models;
mod handlers;




#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register));




    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Grugu API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

