use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use log::info;

use crate::models::{SocialMediaAnalysis};
use super::server::AppState;

#[api_v2_operation]
#[paperclip::actix::post("/api/v1/talents/{id}/social-analysis", summary = "Update social media analysis for a talent")]
pub async fn update_social_analysis(
    data: web::Data<AppState>,
    path: web::Path<String>,
    json: web::Json<SocialMediaAnalysis>,
) -> ActixResult<HttpResponse> {
    let talent_id = path.into_inner();
    let pool = &data.db_pool;

    // Serialize the social media analysis to JSON
    let social_analysis_json = serde_json::to_string(&json)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let x_handle = json.x_handle.clone();

    // Update the talent with social analysis data
    let updated = crate::database::update_talent_social_analysis(
        pool,
        talent_id.clone(),
        Some(social_analysis_json),
        x_handle,
    ).await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or(actix_web::error::ErrorNotFound("Talent not found"))?;

    info!("Updated social media analysis for talent: {}", talent_id);

    Ok(HttpResponse::Ok().json(updated))
}