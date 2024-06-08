use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub auth_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub profile_name: String,        // Starts as username, can be changed
    pub profile_photo: String,       // Has default photo for new users
    pub games: Vec<uuid::Uuid>,      // Games are globally seen in the server. These are the games the user owns
    pub rulesets: Vec<uuid::Uuid>,   // The rulesets this user has created
    pub settings: Vec<uuid::Uuid>,   // The settings this user has created
    pub characters: Vec<uuid::Uuid>, // Characters stored in a local per-user format. These are the character the user owns
}