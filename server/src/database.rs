use sqlx::SqlitePool;
use crate::models::{Talent, UpdateTalentRequest, Job, UpdateJobRequest, Application, HiringRequirement, UpdateHiringRequirementRequest};

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

    let applications_schema = include_str!("../migrations/003_create_applications_table.sql");
    for statement in applications_schema.split(';').filter(|s| !s.trim().is_empty()) {
        sqlx::query(statement).execute(&pool).await?;
    }

    // Add resume fields to talents (ignore error if columns already exist)
    let resume_fields_schema = include_str!("../migrations/004_add_talent_resume_fields.sql");
    for statement in resume_fields_schema.split(';').filter(|s| !s.trim().is_empty()) {
        let _ = sqlx::query(statement).execute(&pool).await;
    }

    // Add collection_id field (ignore error if column already exists)
    let collection_id_schema = include_str!("../migrations/005_add_talent_collection_id.sql");
    for statement in collection_id_schema.split(';').filter(|s| !s.trim().is_empty()) {
        let _ = sqlx::query(statement).execute(&pool).await;
    }

    // Add resume_document_id field (ignore error if column already exists)
    let resume_doc_schema = include_str!("../migrations/006_add_talent_resume_document_id.sql");
    for statement in resume_doc_schema.split(';').filter(|s| !s.trim().is_empty()) {
        let _ = sqlx::query(statement).execute(&pool).await;
    }

    // Create hiring_requirements table
    let hiring_req_schema = include_str!("../migrations/007_create_hiring_requirements_table.sql");
    for statement in hiring_req_schema.split(';').filter(|s| !s.trim().is_empty()) {
        let _ = sqlx::query(statement).execute(&pool).await;
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

pub async fn get_talent_by_email(pool: &Pool, email: String) -> Result<Option<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(include_str!("queries/get_talent_by_email.sql"))
        .bind(&email)
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

// Application database functions

pub async fn create_application(pool: &Pool, application: &Application) -> Result<Application, sqlx::Error> {
    sqlx::query_as::<_, Application>(include_str!("queries/insert_application.sql"))
        .bind(&application.id)
        .bind(&application.talent_id)
        .bind(&application.job_id)
        .bind(&application.resume_data)
        .bind(&application.resume_filename)
        .bind(&application.resume_content_type)
        .bind(&application.cover_letter)
        .bind(&application.status)
        .bind(&application.created_at)
        .fetch_one(pool)
        .await
}

pub async fn get_application_by_id(pool: &Pool, id: String) -> Result<Option<Application>, sqlx::Error> {
    sqlx::query_as::<_, Application>(include_str!("queries/get_application_by_id.sql"))
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn get_applications_by_talent(pool: &Pool, talent_id: String) -> Result<Vec<Application>, sqlx::Error> {
    sqlx::query_as::<_, Application>(include_str!("queries/get_applications_by_talent.sql"))
        .bind(&talent_id)
        .fetch_all(pool)
        .await
}

pub async fn get_applications_by_job(pool: &Pool, job_id: String) -> Result<Vec<Application>, sqlx::Error> {
    sqlx::query_as::<_, Application>(include_str!("queries/get_applications_by_job.sql"))
        .bind(&job_id)
        .fetch_all(pool)
        .await
}

/// Update talent's resume_document_id
pub async fn update_talent_resume_document_id(
    pool: &Pool,
    talent_id: String,
    resume_document_id: Option<String>,
) -> Result<Option<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(
        "UPDATE talents SET resume_document_id = ? WHERE id = ? RETURNING *"
    )
        .bind(&resume_document_id)
        .bind(&talent_id)
        .fetch_optional(pool)
        .await
}

/// Update talent's collection_id
pub async fn update_talent_collection_id(
    pool: &Pool,
    talent_id: String,
    collection_id: String,
) -> Result<Option<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(
        "UPDATE talents SET collection_id = ? WHERE id = ? RETURNING *"
    )
        .bind(&collection_id)
        .bind(&talent_id)
        .fetch_optional(pool)
        .await
}

/// Update talent's resume-extracted fields
pub async fn update_talent_resume_fields(
    pool: &Pool,
    talent_id: String,
    resume_experiences: Option<String>,
    linkedin_url: Option<String>,
    x_url: Option<String>,
    github_url: Option<String>,
    gitlab_url: Option<String>,
) -> Result<Option<Talent>, sqlx::Error> {
    sqlx::query_as::<_, Talent>(include_str!("queries/update_talent_resume_fields.sql"))
        .bind(&resume_experiences)
        .bind(&linkedin_url)
        .bind(&x_url)
        .bind(&github_url)
        .bind(&gitlab_url)
        .bind(&talent_id)
        .fetch_optional(pool)
        .await
}

// Hiring Requirements database functions

pub async fn create_hiring_requirement(pool: &Pool, requirement: &HiringRequirement) -> Result<HiringRequirement, sqlx::Error> {
    sqlx::query_as::<_, HiringRequirement>(
        "INSERT INTO hiring_requirements (id, job_id, title, company_name, requirements_text, created_at)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING *"
    )
        .bind(&requirement.id)
        .bind(&requirement.job_id)
        .bind(&requirement.title)
        .bind(&requirement.company_name)
        .bind(&requirement.requirements_text)
        .bind(&requirement.created_at)
        .fetch_one(pool)
        .await
}

pub async fn get_all_hiring_requirements(pool: &Pool) -> Result<Vec<HiringRequirement>, sqlx::Error> {
    sqlx::query_as::<_, HiringRequirement>(
        "SELECT * FROM hiring_requirements ORDER BY created_at DESC"
    )
        .fetch_all(pool)
        .await
}

pub async fn get_hiring_requirement_by_id(pool: &Pool, id: String) -> Result<Option<HiringRequirement>, sqlx::Error> {
    sqlx::query_as::<_, HiringRequirement>(
        "SELECT * FROM hiring_requirements WHERE id = ?"
    )
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn get_hiring_requirements_by_job(pool: &Pool, job_id: String) -> Result<Vec<HiringRequirement>, sqlx::Error> {
    sqlx::query_as::<_, HiringRequirement>(
        "SELECT * FROM hiring_requirements WHERE job_id = ? ORDER BY created_at DESC"
    )
        .bind(&job_id)
        .fetch_all(pool)
        .await
}

pub async fn update_hiring_requirement(
    pool: &Pool,
    id: String,
    updates: &UpdateHiringRequirementRequest,
) -> Result<Option<HiringRequirement>, sqlx::Error> {
    let title = updates.title.as_ref().map(|s| s as &str).unwrap_or("");
    let company_name = updates.company_name.as_ref().map(|s| s as &str).unwrap_or("");
    let requirements_text = updates.requirements_text.as_ref().map(|s| s as &str).unwrap_or("");

    sqlx::query_as::<_, HiringRequirement>(
        "UPDATE hiring_requirements SET
            job_id = COALESCE(?, job_id),
            title = CASE WHEN ?1 = '' THEN title ELSE ?1 END,
            company_name = CASE WHEN ?2 = '' THEN company_name ELSE ?2 END,
            requirements_text = CASE WHEN ?3 = '' THEN requirements_text ELSE ?3 END
         WHERE id = ?4
         RETURNING *"
    )
        .bind(&updates.job_id)
        .bind(title)
        .bind(company_name)
        .bind(requirements_text)
        .bind(&id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_hiring_requirement(pool: &Pool, id: String) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query("DELETE FROM hiring_requirements WHERE id = ?")
        .bind(&id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows > 0)
}