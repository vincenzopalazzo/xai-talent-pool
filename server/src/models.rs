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