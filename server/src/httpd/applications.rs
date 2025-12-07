use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Application, CreateApplicationRequest, ApplicationResponse, ApiError};
use super::server::AppState;

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/applications", summary = "Create a new job application")]
pub async fn create_application(
    data: web::Data<AppState>,
    json: web::Json<CreateApplicationRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;

    // Verify talent exists
    let talent = crate::database::get_talent_by_id(pool, json.talent_id.clone()).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if talent.is_none() {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "Talent not found".to_string(),
            code: 400,
        }));
    }

    // Verify job exists
    let job = crate::database::get_job_by_id(pool, json.job_id.clone()).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if job.is_none() {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "Job not found".to_string(),
            code: 400,
        }));
    }

    let new_application = Application {
        id: Uuid::new_v4().to_string(),
        talent_id: json.talent_id.clone(),
        job_id: json.job_id.clone(),
        resume_data: json.resume_data.clone(),
        resume_filename: json.resume_filename.clone(),
        resume_content_type: json.resume_content_type.clone(),
        cover_letter: json.cover_letter.clone(),
        status: "pending".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };

    let inserted = crate::database::create_application(pool, &new_application).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return response without the full resume data
    let response = ApplicationResponse {
        id: inserted.id,
        talent_id: inserted.talent_id,
        job_id: inserted.job_id,
        has_resume: inserted.resume_data.is_some(),
        resume_filename: inserted.resume_filename,
        cover_letter: inserted.cover_letter,
        status: inserted.status,
        created_at: inserted.created_at,
    };

    Ok(HttpResponse::Created().json(response))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/applications/{id}", summary = "Get an application by ID")]
pub async fn get_application(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;

    match crate::database::get_application_by_id(pool, id).await {
        Ok(Some(application)) => {
            let response = ApplicationResponse {
                id: application.id,
                talent_id: application.talent_id,
                job_id: application.job_id,
                has_resume: application.resume_data.is_some(),
                resume_filename: application.resume_filename,
                cover_letter: application.cover_letter,
                status: application.status,
                created_at: application.created_at,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiError {
            message: "Application not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/applications/talent/{talent_id}", summary = "Get applications by talent")]
pub async fn get_applications_by_talent(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let talent_id = path.into_inner();
    let pool = &data.db_pool;

    let applications = crate::database::get_applications_by_talent(pool, talent_id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let responses: Vec<ApplicationResponse> = applications
        .into_iter()
        .map(|app| ApplicationResponse {
            id: app.id,
            talent_id: app.talent_id,
            job_id: app.job_id,
            has_resume: app.resume_data.is_some(),
            resume_filename: app.resume_filename,
            cover_letter: app.cover_letter,
            status: app.status,
            created_at: app.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(responses))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/applications/job/{job_id}", summary = "Get applications for a job")]
pub async fn get_applications_by_job(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let job_id = path.into_inner();
    let pool = &data.db_pool;

    let applications = crate::database::get_applications_by_job(pool, job_id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let responses: Vec<ApplicationResponse> = applications
        .into_iter()
        .map(|app| ApplicationResponse {
            id: app.id,
            talent_id: app.talent_id,
            job_id: app.job_id,
            has_resume: app.resume_data.is_some(),
            resume_filename: app.resume_filename,
            cover_letter: app.cover_letter,
            status: app.status,
            created_at: app.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(responses))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/applications/{id}/resume", summary = "Download resume for an application")]
pub async fn get_application_resume(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;

    match crate::database::get_application_by_id(pool, id).await {
        Ok(Some(application)) => {
            if let Some(resume_data) = application.resume_data {
                let content_type = application.resume_content_type
                    .unwrap_or_else(|| "application/octet-stream".to_string());
                let filename = application.resume_filename
                    .unwrap_or_else(|| "resume".to_string());

                // Decode base64
                use base64::{Engine as _, engine::general_purpose::STANDARD};
                match STANDARD.decode(&resume_data) {
                    Ok(bytes) => {
                        Ok(HttpResponse::Ok()
                            .content_type(content_type)
                            .insert_header(("Content-Disposition", format!("inline; filename=\"{}\"", filename)))
                            .body(bytes))
                    }
                    Err(_) => Ok(HttpResponse::InternalServerError().json(ApiError {
                        message: "Failed to decode resume".to_string(),
                        code: 500,
                    })),
                }
            } else {
                Ok(HttpResponse::NotFound().json(ApiError {
                    message: "No resume attached to this application".to_string(),
                    code: 404,
                }))
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiError {
            message: "Application not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
