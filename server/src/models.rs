use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq, Debug, FromRow)]
pub struct Talent {
    pub id: String,
    pub name: String,
    pub email: String,
    pub handle: String,
    pub skills: String,
    pub avatar: Option<String>,
    pub title: String,
    pub location: Option<String>,
    pub experience: String,
    pub bio: Option<String>,
    pub verified: i32,
    pub created_at: String,
    // Resume-extracted fields
    pub resume_experiences: Option<String>,  // JSON array of experiences
    pub linkedin_url: Option<String>,
    pub x_url: Option<String>,
    pub github_url: Option<String>,
    pub gitlab_url: Option<String>,
    // xAI Collections integration
    pub collection_id: Option<String>,
    pub resume_document_id: Option<String>,
}


#[derive(Deserialize, Apiv2Schema)]
pub struct CreateTalentRequest {
    pub name: String,
    pub email: String,
    pub handle: String,
    pub avatar: Option<String>,
    pub title: String,
    pub location: Option<String>,
    pub experience: String,
    pub skills: String, // comma-separated
    pub bio: Option<String>,
    pub verified: bool,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct UpdateTalentRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub handle: Option<String>,
    pub avatar: Option<String>,
    pub title: Option<String>,
    pub location: Option<String>,
    pub experience: Option<String>,
    pub skills: Option<String>,
    pub bio: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Serialize, Apiv2Schema)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

#[derive(Serialize, Apiv2Schema)]
pub struct BulkDeleteResponse {
    pub deleted_count: u64,
    pub total_requested: usize,
}

// Job models

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq, Debug, FromRow)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub company_logo: Option<String>,
    pub location: Option<String>,
    pub location_type: String,  // remote, onsite, hybrid
    pub employment_type: String, // full-time, part-time, contract
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub salary_currency: Option<String>,
    pub skills_required: String, // comma-separated
    pub experience_level: String, // entry, mid, senior, lead
    pub status: String, // active, closed, draft
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct CreateJobRequest {
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub company_logo: Option<String>,
    pub location: Option<String>,
    pub location_type: String,
    pub employment_type: String,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub salary_currency: Option<String>,
    pub skills_required: String,
    pub experience_level: String,
    pub expires_at: Option<String>,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct UpdateJobRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub company_name: Option<String>,
    pub company_logo: Option<String>,
    pub location: Option<String>,
    pub location_type: Option<String>,
    pub employment_type: Option<String>,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub salary_currency: Option<String>,
    pub skills_required: Option<String>,
    pub experience_level: Option<String>,
    pub status: Option<String>,
    pub expires_at: Option<String>,
}

// Application models

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq, Debug, FromRow)]
pub struct Application {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub resume_data: Option<String>,      // Base64 encoded resume
    pub resume_filename: Option<String>,
    pub resume_content_type: Option<String>,
    pub cover_letter: Option<String>,
    pub status: String,                    // pending, reviewed, accepted, rejected
    pub created_at: String,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct CreateApplicationRequest {
    pub talent_id: String,
    pub job_id: String,
    pub resume_data: Option<String>,       // Base64 encoded resume
    pub resume_filename: Option<String>,
    pub resume_content_type: Option<String>,
    pub cover_letter: Option<String>,
}

#[derive(Serialize, Apiv2Schema)]
pub struct ApplicationResponse {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub has_resume: bool,
    pub resume_filename: Option<String>,
    pub cover_letter: Option<String>,
    pub status: String,
    pub created_at: String,
}

// RLHF Feedback models

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq, Debug)]
pub struct Feedback {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub recruiter_id: Option<String>,
    pub feedback_type: String,  // "upvote" or "downvote"
    pub expected_rank: Option<f64>,  // Where algorithm ranked them (0.0-1.0)
    pub actual_performance: String,  // better_than_expected, worse_than_expected, as_expected
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct CreateFeedbackRequest {
    pub talent_id: String,
    pub job_id: String,
    pub recruiter_id: Option<String>,
    pub feedback_type: String,  // "upvote" or "downvote"
    pub expected_rank: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Serialize, Apiv2Schema)]
pub struct FeedbackStats {
    pub talent_id: String,
    pub job_id: Option<String>,
    pub upvotes: i32,
    pub downvotes: i32,
    pub net_score: i32,
    pub total_feedback: i32,
}

// GRPO Ranking models

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq, Debug)]
pub struct CandidateRanking {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub rank_score: f64,  // 0.0 to 1.0
    pub rank_position: i32,  // 1, 2, 3...
    pub confidence: Option<f64>,
    pub match_factors: String,  // JSON object with breakdown
    pub model_version: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatchFactors {
    pub skills_match: f64,
    pub experience_match: f64,
    pub location_match: f64,
    pub title_match: f64,
    pub overall_fit: f64,
}

#[derive(Serialize, Apiv2Schema)]
pub struct RankedCandidate {
    pub talent: Talent,
    pub ranking: CandidateRanking,
    pub feedback_stats: Option<FeedbackStats>,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct RankCandidatesRequest {
    pub job_id: String,
    pub talent_ids: Option<Vec<String>>,  // If None, rank all candidates
    pub use_feedback: bool,  // Whether to incorporate RLHF feedback
}