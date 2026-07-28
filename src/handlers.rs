use axum::routing::get;
use bcrypt::hash;
use axum::extract::Path;
use axum::response::IntoResponse;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use axum::extract::State;
use axum::Json;
use crate::models::api_response::ApiResponse;
use crate::models::api_status::ApiStatus;
use crate::models::user::PublicUser;
use crate::models::user::User;
use axum::http::StatusCode;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::claims::Claims;
use crate::models::user::LoginRequest;
use chrono::Utc;

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

    let hashed_password = hash(&user.password, bcrypt::DEFAULT_COST).unwrap();

    let result = sqlx::query(
        "INSERT INTO users (name, age, relationship_years, password) VALUES (?, ?, ?, ?)"
    )
    .bind(&user.name)
    .bind(user.age)
    .bind(user.relationship_years)
    .bind(&hashed_password)
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
    let rows = match sqlx::query("SELECT name, age, relationship_years FROM users WHERE active = 1")
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


    let users: Vec<PublicUser> = rows
        .iter()
        .map(|row| PublicUser {
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
        let row = sqlx::query("SELECT name, age, relationship_years FROM users WHERE id = ? AND active = 1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some(row)) => {
            let user = PublicUser {
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


pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(user): Json<User>
) -> impl IntoResponse {
    if user.name.trim().is_empty() {
        let response = ApiResponse {
            success: false,
            message: "El nombre no puede estar vacío".to_string(),
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
        "UPDATE users SET name = ?, age = ?, relationship_years = ? WHERE id = ?"
    )
    .bind(&user.name)
    .bind(user.age)
    .bind(user.relationship_years)
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => {
            let response = ApiResponse {
                success: false,
                message: "Usuario no encontrado".to_string(),
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Usuario actualizado exitosamente".to_string(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al actualizar usuario".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn delete_user (
    State(pool): State<SqlitePool>,
    Path(id) : Path<i32>
) -> impl IntoResponse{
    let result = sqlx::query("UPDATE users SET active = 0 WHERE id = ?")
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() ==  0 => {
            let response = ApiResponse{
                success: false,
                message :"Usuario no encontrado".to_string(),
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
        Ok(_) => {
            let response = ApiResponse{
                success : true,
                message: "Usuario eliminado exitosamente".to_string(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = ApiResponse{
                success : false,
                message : "Error al eliminar usuario".to_string(),
            };
             (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn login (
    State(pool): State<SqlitePool>,
    Json (login_req) : Json <LoginRequest>,
)-> impl IntoResponse{
    let row = sqlx::query("SELECT id, password FROM users WHERE name = ? AND active = 1")
    .bind(&login_req.name)
    .fetch_optional(&pool)
    .await;

    let row = match row{
        Ok(Some(row)) => row,
        Ok(None) => {
            let response = ApiResponse{
                success : false,
                message : "Usuario o contraseña no válida".to_string(),
            };
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }
        Err(_) => {
            let response = ApiResponse {
                success:false,
                message:"Error al buscar usuario".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };
    let user_id :i32 =row.get(0);
    let stored_hash: String = row.get(1);
    let password_matches = verify(&login_req.password, &stored_hash).unwrap_or(false);

    if !password_matches {
        let response = ApiResponse {
            success: false,
            message: "Usuario o contraseña incorrectos".to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
    }

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secreto_super_seguro".as_ref())
    ).unwrap();

    Json(serde_json::json!({ "token": token })).into_response()
}


