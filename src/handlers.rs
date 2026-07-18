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

pub async fn register (Json(_user) : Json<User>) ->Json<ApiResponse> {
    let responsen = ApiResponse{
        success:true,
        message:"Bienvenido a Grugu".to_string(),
    };
    Json(responsen)
}