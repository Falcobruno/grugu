use axum::{routing::{get, post}, Router};
use axum::middleware;
use db::init_db;
use handlers::{list_users, register, root, 
get_user, update_user, delete_user, login, auth_middleware, link_partner};

mod models;
mod handlers;
mod db;

#[tokio::main]
async fn main() {
    let _pool = init_db().await;
    db::create_users_table(&_pool).await;

    let protected_routes = Router::new()
        .route("/users", get(list_users))
        .route("/user/{id}", get(get_user))
        .route("/user/{id}", axum::routing::put(update_user))
        .route("/user/{id}", axum::routing::delete(delete_user))
        .route ("/link/{partener_id}", post (link_partner))
        .route_layer(middleware::from_fn(auth_middleware));

    let public_routes = Router::new()
        .route("/", get(root))
        .route("/register", post(register))
        .route("/login", post(login));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(_pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Grugu API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}