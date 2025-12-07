use actix_web::{web, HttpResponse, Result as ActixResult};
use paperclip::actix::api_v2_operation;
use uuid::Uuid;
use chrono::Utc;
use log::{info, error, warn};

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
            let talent_name = talent.name.clone();
            let collection_id = talent.collection_id.clone();
            let old_resume_document_id = talent.resume_document_id.clone();
            let pdf_bytes_for_upload = pdf_bytes.clone();
            let filename_for_upload = filename.clone();

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

                                // Clone URLs before passing to update (so we can use them for research)
                                let linkedin_url = result.urls.linkedin.clone();
                                let x_url = result.urls.x.clone();
                                let github_url = result.urls.github.clone();

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

                                        // Start parallel social media research based on found URLs
                                        info!("======================================================================");
                                        info!("SOCIAL RESEARCH: Starting parallel research for found URLs");
                                        info!("======================================================================");

                                        // Build X/Twitter URL from handle if not already present
                                        // The talent's "handle" field is their X username
                                        let x_profile_url: Option<String> = if x_url.is_some() {
                                            x_url.clone()
                                        } else if !talent_info.handle.is_empty() {
                                            // Construct X URL from handle (remove @ if present)
                                            let handle = talent_info.handle.trim_start_matches('@');
                                            Some(format!("https://x.com/{}", handle))
                                        } else {
                                            None
                                        };

                                        // Check if there are any URLs to research
                                        let has_urls = github_url.is_some() || linkedin_url.is_some() || x_profile_url.is_some();

                                        // Spawn social research as a SEPARATE task with guaranteed status update
                                        // This ensures status is always updated even if outer task fails
                                        if has_urls {
                                            let pool_for_social = pool_clone.clone();
                                            let talent_id_for_social = talent_id.clone();
                                            let grok_url_for_social = grok_url.clone();
                                            let collection_id_for_social = collection_id.clone();
                                            let talent_for_social = talent.clone();
                                            let talent_info_for_social = talent_info.clone();
                                            let github_url_for_social = github_url.clone();
                                            let linkedin_url_for_social = linkedin_url.clone();
                                            let x_profile_url_for_social = x_profile_url.clone();

                                            // Spawn a completely separate task for social research
                                            tokio::spawn(async move {
                                                // Set status to in_progress at the START of this task
                                                info!("[SOCIAL] Starting social research task for {}", talent_id_for_social);
                                                let _ = crate::database::update_talent_social_research_status(
                                                    &pool_for_social,
                                                    talent_id_for_social.clone(),
                                                    "in_progress",
                                                ).await;

                                                // Use catch_unwind to ensure we ALWAYS update status
                                                let research_future = async {
                                                    let mut research_handles = Vec::new();

                                                    // GitHub research
                                                    if let Some(ref gh_url) = github_url_for_social {
                                                        let client = GrokClient::new(&grok_url_for_social);
                                                        let name = talent_info_for_social.name.clone();
                                                        let email = Some(talent_info_for_social.email.clone());
                                                        let profile_url = Some(gh_url.clone());
                                                        let coll_id = collection_id_for_social.clone();
                                                        let old_doc_id = talent_for_social.github_report_id.clone();
                                                        let pool_for_research = pool_for_social.clone();
                                                        let tid_for_research = talent_id_for_social.clone();

                                                        let handle = tokio::spawn(async move {
                                                            info!("[SOCIAL] Starting GitHub research for {}", name);
                                                            match client.research_platform(
                                                                "github",
                                                                &name,
                                                                email.as_deref(),
                                                                profile_url.as_deref(),
                                                                coll_id.as_deref(),
                                                                old_doc_id.as_deref(),
                                                            ).await {
                                                                Ok(resp) if resp.success => {
                                                                    info!("[SOCIAL] GitHub research completed successfully");
                                                                    let tldr = resp.report.as_ref().and_then(|r| r.tldr.clone());
                                                                    let _ = crate::database::update_talent_platform_research(
                                                                        &pool_for_research,
                                                                        tid_for_research,
                                                                        "github",
                                                                        resp.document_id.clone(),
                                                                        tldr,
                                                                    ).await;
                                                                    ("github".to_string(), resp.document_id)
                                                                },
                                                                Ok(resp) => {
                                                                    error!("[SOCIAL] GitHub research API failed: {:?}", resp.error);
                                                                    ("github".to_string(), None)
                                                                },
                                                                Err(e) => {
                                                                    error!("[SOCIAL] GitHub research error (timeout?): {}", e);
                                                                    ("github".to_string(), None)
                                                                }
                                                            }
                                                        });
                                                        research_handles.push(handle);
                                                    }

                                                    // LinkedIn research
                                                    if let Some(ref li_url) = linkedin_url_for_social {
                                                        let client = GrokClient::new(&grok_url_for_social);
                                                        let name = talent_info_for_social.name.clone();
                                                        let email = Some(talent_info_for_social.email.clone());
                                                        let profile_url = Some(li_url.clone());
                                                        let coll_id = collection_id_for_social.clone();
                                                        let old_doc_id = talent_for_social.linkedin_report_id.clone();
                                                        let pool_for_research = pool_for_social.clone();
                                                        let tid_for_research = talent_id_for_social.clone();

                                                        let handle = tokio::spawn(async move {
                                                            info!("[SOCIAL] Starting LinkedIn research for {}", name);
                                                            match client.research_platform(
                                                                "linkedin",
                                                                &name,
                                                                email.as_deref(),
                                                                profile_url.as_deref(),
                                                                coll_id.as_deref(),
                                                                old_doc_id.as_deref(),
                                                            ).await {
                                                                Ok(resp) if resp.success => {
                                                                    info!("[SOCIAL] LinkedIn research completed successfully");
                                                                    let tldr = resp.report.as_ref().and_then(|r| r.tldr.clone());
                                                                    let _ = crate::database::update_talent_platform_research(
                                                                        &pool_for_research,
                                                                        tid_for_research,
                                                                        "linkedin",
                                                                        resp.document_id.clone(),
                                                                        tldr,
                                                                    ).await;
                                                                    ("linkedin".to_string(), resp.document_id)
                                                                },
                                                                Ok(resp) => {
                                                                    error!("[SOCIAL] LinkedIn research API failed: {:?}", resp.error);
                                                                    ("linkedin".to_string(), None)
                                                                },
                                                                Err(e) => {
                                                                    error!("[SOCIAL] LinkedIn research error (timeout?): {}", e);
                                                                    ("linkedin".to_string(), None)
                                                                }
                                                            }
                                                        });
                                                        research_handles.push(handle);
                                                    }

                                                    // Twitter/X research
                                                    if let Some(ref tw_url) = x_profile_url_for_social {
                                                        let client = GrokClient::new(&grok_url_for_social);
                                                        let name = talent_info_for_social.name.clone();
                                                        let email = Some(talent_info_for_social.email.clone());
                                                        let profile_url = Some(tw_url.clone());
                                                        info!("[SOCIAL] Twitter/X profile URL: {}", tw_url);
                                                        let coll_id = collection_id_for_social.clone();
                                                        let old_doc_id = talent_for_social.twitter_report_id.clone();
                                                        let pool_for_research = pool_for_social.clone();
                                                        let tid_for_research = talent_id_for_social.clone();

                                                        let handle = tokio::spawn(async move {
                                                            info!("[SOCIAL] Starting Twitter research for {}", name);
                                                            match client.research_platform(
                                                                "twitter",
                                                                &name,
                                                                email.as_deref(),
                                                                profile_url.as_deref(),
                                                                coll_id.as_deref(),
                                                                old_doc_id.as_deref(),
                                                            ).await {
                                                                Ok(resp) if resp.success => {
                                                                    info!("[SOCIAL] Twitter research completed successfully");
                                                                    let tldr = resp.report.as_ref().and_then(|r| r.tldr.clone());
                                                                    let _ = crate::database::update_talent_platform_research(
                                                                        &pool_for_research,
                                                                        tid_for_research,
                                                                        "twitter",
                                                                        resp.document_id.clone(),
                                                                        tldr,
                                                                    ).await;
                                                                    ("twitter".to_string(), resp.document_id)
                                                                },
                                                                Ok(resp) => {
                                                                    error!("[SOCIAL] Twitter research API failed: {:?}", resp.error);
                                                                    ("twitter".to_string(), None)
                                                                },
                                                                Err(e) => {
                                                                    error!("[SOCIAL] Twitter research error (timeout?): {}", e);
                                                                    ("twitter".to_string(), None)
                                                                }
                                                            }
                                                        });
                                                        research_handles.push(handle);
                                                    }

                                                    // Wait for all with timeout (5 min per request + buffer = 10 min total)
                                                    let global_timeout = std::time::Duration::from_secs(600);
                                                    let expected_count = research_handles.len();

                                                    let results = tokio::time::timeout(global_timeout, async {
                                                        let mut results = Vec::new();
                                                        for handle in research_handles {
                                                            match handle.await {
                                                                Ok((platform, doc_id)) => results.push((platform, doc_id)),
                                                                Err(e) => {
                                                                    error!("[SOCIAL] Task join error: {:?}", e);
                                                                    results.push(("unknown".to_string(), None));
                                                                }
                                                            }
                                                        }
                                                        results
                                                    }).await;

                                                    match results {
                                                        Ok(results) => Ok((results, expected_count)),
                                                        Err(_) => {
                                                            error!("[SOCIAL] Global timeout (10 min) exceeded!");
                                                            Err("timeout")
                                                        }
                                                    }
                                                };

                                                // Execute research and get result
                                                let research_outcome = research_future.await;

                                                // ALWAYS update status - this is the critical part
                                                let (final_status, reason) = match research_outcome {
                                                    Ok((results, expected)) => {
                                                        let mut gh_id = None;
                                                        let mut li_id = None;
                                                        let mut tw_id = None;
                                                        let mut success_count = 0;

                                                        for (platform, doc_id) in &results {
                                                            if doc_id.is_some() {
                                                                success_count += 1;
                                                            }
                                                            match platform.as_str() {
                                                                "github" => gh_id = doc_id.clone(),
                                                                "linkedin" => li_id = doc_id.clone(),
                                                                "twitter" => tw_id = doc_id.clone(),
                                                                _ => {}
                                                            }
                                                        }

                                                        // Update report IDs if any succeeded
                                                        if success_count > 0 {
                                                            let _ = crate::database::update_talent_social_report_ids(
                                                                &pool_for_social,
                                                                talent_id_for_social.clone(),
                                                                gh_id,
                                                                li_id,
                                                                tw_id,
                                                                None,
                                                            ).await;
                                                        }

                                                        if success_count == 0 {
                                                            ("failed", format!("0/{} platforms succeeded", expected))
                                                        } else {
                                                            ("completed", format!("{}/{} platforms succeeded", success_count, expected))
                                                        }
                                                    },
                                                    Err(_) => ("failed", "global timeout exceeded".to_string()),
                                                };

                                                info!("[SOCIAL] Final status update: {} ({})", final_status, reason);
                                                let _ = crate::database::update_talent_social_research_status(
                                                    &pool_for_social,
                                                    talent_id_for_social,
                                                    final_status,
                                                ).await;
                                            });
                                        }
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

                // Get or create collection for document upload
                let final_collection_id = if let Some(coll_id) = collection_id {
                    Some(coll_id)
                } else {
                    // No collection exists, create one now
                    warn!("======================================================================");
                    warn!("DOCUMENT UPLOAD: No collection_id for talent {}, creating collection now", talent_id);
                    warn!("======================================================================");

                    match client.create_collection(&talent_id, &talent_name).await {
                        Ok(response) => {
                            if response.success {
                                if let Some(collection) = response.collection {
                                    info!("COLLECTION CREATED: {}", collection.collection_id);

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

                                    Some(collection.collection_id)
                                } else {
                                    error!("COLLECTION CREATION: No collection in response");
                                    None
                                }
                            } else {
                                error!("COLLECTION CREATION: Failed - {:?}", response.error);
                                None
                            }
                        }
                        Err(e) => {
                            error!("COLLECTION CREATION ERROR: {}", e);
                            None
                        }
                    }
                };

                // Upload resume to collection
                if let Some(coll_id) = final_collection_id {
                    info!("======================================================================");
                    info!("DOCUMENT UPLOAD: Uploading resume to collection");
                    info!("======================================================================");
                    info!("Collection ID: {}", coll_id);
                    info!("Filename: {}", filename_for_upload);
                    if let Some(ref old_doc_id) = old_resume_document_id {
                        info!("Replacing old document: {}", old_doc_id);
                    }

                    match client.upload_document(
                        &coll_id,
                        &filename_for_upload,
                        &pdf_bytes_for_upload,
                        old_resume_document_id.as_deref(),
                    ).await {
                        Ok(doc_response) => {
                            if doc_response.success {
                                if let Some(doc) = doc_response.document {
                                    info!("======================================================================");
                                    info!("DOCUMENT UPLOAD: Success!");
                                    info!("Document ID: {}", doc.document_id);
                                    info!("======================================================================");

                                    // Update talent with new document_id
                                    match crate::database::update_talent_resume_document_id(
                                        &pool_clone,
                                        talent_id.clone(),
                                        Some(doc.document_id.clone()),
                                    ).await {
                                        Ok(_) => {
                                            info!("SUCCESS: Updated talent {} with document_id {}", talent_id, doc.document_id);
                                        },
                                        Err(e) => {
                                            error!("FAILED to update talent {} with document_id: {}", talent_id, e);
                                        }
                                    }
                                }
                            } else {
                                error!("DOCUMENT UPLOAD: Failed - {:?}", doc_response.error);
                            }
                        }
                        Err(e) => {
                            error!("======================================================================");
                            error!("DOCUMENT UPLOAD ERROR: {}", e);
                            error!("======================================================================");
                        }
                    }
                } else {
                    error!("DOCUMENT UPLOAD: Skipped - no collection available");
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
