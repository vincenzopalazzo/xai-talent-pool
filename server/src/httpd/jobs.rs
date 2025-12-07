use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Job, CreateJobRequest, UpdateJobRequest, ApiError};
use super::server::AppState;

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/jobs", summary = "List all jobs")]
pub async fn get_jobs(data: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let jobs = crate::database::get_all_jobs(pool).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(jobs))
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/jobs", summary = "Create a new job posting")]
pub async fn create_job(
    data: web::Data<AppState>,
    json: web::Json<CreateJobRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let new_job = Job {
        id: Uuid::new_v4().to_string(),
        title: json.title.clone(),
        description: json.description.clone(),
        company_name: json.company_name.clone(),
        company_logo: json.company_logo.clone(),
        location: json.location.clone(),
        location_type: json.location_type.clone(),
        employment_type: json.employment_type.clone(),
        salary_min: json.salary_min,
        salary_max: json.salary_max,
        salary_currency: json.salary_currency.clone(),
        skills_required: json.skills_required.clone(),
        experience_level: json.experience_level.clone(),
        status: "active".to_string(),
        created_at: Utc::now().to_rfc3339(),
        expires_at: json.expires_at.clone(),
    };
    let inserted = crate::database::create_job(pool, &new_job).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(inserted))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/jobs/{id}", summary = "Get a specific job")]
pub async fn get_job(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let job = crate::database::get_job_by_id(pool, id).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Job not found"))?;
    Ok(HttpResponse::Ok().json(job))
}

#[api_v2_operation]
#[paperclip::actix::put("/api/v1/jobs/{id}", summary = "Update a job posting")]
pub async fn update_job(
    data: web::Data<AppState>,
    path: web::Path<String>,
    json: web::Json<UpdateJobRequest>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let updated = crate::database::update_job(pool, id, &json).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Job not found"))?;
    Ok(HttpResponse::Ok().json(updated))
}

#[api_v2_operation]
#[paperclip::actix::delete("/api/v1/jobs/{id}", summary = "Delete a job posting")]
pub async fn delete_job(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let deleted = crate::database::delete_job(pool, id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if deleted {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(ApiError {
            message: "Job not found".to_string(),
            code: 404,
        }))
    }
}
