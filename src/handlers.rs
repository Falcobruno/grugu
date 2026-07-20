use axum::extract::Path;
use axum::response::IntoResponse;
use sqlx::Row;
use sqlx::SqlitePool;
use axum::extract::State;
use axum::Json;
use crate::models::api_response::ApiResponse;
use crate::models::api_status::ApiStatus;
use crate::models::user::User;
use axum::http::StatusCode;



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
) -> impl IntoResponse {
    if user.name.trim().is_empty(){
        let response = ApiResponse{
            success: false,
            message:"el nombre no puede estar vacío.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    if user.age == 0 {
    let response = ApiResponse {
        success: false,
        message: "La edad debe ser mayor a 0".to_string(),
    };
    return (StatusCode::BAD_REQUEST, Json(response));
}
if user.relationship_years > 100 {
    let response = ApiResponse {
        success: false,
        message: "El valor de años en pareja no es válido".to_string(),
    };
    return (StatusCode::BAD_REQUEST, Json(response));
}
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
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al registrar usuario".to_string(),
            };
           (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn list_users(
    State(pool): State<SqlitePool>
) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT name, age, relationship_years FROM users")
        .fetch_all(&pool)
        .await{
    Ok(rows )  => rows,
    Err(_) =>{
        let response = ApiResponse{
            success : false,
            message : "Error al obtener usuarios".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json (response)).into_response();
    }     
        };
        

    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            name: row.get(0),
            age: row.get::<i32, _>(1) as u8,
            relationship_years: row.get::<i32, _>(2) as u8,
        })
        .collect();

    Json(users).into_response()
}

pub async fn get_user (
    State(pool) : State<SqlitePool>,
    Path (id) : Path<i32>) -> impl IntoResponse{
        let row = sqlx::query("SELECT name, age, relationship_years FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some(row)) => {
            let user = User {
                name:row.get(0),
                age:row.get::<i32,_>(1)as u8,
                relationship_years: row.get::<i32,_>(2)as u8,
            };
            Json(user).into_response()
        }
        Ok(None) =>   {
            let response = ApiResponse{
                success : false,
                message : "Usuario no encontrado".to_string()
            };
            (StatusCode::NOT_FOUND, Json (response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse {

                success: false,
                message : "Error al buscar usuario".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
    }