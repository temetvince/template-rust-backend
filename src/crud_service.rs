use sqlx::sqlite::SqlitePool;
use tokio::sync::OnceCell;
use crate::db;
use crate::models;

pub struct CrudService {
    pool: SqlitePool,
}

type SharedConnection = SqlitePool;
static DATABASE_URL: &str = "sqlite://DB.db";
static DB_POOL: OnceCell<SharedConnection> = OnceCell::const_new();

impl CrudService {
    /// Creates a new instance of `CrudService`.
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(DATABASE_URL)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to apply migrations");

        DB_POOL
            .set(pool.clone())
            .expect("Failed to set DB pool");

        CrudService { pool }
    }

    /// Creates a new user.
    pub async fn create_user(&self, user: &models::User) -> Result<(), sqlx::Error> {
        db::create_user(&self.pool, user).await.map_err(|e| e)
    }

    /// Retrieves a user by ID.
    pub async fn get_user(&self, user_id: i64) -> Result<Option<models::User>, sqlx::Error> {
        db::get_user(&self.pool, user_id).await.map(Some).or_else(|e| {
            if let sqlx::Error::RowNotFound = e {
                Ok(None)
            } else {
                Err(e)
            }
        })
    }

    /// Retrieves all users.
    pub async fn list_users(&self) -> Result<Vec<models::User>, sqlx::Error> {
        db::list_users(&self.pool).await.map_err(|e| e)
    }

    /// Updates a user.
    pub async fn update_user(&self, user: &models::User) -> Result<(), sqlx::Error> {
        db::update_user(&self.pool, user).await.map_err(|e| e)
    }

    /// Deletes a user by ID.
    pub async fn delete_user(&self, user_id: i64) -> Result<(), sqlx::Error> {
        db::delete_user(&self.pool, user_id).await.map_err(|e| e)
    }
}
