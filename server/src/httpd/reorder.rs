use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use crate::{
    database,
    models::{ApiError, CreateReorderEventRequest, ReorderEvent, ReorderResponse},
    reorder::derive_pairwise_preferences,
};
use super::server::AppState;

/// Reorder candidates for a job
///
/// This endpoint accepts a reorder event (before/after candidate order) and:
/// 1. Stores the raw reorder event
/// 2. Derives pairwise preferences (C ≻ D) from position changes
/// 3. Persists preferences to the database (idempotent via UNIQUE constraint)
#[api_v2_operation]
#[paperclip::actix::post("/api/v1/reorder", summary = "Reorder candidates and derive preferences")]
pub async fn reorder_candidates(
    data: web::Data<AppState>,
    request: web::Json<CreateReorderEventRequest>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let job_id = &request.job_id;
    let before_order = &request.before_order;
    let after_order = &request.after_order;

    // Validate input
    if before_order.len() != after_order.len() {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "Before and after order lengths must match".to_string(),
            code: 400,
        }));
    }

    if before_order.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "Order arrays cannot be empty".to_string(),
            code: 400,
        }));
    }

    // Fetch job details for job_text
    let job = database::get_job_by_id(&pool, job_id.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(format!("Job not found: {}", job_id))
        })?;

    let job_text = format!("{} at {}", job.title, job.company_name);

    // Fetch talent names for all candidates
    let mut talent_names = HashMap::new();
    for talent_id in before_order.iter().chain(after_order.iter()) {
        if !talent_names.contains_key(talent_id) {
            if let Ok(Some(talent)) = database::get_talent_by_id(&pool, talent_id.clone()).await {
                talent_names.insert(talent_id.clone(), talent.name);
            }
        }
    }

    // Create reorder event
    let event_id = Uuid::new_v4().to_string();
    let event = ReorderEvent {
        id: event_id.clone(),
        job_id: job_id.clone(),
        before_order: serde_json::to_string(&before_order).unwrap(),
        after_order: serde_json::to_string(&after_order).unwrap(),
        moved_talent_id: request.moved_talent_id.clone(),
        event_timestamp: Utc::now().to_rfc3339(),
        created_at: Utc::now().to_rfc3339(),
    };

    // Store reorder event
    database::create_reorder_event(&pool, &event)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Derive pairwise preferences
    let preferences = derive_pairwise_preferences(
        before_order,
        after_order,
        job_id,
        &job_text,
        &talent_names,
        &event_id,
        request.moved_talent_id.as_deref(),
    );

    // Store preferences (INSERT OR IGNORE for idempotency)
    let mut created_count = 0;
    for pref in preferences {
        match database::create_pairwise_preference(&pool, &pref).await {
            Ok(Some(_)) => created_count += 1,
            Ok(None) => {
                // Duplicate preference, ignored
                log::debug!("Duplicate preference ignored: {} ≻ {}", pref.winner_id, pref.loser_id);
            }
            Err(e) => {
                log::error!("Failed to create preference: {}", e);
            }
        }
    }

    Ok(HttpResponse::Ok().json(ReorderResponse {
        event_id,
        preferences_created: created_count,
        message: format!("Reorder event saved with {} preferences created", created_count),
    }))
}

/// Get all pairwise preferences for a job
#[api_v2_operation]
#[paperclip::actix::get("/api/v1/jobs/{job_id}/preferences", summary = "Get pairwise preferences for a job")]
pub async fn get_preferences_for_job(
    data: web::Data<AppState>,
    job_id: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let preferences = database::get_pairwise_preferences_for_job(pool, job_id.to_string())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(preferences))
}

/// Get all reorder events for a job
#[api_v2_operation]
#[paperclip::actix::get("/api/v1/jobs/{job_id}/reorder-events", summary = "Get reorder events for a job")]
pub async fn get_reorder_events_for_job(
    data: web::Data<AppState>,
    job_id: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let pool = &data.db_pool;
    let events = database::get_reorder_events_for_job(pool, job_id.to_string())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(events))
}
