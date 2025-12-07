use sqlx::SqlitePool;
use crate::models::{Talent, UpdateTalentRequest, Job, UpdateJobRequest};

pub type Pool = SqlitePool;

pub async fn init_pool(database_url: &str) -> Result<Pool, sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;

    // Run migrations
    let talents_schema = include_str!("../migrations/001_create_talents_table.sql");
    sqlx::query(talents_schema).execute(&pool).await?;

    let jobs_schema = include_str!("../migrations/002_create_jobs_table.sql");
    // Execute each statement separately for SQLite
    for statement in jobs_schema.split(';').filter(|s| !s.trim().is_empty()) {
        sqlx::query(statement).execute(&pool).await?;
    }

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

// Job database functions

pub async fn create_job(pool: &Pool, job: &Job) -> Result<Job, sqlx::Error> {
    sqlx::query_as::<_, Job>(include_str!("queries/insert_job.sql"))
        .bind(&job.id)
        .bind(&job.title)
        .bind(&job.description)
        .bind(&job.company_name)
        .bind(&job.company_logo)
        .bind(&job.location)
        .bind(&job.location_type)
        .bind(&job.employment_type)
        .bind(&job.salary_min)
        .bind(&job.salary_max)
        .bind(&job.salary_currency)
        .bind(&job.skills_required)
        .bind(&job.experience_level)
        .bind(&job.status)
        .bind(&job.created_at)
        .bind(&job.expires_at)
        .fetch_one(pool)
        .await
}

pub async fn get_all_jobs(pool: &Pool) -> Result<Vec<Job>, sqlx::Error> {
    sqlx::query_as::<_, Job>(include_str!("queries/get_all_jobs.sql"))
        .fetch_all(pool)
        .await
}

pub async fn get_job_by_id(pool: &Pool, id: String) -> Result<Option<Job>, sqlx::Error> {
    sqlx::query_as::<_, Job>(include_str!("queries/get_job_by_id.sql"))
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn update_job(pool: &Pool, id: String, updates: &UpdateJobRequest) -> Result<Option<Job>, sqlx::Error> {
    let title = updates.title.as_ref().map(|s| s as &str).unwrap_or("");
    let description = updates.description.as_ref().map(|s| s as &str).unwrap_or("");
    let company_name = updates.company_name.as_ref().map(|s| s as &str).unwrap_or("");
    let location_type = updates.location_type.as_ref().map(|s| s as &str).unwrap_or("");
    let employment_type = updates.employment_type.as_ref().map(|s| s as &str).unwrap_or("");
    let skills_required = updates.skills_required.as_ref().map(|s| s as &str).unwrap_or("");
    let experience_level = updates.experience_level.as_ref().map(|s| s as &str).unwrap_or("");
    let status = updates.status.as_ref().map(|s| s as &str).unwrap_or("");

    sqlx::query_as::<_, Job>(include_str!("queries/update_job.sql"))
        .bind(title)
        .bind(description)
        .bind(company_name)
        .bind(&updates.company_logo)
        .bind(&updates.location)
        .bind(location_type)
        .bind(employment_type)
        .bind(&updates.salary_min)
        .bind(&updates.salary_max)
        .bind(&updates.salary_currency)
        .bind(skills_required)
        .bind(experience_level)
        .bind(status)
        .bind(&updates.expires_at)
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_job(pool: &Pool, id: String) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query(include_str!("queries/delete_job.sql"))
        .bind(&id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows > 0)
}