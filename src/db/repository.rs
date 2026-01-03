use crate::models::{User, Connection, Script, QueryExecution};
use crate::db::DbPool;
use chrono::Utc;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &DbPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, anyhow::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(pool: &DbPool, username: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }
}

pub struct ConnectionRepository;

impl ConnectionRepository {
    pub async fn create(
        pool: &DbPool,
        user_id: &str,
        name: &str,
        db_type: &str,
        host: &str,
        port: i32,
        username: &str,
        encrypted_password: &str,
        database_name: Option<&str>,
    ) -> Result<Connection, anyhow::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let conn = sqlx::query_as::<_, Connection>(
            r#"
            INSERT INTO connections 
            (id, user_id, name, db_type, host, port, username, encrypted_password, database_name, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'disconnected', ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(name)
        .bind(db_type)
        .bind(host)
        .bind(port)
        .bind(username)
        .bind(encrypted_password)
        .bind(database_name)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .fetch_one(pool)
        .await?;

        Ok(conn)
    }

    pub async fn find_by_user(pool: &DbPool, user_id: &str) -> Result<Vec<Connection>, anyhow::Error> {
        let connections = sqlx::query_as::<_, Connection>(
            "SELECT * FROM connections WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(connections)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str, user_id: &str) -> Result<Option<Connection>, anyhow::Error> {
        let conn = sqlx::query_as::<_, Connection>(
            "SELECT * FROM connections WHERE id = ? AND user_id = ?"
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        Ok(conn)
    }

    pub async fn delete(pool: &DbPool, id: &str, user_id: &str) -> Result<bool, anyhow::Error> {
        let result = sqlx::query("DELETE FROM connections WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

pub struct ScriptRepository;

impl ScriptRepository {
    pub async fn create(
        pool: &DbPool,
        user_id: &str,
        name: &str,
        query: &str,
        db_type: &str,
    ) -> Result<Script, anyhow::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let script = sqlx::query_as::<_, Script>(
            r#"
            INSERT INTO scripts (id, user_id, name, query, db_type, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(name)
        .bind(query)
        .bind(db_type)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .fetch_one(pool)
        .await?;

        Ok(script)
    }

    pub async fn find_by_user(pool: &DbPool, user_id: &str) -> Result<Vec<Script>, anyhow::Error> {
        let scripts = sqlx::query_as::<_, Script>(
            "SELECT * FROM scripts WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(scripts)
    }

    pub async fn delete(pool: &DbPool, id: &str, user_id: &str) -> Result<bool, anyhow::Error> {
        let result = sqlx::query("DELETE FROM scripts WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
