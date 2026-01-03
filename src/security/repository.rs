use crate::db::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BannedEntity {
    pub id: String,
    pub entity_type: String, // "IP" or "USER"
    pub value: String,
    pub reason: Option<String>,
    pub banned_at: String,
    pub expires_at: Option<String>,
    pub created_by: Option<String>,
}

pub struct SecurityRepository;

impl SecurityRepository {
    pub async fn ban_entity(
        pool: &DbPool,
        entity_type: &str,
        value: &str,
        reason: Option<&str>,
        duration_hours: Option<i64>,
        created_by: Option<&str>,
    ) -> Result<BannedEntity, anyhow::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = duration_hours.map(|h| (now + chrono::Duration::hours(h)).to_rfc3339());

        let banned = sqlx::query_as::<_, BannedEntity>(
            r#"
            INSERT INTO banned_entities (id, entity_type, value, reason, banned_at, expires_at, created_by)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(entity_type)
        .bind(value)
        .bind(reason)
        .bind(now.to_rfc3339())
        .bind(expires_at)
        .bind(created_by)
        .fetch_one(pool)
        .await?;

        Ok(banned)
    }

    pub async fn is_banned(pool: &DbPool, entity_type: &str, value: &str) -> Result<bool, anyhow::Error> {
        let now = Utc::now().to_rfc3339();
        
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM banned_entities 
            WHERE entity_type = ? AND value = ? 
            AND (expires_at IS NULL OR expires_at > ?)
            "#,
        )
        .bind(entity_type)
        .bind(value)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(count.0 > 0)
    }
}
