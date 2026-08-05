use bcrypt::hash;
use axum::extract::Path;
use axum::response::IntoResponse;
use sqlx::Row;
use sqlx::SqlitePool;
use axum::extract::State;
use axum::Json;
use crate::models::api_response::ApiResponse;
use crate::models::api_status::ApiStatus;
use crate::models::mood::MoodRequest;
use crate::models::user::PublicUser;
use crate::models::user::User;
use axum::http::StatusCode;
use bcrypt::verify;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use crate::models::claims::Claims;
use crate::models::user::LoginRequest;
use chrono::Utc;
use axum::{middleware::Next, extract::Request};
use axum::extract::Extension;
use tracing::info;

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
            info!("Usuario registrado: {}", user.name);
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
    Extension(claims): Extension<Claims>,
    Json(user): Json<User>
) -> impl IntoResponse {
    if claims.sub != id{
        let response = ApiResponse {
            success:false,
            message: "No tenés permiso para modificar este usuario".to_string(),
        };

        return  (StatusCode::FORBIDDEN, Json(response));

    }
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
    Path(id) : Path<i32>,
    Extension(claims): Extension<Claims>
) -> impl IntoResponse{
    if claims.sub != id{
        let response = ApiResponse{
            success:false,
            message :"No tenés permiso para eliminar este usuario".to_string(),
        };
         return (StatusCode::FORBIDDEN, Json(response));
    }
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
    let user_id: i32 = row.get(0);
    let stored_hash: String = row.get(1);
    let password_matches = verify(&login_req.password, &stored_hash).unwrap_or(false);
    info!("Login exitoso: {}", login_req.name);

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

pub async fn auth_middleware (
    mut req: Request,
    next: Next
) -> Result<impl IntoResponse, impl IntoResponse> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => h.trim_start_matches("Bearer ").to_string(),
        _ => {
            let response = ApiResponse {
                success: false,
                message: "Token no provisto".to_string(),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(response)));
        }
    };

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secreto_super_seguro".as_ref()),
        &Validation::default()
    );

    let claims = match claims {
        Ok(data) => data.claims,
        Err(_) => {
            info!("Intento de acceso con token inválido");
            let response = ApiResponse {
                success: false,
                message: "Token inválido o expirado".to_string(),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(response)));
        }
    };

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

pub async fn link_partner(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(partner_id): Path<i32>
) -> impl IntoResponse {
    let my_id = claims.sub;

    if my_id == partner_id {
        let response = ApiResponse {
            success: false,
            message: "No podés vincularte con vos mismo".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let my_row = sqlx::query("SELECT partner_id FROM users WHERE id = ? AND active = 1")
        .bind(my_id)
        .fetch_optional(&pool)
        .await;

    let my_partner_id: Option<i32> = match my_row {
        Ok(Some(row)) => row.get(0),
        Ok(None) => {
            let response = ApiResponse {
                success: false,
                message: "Usuario no encontrado".to_string(),
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al buscar usuario".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    if my_partner_id.is_some() {
        let response = ApiResponse {
            success: false,
            message: "Ya tenés una pareja vinculada".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let partner_row = sqlx::query("SELECT partner_id FROM users WHERE id = ? AND active = 1")
        .bind(partner_id)
        .fetch_optional(&pool)
        .await;

    let partner_partner_id: Option<i32> = match partner_row {
        Ok(Some(row)) => row.get(0),
        Ok(None) => {
            let response = ApiResponse {
                success: false,
                message: "El usuario a vincular no existe".to_string(),
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al buscar el usuario a vincular".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    if partner_partner_id.is_some() {
        let response = ApiResponse {
            success: false,
            message: "Ese usuario ya tiene una pareja vinculada".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let result1 = sqlx::query("UPDATE users SET partner_id = ? WHERE id = ?")
        .bind(partner_id)
        .bind(my_id)
        .execute(&pool)
        .await;

    let result2 = sqlx::query("UPDATE users SET partner_id = ? WHERE id = ?")
        .bind(my_id)
        .bind(partner_id)
        .execute(&pool)
        .await;

    if result1.is_err() || result2.is_err() {
        let response = ApiResponse {
            success: false,
            message: "Error al vincular usuarios".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }

    let response = ApiResponse {
        success: true,
        message: "Usuarios vinculados exitosamente".to_string(),
    };
    (StatusCode::OK, Json(response))
}

pub async fn add_mood(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(mood): Json<MoodRequest>
) -> impl IntoResponse{
    let ai_response_to_user = "Una respuesta de la IA ".to_string();
    let ai_suggestion_to_partner = "Sugerencia generada por IA".to_string();

    let result = sqlx::query(
        "INSERT INTO mood_entries (user_id, text, ai_response_to_user, ai_suggestion_to_partner) VALUES (?, ?, ?, ?)"
    )

    .bind(claims.sub)
    .bind(&mood.text)
    .bind(&ai_response_to_user)
    .bind(&ai_suggestion_to_partner)
    .execute(&pool)
    .await;

     match result {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: ai_response_to_user,
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Error al guardar el registro".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
