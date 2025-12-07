//! Grok service client for resume analysis

use log::{info, error, debug};
use reqwest::multipart;
use serde::{Deserialize, Serialize};

/// Experience summary extracted from resume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceSummary {
    pub company: String,
    pub role: String,
    pub duration: Option<String>,
    pub summary: String,
}

/// Profile URLs extracted from resume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUrls {
    pub linkedin: Option<String>,
    pub x: Option<String>,
    pub github: Option<String>,
    pub gitlab: Option<String>,
}

/// Screening result from Grok service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningResult {
    pub talent_id: String,
    pub experiences: Vec<ExperienceSummary>,
    pub urls: ProfileUrls,
}

/// Response from Grok screening endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningResponse {
    pub success: bool,
    pub result: Option<ScreeningResult>,
    pub error: Option<String>,
}

/// Talent info for screening request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub handle: String,
    pub skills: String,
    pub title: String,
    pub location: Option<String>,
    pub experience: String,
    pub bio: Option<String>,
}

/// Collection info from Grok service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub collection_id: String,
    pub collection_name: String,
}

/// Response from Grok collection endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub success: bool,
    pub collection: Option<CollectionInfo>,
    pub error: Option<String>,
}

/// Request to create a collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub talent_id: String,
    pub talent_name: String,
}

/// Document info from Grok service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub document_id: String,
    pub document_name: String,
}

/// Response from Grok document upload endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResponse {
    pub success: bool,
    pub document: Option<DocumentInfo>,
    pub error: Option<String>,
}

/// Request for platform research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRequest {
    pub name: String,
    pub email: Option<String>,
    pub profile_url: Option<String>,
    pub collection_id: Option<String>,
    pub old_document_id: Option<String>,
}

/// Platform research report (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformReport {
    pub platform: String,
    pub person_name: String,
    pub created_at: Option<String>,
    pub tldr: Option<String>,
    pub raw_content: String,
}

/// Response from platform research endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResponse {
    pub success: bool,
    pub report: Option<PlatformReport>,
    pub document_id: Option<String>,
    pub error: Option<String>,
}

/// Grok service client
pub struct GrokClient {
    base_url: String,
    client: reqwest::Client,
}

