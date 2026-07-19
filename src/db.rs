use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();
    pool
}

pub async fn create_users_table(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            relationship_years INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await
    .unwrap();
}