use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};

use crate::models::{Application, CreateApplicationRequest, ApplicationResponse, ApiError};
use crate::grok_client::{GrokClient, TalentInfo};
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

    // If there's a resume, analyze it with Grok service
    if let (Some(resume_data), Some(talent)) = (&inserted.resume_data, talent) {
        // Decode base64 resume
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        if let Ok(pdf_bytes) = STANDARD.decode(resume_data) {
            let talent_info = TalentInfo {
                id: talent.id.clone(),
                name: talent.name.clone(),
                email: talent.email.clone(),
                handle: talent.handle.clone(),
                skills: talent.skills.clone(),
                title: talent.title.clone(),
                location: talent.location.clone(),
                experience: talent.experience.clone(),
                bio: talent.bio.clone(),
            };

            let filename = inserted.resume_filename
                .clone()
                .unwrap_or_else(|| "resume.pdf".to_string());

            // Call Grok service asynchronously (don't block response)
            let grok_url = data.grok_service_url.clone();
            let pool_clone = pool.clone();
            let talent_id = talent.id.clone();

            tokio::spawn(async move {
                info!("======================================================================");
                info!("GROK ANALYSIS STARTING");
                info!("======================================================================");
                info!("Talent ID: {}", talent_id);
                info!("Grok service URL: {}", grok_url);
                info!("Resume filename: {}", filename);
                info!("PDF size: {} bytes", pdf_bytes.len());

                let client = GrokClient::new(&grok_url);

                match client.analyze_resume(&talent_info, &pdf_bytes, &filename).await {
                    Ok(response) => {
                        info!("======================================================================");
                        info!("GROK RESPONSE RECEIVED - Success: {}", response.success);
                        if let Some(ref err) = response.error {
                            error!("Grok error: {}", err);
                        }

                        if response.success {
                            if let Some(result) = response.result {
                                info!("======================================================================");
                                info!("EXTRACTED INFORMATION FROM RESUME");
                                info!("======================================================================");
                                info!("Talent ID: {}", result.talent_id);
                                info!("WORK EXPERIENCES ({} found):", result.experiences.len());
                                for (i, exp) in result.experiences.iter().enumerate() {
                                    info!("  {}. {} at {}", i + 1, exp.role, exp.company);
                                    if let Some(ref duration) = exp.duration {
                                        info!("     Duration: {}", duration);
                                    }
                                    info!("     Summary: {}", exp.summary);
                                }
                                info!("SOCIAL PROFILE URLS:");
                                info!("  LinkedIn: {}", result.urls.linkedin.as_deref().unwrap_or("Not found"));
                                info!("  X/Twitter: {}", result.urls.x.as_deref().unwrap_or("Not found"));
                                info!("  GitHub: {}", result.urls.github.as_deref().unwrap_or("Not found"));
                                info!("  GitLab: {}", result.urls.gitlab.as_deref().unwrap_or("Not found"));
                                info!("======================================================================");

                                // Serialize experiences to JSON
                                let experiences_json = serde_json::to_string(&result.experiences)
                                    .ok();

                                // Update talent with extracted info
                                match crate::database::update_talent_resume_fields(
                                    &pool_clone,
                                    talent_id.clone(),
                                    experiences_json,
                                    result.urls.linkedin,
                                    result.urls.x,
                                    result.urls.github,
                                    result.urls.gitlab,
                                ).await {
                                    Ok(_updated) => {
                                        info!("SUCCESS: Updated talent {} with resume data", talent_id);
                                    },
                                    Err(e) => {
                                        error!("FAILED to update talent {}: {}", talent_id, e);
                                    }
                                }
                            } else {
                                info!("No result in response (result is None)");
                            }
                        } else {
                            error!("Grok analysis failed: {:?}", response.error);
                        }
                    }
                    Err(e) => {
                        error!("======================================================================");
                        error!("GROK SERVICE ERROR: {}", e);
                        error!("======================================================================");
                    }
                }
            });
        }
    }

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