impl GrokClient {
    /// Create a new Grok client
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Analyze a resume using the Grok service
    pub async fn analyze_resume(
        &self,
        talent_info: &TalentInfo,
        pdf_data: &[u8],
        filename: &str,
    ) -> Result<ScreeningResponse, String> {
        info!("[GrokClient] Preparing request...");
        info!("[GrokClient] Talent: {} ({})", talent_info.name, talent_info.id);

        let talent_json = serde_json::to_string(talent_info)
            .map_err(|e| format!("Failed to serialize talent info: {}", e))?;

        debug!("[GrokClient] Talent JSON: {}", talent_json);

        let form = multipart::Form::new()
            .text("talent_info", talent_json.clone())
            .part(
                "resume",
                multipart::Part::bytes(pdf_data.to_vec())
                    .file_name(filename.to_string())
                    .mime_str("application/pdf")
                    .map_err(|e| format!("Failed to set MIME type: {}", e))?,
            );

        let url = format!("{}/api/v1/screening/initial", self.base_url);
        info!("[GrokClient] Sending POST to: {}", url);
        info!("[GrokClient] PDF size: {} bytes, filename: {}", pdf_data.len(), filename);

        let response = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                error!("[GrokClient] Request failed: {}", e);
                format!("Failed to send request to Grok service: {}", e)
            })?;

        let status = response.status();
        info!("[GrokClient] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("[GrokClient] Error response body: {}", body);
            return Err(format!(
                "Grok service returned error {}: {}",
                status, body
            ));
        }

        // Get the raw response text first for debugging
        let response_text = response.text().await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        info!("[GrokClient] Raw response: {}", response_text);

        // Parse the JSON
        let parsed: ScreeningResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("[GrokClient] JSON parse error: {}", e);
                format!("Failed to parse Grok response: {}", e)
            })?;

        info!("[GrokClient] Parsed response - success: {}", parsed.success);

        Ok(parsed)
    }

    /// Create a collection for a talent
    pub async fn create_collection(
        &self,
        talent_id: &str,
        talent_name: &str,
    ) -> Result<CollectionResponse, String> {
        info!("[GrokClient] Creating collection for talent: {} ({})", talent_name, talent_id);

        let request = CreateCollectionRequest {
            talent_id: talent_id.to_string(),
            talent_name: talent_name.to_string(),
        };

        let url = format!("{}/api/v1/collections/create", self.base_url);
        info!("[GrokClient] Sending POST to: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("[GrokClient] Collection request failed: {}", e);
                format!("Failed to send collection request to Grok service: {}", e)
            })?;

        let status = response.status();
        info!("[GrokClient] Collection response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("[GrokClient] Collection error response: {}", body);
            return Err(format!(
                "Grok service returned error {}: {}",
                status, body
            ));
        }

        let response_text = response.text().await
            .map_err(|e| format!("Failed to read collection response: {}", e))?;

        info!("[GrokClient] Collection response: {}", response_text);

        let parsed: CollectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("[GrokClient] Collection JSON parse error: {}", e);
                format!("Failed to parse collection response: {}", e)
            })?;

        info!("[GrokClient] Collection created - success: {}", parsed.success);
        if let Some(ref collection) = parsed.collection {
            info!("[GrokClient] Collection ID: {}", collection.collection_id);
        }

        Ok(parsed)
    }

    /// Upload a document to a collection
    pub async fn upload_document(
        &self,
        collection_id: &str,
        document_name: &str,
        document_data: &[u8],
        old_document_id: Option<&str>,
    ) -> Result<DocumentResponse, String> {
        info!("[GrokClient] Uploading document to collection: {}", collection_id);
        info!("[GrokClient] Document name: {}, size: {} bytes", document_name, document_data.len());
        if let Some(old_id) = old_document_id {
            info!("[GrokClient] Replacing old document: {}", old_id);
        }

        let mut form = multipart::Form::new()
            .text("collection_id", collection_id.to_string())
            .text("document_name", document_name.to_string())
            .part(
                "document",
                multipart::Part::bytes(document_data.to_vec())
                    .file_name(document_name.to_string())
                    .mime_str("application/pdf")
                    .map_err(|e| format!("Failed to set MIME type: {}", e))?,
            );

        if let Some(old_id) = old_document_id {
            form = form.text("old_document_id", old_id.to_string());
        }

        let url = format!("{}/api/v1/collections/documents/upload", self.base_url);
        info!("[GrokClient] Sending POST to: {}", url);

        let response = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                error!("[GrokClient] Document upload request failed: {}", e);
                format!("Failed to upload document to Grok service: {}", e)
            })?;

        let status = response.status();
        info!("[GrokClient] Document upload response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("[GrokClient] Document upload error response: {}", body);
            return Err(format!(
                "Grok service returned error {}: {}",
                status, body
            ));
        }

        let response_text = response.text().await
            .map_err(|e| format!("Failed to read document upload response: {}", e))?;

        info!("[GrokClient] Document upload response: {}", response_text);

        let parsed: DocumentResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("[GrokClient] Document upload JSON parse error: {}", e);
                format!("Failed to parse document upload response: {}", e)
            })?;

        info!("[GrokClient] Document upload - success: {}", parsed.success);
        if let Some(ref doc) = parsed.document {
            info!("[GrokClient] New document ID: {}", doc.document_id);
        }

        Ok(parsed)
    }

    /// Research a candidate on a specific platform
    pub async fn research_platform(
        &self,
        platform: &str,
        name: &str,
        email: Option<&str>,
        profile_url: Option<&str>,
        collection_id: Option<&str>,
        old_document_id: Option<&str>,
    ) -> Result<ResearchResponse, String> {
        info!("[GrokClient] Researching {} on {}", name, platform);
        if let Some(url) = profile_url {
            info!("[GrokClient] Using profile URL: {}", url);
        }

        let request = ResearchRequest {
            name: name.to_string(),
            email: email.map(|s| s.to_string()),
            profile_url: profile_url.map(|s| s.to_string()),
            collection_id: collection_id.map(|s| s.to_string()),
            old_document_id: old_document_id.map(|s| s.to_string()),
        };

        let url = format!("{}/api/v1/candidate-research/{}", self.base_url, platform);
        info!("[GrokClient] Sending POST to: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("[GrokClient] Research request failed: {}", e);
                format!("Failed to send research request to Grok service: {}", e)
            })?;

        let status = response.status();
        info!("[GrokClient] Research response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("[GrokClient] Research error response: {}", body);
            return Err(format!(
                "Grok service returned error {}: {}",
                status, body
            ));
        }

        let response_text = response.text().await
            .map_err(|e| format!("Failed to read research response: {}", e))?;

        debug!("[GrokClient] Research response: {}", response_text);

        let parsed: ResearchResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("[GrokClient] Research JSON parse error: {}", e);
                format!("Failed to parse research response: {}", e)
            })?;

        info!("[GrokClient] Research completed - success: {}", parsed.success);
        if let Some(ref doc_id) = parsed.document_id {
            info!("[GrokClient] Document ID: {}", doc_id);
        }

        Ok(parsed)
    }
}
