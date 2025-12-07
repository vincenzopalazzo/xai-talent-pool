use actix_web::{web, HttpResponse, Result as ActixResult};
use log::{info, error};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Job, CreateJobRequest, UpdateJobRequest, ApiError, JobMatch, JobMatchWithTalent};
use crate::grok_client::{GrokClient, JobMatchingRequest, TalentForMatching};
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

#[api_v2_operation]
#[paperclip::actix::get("/api/v1/jobs/{id}/matches", summary = "Get top candidate matches for a job")]
pub async fn get_job_matches(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let job_id = path.into_inner();
    let pool = &data.db_pool;

    // Verify job exists
    let _job = crate::database::get_job_by_id(pool, job_id.clone()).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Job not found"))?;

    // Get matches
    let matches = crate::database::get_job_matches(pool, &job_id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Enrich with talent data
    let mut matches_with_talent: Vec<JobMatchWithTalent> = Vec::new();
    for m in matches {
        let talent = crate::database::get_talent_by_id(pool, m.talent_id.clone()).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        matches_with_talent.push(JobMatchWithTalent {
            id: m.id,
            job_id: m.job_id,
            talent_id: m.talent_id,
            score: m.score,
            rank: m.rank,
            match_reasons: m.match_reasons
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            concerns: m.concerns
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            summary: m.summary.unwrap_or_default(),
            created_at: m.created_at,
            talent,
        });
    }

    Ok(HttpResponse::Ok().json(matches_with_talent))
}

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/jobs/{id}/matches/generate", summary = "Generate candidate matches for a job using AI")]
pub async fn generate_job_matches(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let job_id = path.into_inner();
    let pool = &data.db_pool;

    info!("[generate_job_matches] Starting for job: {}", job_id);

    // Get the job
    let job = crate::database::get_job_by_id(pool, job_id.clone()).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("Job not found"))?;

    info!("[generate_job_matches] Job found: {} - {}", job.id, job.title);

    // Get all talents with collections
    let talents = crate::database::get_talents_with_collections(pool).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    info!("[generate_job_matches] Found {} talents with collections", talents.len());

    if talents.is_empty() {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "No candidates with profiles to match",
            "matches": []
        })));
    }

    // Build candidates list for Grok
    let candidates: Vec<TalentForMatching> = talents
        .iter()
        .filter_map(|t| {
            t.collection_id.as_ref().map(|cid| TalentForMatching {
                id: t.id.clone(),
                name: t.name.clone(),
                title: t.title.clone(),
                skills: t.skills.clone(),
                experience: t.experience.clone(),
                collection_id: cid.clone(),
            })
        })
        .collect();

    info!("[generate_job_matches] Prepared {} candidates for matching", candidates.len());

    // Call Grok service
    let grok_url = std::env::var("GROK_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8001".to_string());
    let grok_client = GrokClient::new(&grok_url);

    let request = JobMatchingRequest {
        job_id: job.id.clone(),
        job_title: job.title.clone(),
        job_description: job.description.clone(),
        company_name: job.company_name.clone(),
        skills_required: job.skills_required.clone(),
        experience_level: job.experience_level.clone(),
        candidates,
        top_n: 10,
    };

    info!("[generate_job_matches] Calling Grok service...");

    let response = grok_client.match_candidates_to_job(&request).await
        .map_err(|e| {
            error!("[generate_job_matches] Grok service error: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Matching failed: {}", e))
        })?;

    if !response.success {
        error!("[generate_job_matches] Grok returned error: {:?}", response.error);
        return Err(actix_web::error::ErrorInternalServerError(
            response.error.unwrap_or_else(|| "Unknown error".to_string())
        ));
    }

    let result = response.result.ok_or_else(|| {
        actix_web::error::ErrorInternalServerError("No result from matching service")
    })?;

    info!("[generate_job_matches] Got {} matches from Grok", result.matches.len());

    // Delete existing matches for this job
    crate::database::delete_job_matches_by_job_id(pool, &job_id).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Store new matches
    let mut stored_matches: Vec<JobMatchWithTalent> = Vec::new();
    for m in result.matches {
        let job_match = JobMatch {
            id: Uuid::new_v4().to_string(),
            job_id: job_id.clone(),
            talent_id: m.talent_id.clone(),
            score: m.score,
            rank: m.rank,
            match_reasons: Some(serde_json::to_string(&m.match_reasons).unwrap_or_default()),
            concerns: Some(serde_json::to_string(&m.concerns).unwrap_or_default()),
            summary: Some(m.summary.clone()),
            created_at: Utc::now().to_rfc3339(),
        };

        let stored = crate::database::create_job_match(pool, &job_match).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        // Get talent for response
        let talent = crate::database::get_talent_by_id(pool, m.talent_id.clone()).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        stored_matches.push(JobMatchWithTalent {
            id: stored.id,
            job_id: stored.job_id,
            talent_id: stored.talent_id,
            score: stored.score,
            rank: stored.rank,
            match_reasons: m.match_reasons,
            concerns: m.concerns,
            summary: m.summary,
            created_at: stored.created_at,
            talent,
        });
    }

    info!("[generate_job_matches] Stored {} matches in database", stored_matches.len());

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Generated {} matches", stored_matches.len()),
        "matches": stored_matches
    })))
}
