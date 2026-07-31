use sqlx::SqlitePool;
use std::fs;

pub async fn init_db() -> SqlitePool {
    let data_dir = dirs::data_local_dir()
        .expect("No se pudo obtener la carpeta de datos del usuario");
    let db_dir = data_dir.join("grugu");
    fs::create_dir_all(&db_dir).expect("No se pudo crear la carpeta grugu");
    
    let db_path = db_dir.join("grugu.db");
    let url = format!("sqlite:{}?mode=rwc", db_path.to_str().unwrap().replace('\\', "/"));
    
    SqlitePool::connect(&url)
        .await
        .expect("No se pudo conectar a la base de datos")
}

pub async fn create_users_table(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    relationship_years INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT 1,
    password NOT NULL,
    partner_id INTEGER
)"
    )
    .execute(pool)
    .await
    .expect("No se pudo crear la tabla users");
}

pub async fn create_mood_entries_table(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mood_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            ai_response_to_user TEXT,
            ai_suggestion_to_partner TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await
    .expect("No se pudo crear la tabla mood_entries");
}