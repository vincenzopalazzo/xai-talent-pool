use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Talent, CreateTalentRequest, UpdateTalentRequest, ApiError};
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
    };
    let inserted = crate::database::create_talent(&pool, &new_talent).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
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