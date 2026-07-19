use axum::{routing::{get,post},Router,};

use handlers::{root,register};

use db::init_db;





mod models;
mod handlers;
mod db;




#[tokio::main]
async fn main() {
    let _pool= init_db().await;
    db::create_users_table(&_pool).await;


let app = Router::new()
    .route("/", get(root))
    .route("/register", post(register))
    .with_state(_pool);




    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Grugu API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

