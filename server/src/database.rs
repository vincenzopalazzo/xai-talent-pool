use sqlx::SqlitePool;
use crate::models::{Talent, UpdateTalentRequest};

pub type Pool = SqlitePool;

pub async fn init_pool(database_url: &str) -> Result<Pool, sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;

    let schema_sql = include_str!("../migrations/001_create_talents_table.sql");
    sqlx::query(schema_sql).execute(&pool).await?;

    Ok(pool)
}

pub async fn create_talent(pool: &Pool, talent: &Talent) -> Result<Talent, sqlx::Error> {
    sqlx::query_as::<_, Talent>(include_str!("queries/insert_talent.sql"))
        .bind(&talent.id)
        .bind(&talent.name)
        .bind(&talent.email)
        .bind(&talent.handle)
        .bind(&talent.avatar)
        .bind(&talent.title)
        .bind(&talent.location)
        .bind(&talent.experience)
        .bind(sqlx::types::Json(&talent.skills))
        .bind(&talent.bio)
        .bind(talent.verified as i32)
        .bind(&talent.created_at)
        .fetch_one(pool)
        .await
}

pub async fn get_all_talents(pool: &Pool) -> Result<Vec<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(include_str!("queries/get_all_talents.sql"))
    .fetch_all(pool)
    .await
}

pub async fn get_talent_by_id(pool: &Pool, id: String) -> Result<Option<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(include_str!("queries/get_talent_by_id.sql"))
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn update_talent(pool: &Pool, id: String, updates: &UpdateTalentRequest) -> Result<Option<Talent>, sqlx::Error> {
    let query_str = include_str!("queries/update_talent.sql");
    let name = updates.name.as_ref().map(|s| s as &str).unwrap_or("");
    let email = updates.email.as_ref().map(|s| s as &str).unwrap_or("");
    let handle = updates.handle.as_ref().map(|s| s as &str).unwrap_or("");
    let avatar = &updates.avatar;
    let title = updates.title.as_ref().map(|s| s as &str).unwrap_or("");
    let location = &updates.location;
    let experience = updates.experience.as_ref().map(|s| s as &str).unwrap_or("");
    let skills_json = updates.skills.as_ref().map(|s| sqlx::types::Json(s));
    let bio = &updates.bio;
    let verified = updates.verified.unwrap_or(false) as i32;
    sqlx::query_as::<_, Talent>(query_str)
        .bind(name)
        .bind(email)
        .bind(handle)
        .bind(avatar)
        .bind(title)
        .bind(location)
        .bind(experience)
        .bind(skills_json)
        .bind(bio)
        .bind(verified)
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_talent(pool: &Pool, id: String) -> Result<bool, sqlx::Error> {
    let query_str = include_str!("queries/delete_talent.sql");
    let rows = sqlx::query(query_str)
        .bind(&id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows > 0)
}