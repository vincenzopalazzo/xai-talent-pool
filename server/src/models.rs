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