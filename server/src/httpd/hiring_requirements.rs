use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use log::info;

use crate::models::{HiringRequirement, CreateHiringRequirementRequest, UpdateHiringRequirementRequest, ApiError};
use super::server::AppState;

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/hiring-requirements", summary = "List all hiring requirements")]
pub async fn get_hiring_requirements(
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let requirements = crate::database::get_all_hiring_requirements(pool).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(requirements))
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/hiring-requirements", summary = "Create a new hiring requirement")]
pub async fn create_hiring_requirement(
    data: web::Data<AppState>,
    json: web::Json<CreateHiringRequirementRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;

    // Verify job exists if job_id is provided
    if let Some(ref job_id) = json.job_id {
        let job = crate::database::get_job_by_id(pool, job_id.clone()).await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        if job.is_none() {
            return Ok(HttpResponse::BadRequest().json(ApiError {
                message: "Job not found".to_string(),
                code: 400,
            }));
        }
    }

    let new_requirement = HiringRequirement {
        id: Uuid::new_v4().to_string(),
        job_id: json.job_id.clone(),
        title: json.title.clone(),
        company_name: json.company_name.clone(),
        requirements_text: json.requirements_text.clone(),
        created_at: Utc::now().to_rfc3339(),
    };

    info!("Creating hiring requirement: {} for {}", new_requirement.title, new_requirement.company_name);

    let inserted = crate::database::create_hiring_requirement(pool, &new_requirement).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(inserted))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/hiring-requirements/{id}", summary = "Get a hiring requirement by ID")]
pub async fn get_hiring_requirement(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;

    match crate::database::get_hiring_requirement_by_id(pool, id).await {
        Ok(Some(requirement)) => Ok(HttpResponse::Ok().json(requirement)),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiError {
            message: "Hiring requirement not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/hiring-requirements/job/{job_id}", summary = "Get hiring requirements for a job")]
pub async fn get_hiring_requirements_by_job(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let job_id = path.into_inner();
    let pool = &data.db_pool;

    let requirements = crate::database::get_hiring_requirements_by_job(pool, job_id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(requirements))
}

#[api_v2_operation]
#[paperclip::actix::put("/api/v1/hiring-requirements/{id}", summary = "Update a hiring requirement")]
pub async fn update_hiring_requirement(
    data: web::Data<AppState>,
    path: web::Path<String>,
    json: web::Json<UpdateHiringRequirementRequest>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;

    // Verify job exists if job_id is provided
    if let Some(ref job_id) = json.job_id {
        let job = crate::database::get_job_by_id(pool, job_id.clone()).await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        if job.is_none() {
            return Ok(HttpResponse::BadRequest().json(ApiError {
                message: "Job not found".to_string(),
                code: 400,
            }));
        }
    }

    match crate::database::update_hiring_requirement(pool, id, &json).await {
        Ok(Some(requirement)) => Ok(HttpResponse::Ok().json(requirement)),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiError {
            message: "Hiring requirement not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[api_v2_operation]
#[paperclip::actix::delete("/api/v1/hiring-requirements/{id}", summary = "Delete a hiring requirement")]
pub async fn delete_hiring_requirement(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;

    match crate::database::delete_hiring_requirement(pool, id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiError {
            message: "Hiring requirement not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
