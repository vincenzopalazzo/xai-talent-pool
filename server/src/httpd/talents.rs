use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};

use crate::models::{Talent, CreateTalentRequest, UpdateTalentRequest, ApiError, BulkDeleteRequest, BulkDeleteResponse};
use crate::grok_client::GrokClient;
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