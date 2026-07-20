use axum::{routing::{get, post}, Router};
use handlers::{list_users, register, root, get_user, update_user, delete_user};
use db::init_db;

mod models;
mod handlers;
mod db;

#[tokio::main]
async fn main() {
    let _pool = init_db().await;
    db::create_users_table(&_pool).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register))
        .route("/users", get(list_users))
        .route ("/user/{id}", get(get_user))
        .route ("/user/{id}", axum::routing::put(update_user))
        .route ("/user/{id}", axum::routing::delete(delete_user))
        .with_state(_pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Grugu API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

