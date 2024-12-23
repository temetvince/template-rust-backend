use sqlx::sqlite::SqlitePool;
use sqlx::{query, query_as};
use crate::models::User;

pub async fn create_user(pool: &SqlitePool, user: &User) -> Result<(), sqlx::Error> {
    query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user(pool: &SqlitePool, user_id: i64) -> Result<User, sqlx::Error> {
    let user = query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn update_user(pool: &SqlitePool, user: &User) -> Result<(), sqlx::Error> {
    query("UPDATE users SET name = ?, email = ? WHERE id = ?")
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user(pool: &SqlitePool, user_id: i64) -> Result<(), sqlx::Error> {
    query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
