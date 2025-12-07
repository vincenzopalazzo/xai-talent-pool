use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};

use crate::models::{Talent, CreateTalentRequest, UpdateTalentRequest, ApiError, BulkDeleteRequest, BulkDeleteResponse, TriggerScoringRequest, TriggerScoringResponse};
use crate::grok_client::{GrokClient, CandidateScoringRequest, JobInfoForScoring};
use super::server::AppState;

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/talents", summary = "List all talents")]
async fn get_talents(data: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let talents = crate::database::get_all_talents(pool).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(talents))
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/talents", summary = "Create a new talent")]
async fn create_talent(
    data: web::Data<AppState>,
    json: web::Json<CreateTalentRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let new_talent = Talent {
        id: Uuid::new_v4().to_string(),
        name: json.name.clone(),
        email: json.email.clone(),
        handle: json.handle.clone(),
        avatar: Some(format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", json.name.replace(" ", "").to_lowercase())),
        title: json.title.clone(),
        location: json.location.clone(),
        experience: json.experience.clone(),
        skills: json.skills.clone(),
        bio: json.bio.clone(),
        verified: json.verified as i32,
        created_at: Utc::now().to_rfc3339(),
        // Resume-extracted fields (populated later by Grok service)
        resume_experiences: None,
        linkedin_url: None,
        x_url: None,
        github_url: None,
        gitlab_url: None,
        collection_id: None,
        resume_document_id: None,
        social_analysis: None,
        x_handle_discovered: None,
        candidate_score: None,
        candidate_score_details: None,
    };
    let inserted = crate::database::create_talent(&pool, &new_talent).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Create a collection for this talent asynchronously
    let grok_url = data.grok_service_url.clone();
    let pool_clone = pool.clone();
    let talent_id = inserted.id.clone();
    let talent_name = inserted.name.clone();

    tokio::spawn(async move {
        info!("======================================================================");
        info!("COLLECTION CREATION: Starting for talent {}", talent_id);
        info!("======================================================================");

        let client = GrokClient::new(&grok_url);

        match client.create_collection(&talent_id, &talent_name).await {
            Ok(response) => {
                if response.success {
                    if let Some(collection) = response.collection {
                        info!("======================================================================");
                        info!("COLLECTION CREATION: Success!");
                        info!("Talent ID: {}", talent_id);
                        info!("Collection ID: {}", collection.collection_id);
                        info!("Collection Name: {}", collection.collection_name);
                        info!("======================================================================");

                        // Update talent with collection_id
                        match crate::database::update_talent_collection_id(
                            &pool_clone,
                            talent_id.clone(),
                            collection.collection_id.clone(),
                        ).await {
                            Ok(_) => {
                                info!("SUCCESS: Updated talent {} with collection_id {}", talent_id, collection.collection_id);
                            },
                            Err(e) => {
                                error!("FAILED to update talent {} with collection_id: {}", talent_id, e);
                            }
                        }
                    }
                } else {
                    error!("COLLECTION CREATION: Failed - {:?}", response.error);
                }
            }
            Err(e) => {
                error!("======================================================================");
                error!("COLLECTION CREATION: Error - {}", e);
                error!("======================================================================");
            }
        }
    });

    Ok(HttpResponse::Created().json(inserted))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/talents/{id}", summary = "Get a specific talent")]
async fn get_talent(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let talent = crate::database::get_talent_by_id(pool, id).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Talent not found"))?;
    Ok(HttpResponse::Ok().json(talent))
}

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/talents/email/{email}", summary = "Get talent by email")]
pub async fn get_talent_by_email(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let email = path.into_inner();
    let pool = &data.db_pool;
    match crate::database::get_talent_by_email(pool, email).await {
        Ok(Some(talent)) => Ok(HttpResponse::Ok().json(talent)),
        Ok(None) => Ok(HttpResponse::NotFound().json(crate::models::ApiError {
            message: "Talent not found".to_string(),
            code: 404,
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[api_v2_operation]
#[paperclip::actix::put("/api/v1/talents/{id}", summary = "Update a talent")]
async fn update_talent(
    data: web::Data<AppState>,
    path: web::Path<String>,
    json: web::Json<UpdateTalentRequest>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let updated = crate::database::update_talent(pool, id, &json).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Talent not found"))?;
    Ok(HttpResponse::Ok().json(updated))
}

#[api_v2_operation]
#[paperclip::actix::delete("/api/v1/talents/{id}", summary = "Delete a talent")]
async fn delete_talent(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let pool = &data.db_pool;
    let deleted = crate::database::delete_talent(&pool, id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if deleted {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(ApiError {
            message: "Talent not found".to_string(),
            code: 404,
        }))
    }
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/talents/bulk-delete", summary = "Delete multiple talents")]
pub async fn delete_talents_bulk(
    data: web::Data<AppState>,
    json: web::Json<BulkDeleteRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let ids = &json.ids;

    if ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "No talent IDs provided".to_string(),
            code: 400,
        }));
    }

    info!("Bulk deleting {} talents", ids.len());

    let deleted_count = crate::database::delete_talents_bulk(pool, ids).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(BulkDeleteResponse {
        deleted_count,
        total_requested: ids.len(),
    }))
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/talents/{id}/score", summary = "Trigger candidate scoring for a talent against a job")]
pub async fn trigger_scoring(
    data: web::Data<AppState>,
    path: web::Path<String>,
    json: web::Json<TriggerScoringRequest>,
) -> ActixResult<HttpResponse> {
    let talent_id = path.into_inner();
    let pool = &data.db_pool;

    info!("======================================================================");
    info!("TRIGGER SCORING: Request received");
    info!("Talent ID: {}", talent_id);
    info!("Job ID: {}", json.job_id);
    info!("======================================================================");

    // Get talent
    let talent = match crate::database::get_talent_by_id(pool, talent_id.clone()).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(TriggerScoringResponse {
                success: false,
                message: "Talent not found".to_string(),
                score: None,
                recommendation: None,
            }));
        }
        Err(e) => {
            error!("Failed to get talent: {}", e);
            return Ok(HttpResponse::InternalServerError().json(TriggerScoringResponse {
                success: false,
                message: format!("Database error: {}", e),
                score: None,
                recommendation: None,
            }));
        }
    };

    // Check if talent has a collection
    let collection_id = match &talent.collection_id {
        Some(id) => id.clone(),
        None => {
            return Ok(HttpResponse::BadRequest().json(TriggerScoringResponse {
                success: false,
                message: "Talent does not have a collection. Please upload a resume first.".to_string(),
                score: None,
                recommendation: None,
            }));
        }
    };

    // Get job
    let job = match crate::database::get_job_by_id(pool, json.job_id.clone()).await {
        Ok(Some(j)) => j,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(TriggerScoringResponse {
                success: false,
                message: "Job not found".to_string(),
                score: None,
                recommendation: None,
            }));
        }
        Err(e) => {
            error!("Failed to get job: {}", e);
            return Ok(HttpResponse::InternalServerError().json(TriggerScoringResponse {
                success: false,
                message: format!("Database error: {}", e),
                score: None,
                recommendation: None,
            }));
        }
    };

    // Call Grok scoring service
    let client = GrokClient::new(&data.grok_service_url);

    let scoring_request = CandidateScoringRequest {
        talent_id: talent.id.clone(),
        collection_id,
        job: JobInfoForScoring {
            id: job.id.clone(),
            title: job.title.clone(),
            description: job.description.clone(),
            company_name: job.company_name.clone(),
            skills_required: job.skills_required.clone(),
            experience_level: job.experience_level.clone(),
            location: job.location.clone(),
            location_type: job.location_type.clone(),
        },
        candidate_name: talent.name.clone(),
        candidate_title: talent.title.clone(),
        candidate_skills: talent.skills.clone(),
    };

    match client.score_candidate(&scoring_request).await {
        Ok(response) => {
            if response.success {
                if let Some(result) = response.result {
                    info!("TRIGGER SCORING: Success!");
                    info!("Score: {}", result.overall_score);
                    info!("Recommendation: {}", result.recommendation);

                    // Store the scoring result
                    let scoring_json = serde_json::to_string(&result).ok();

                    if let Err(e) = crate::database::update_talent_candidate_score(
                        pool,
                        talent.id.clone(),
                        result.overall_score,
                        scoring_json,
                    ).await {
                        error!("Failed to update talent score: {}", e);
                    }

                    return Ok(HttpResponse::Ok().json(TriggerScoringResponse {
                        success: true,
                        message: "Scoring completed successfully".to_string(),
                        score: Some(result.overall_score),
                        recommendation: Some(result.recommendation),
                    }));
                }
            }

            Ok(HttpResponse::InternalServerError().json(TriggerScoringResponse {
                success: false,
                message: response.error.unwrap_or_else(|| "Scoring failed".to_string()),
                score: None,
                recommendation: None,
            }))
        }
        Err(e) => {
            error!("TRIGGER SCORING: Error - {}", e);
            Ok(HttpResponse::InternalServerError().json(TriggerScoringResponse {
                success: false,
                message: format!("Scoring service error: {}", e),
                score: None,
                recommendation: None,
            }))
        }
    }
}