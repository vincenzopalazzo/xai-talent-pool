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
}
