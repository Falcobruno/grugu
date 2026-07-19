use sqlx::SqlitePool;
use axum::extract::State;
use axum::Json;
use crate::models::api_response::ApiResponse;
use crate::models::api_status::ApiStatus;
use crate::models::user::User;

pub async fn root() -> Json<ApiStatus> {
    let status = ApiStatus {
        name: "Grugu API".to_string(),
        status: "Running".to_string(),
        version: "0.1.0".to_string(),
    };
    Json(status)
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(user): Json<User>
) -> Json<ApiResponse> {
    let result = sqlx::query(
        "INSERT INTO users (name, age, relationship_years) VALUES (?, ?, ?)"
    )
    .bind(&user.name)
    .bind(user.age)
    .bind(user.relationship_years)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Usuario registrado exitosamente".to_string(),
            };
            Json(response)
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al registrar usuario".to_string(),
            };
            Json(response)
        }
    }
}