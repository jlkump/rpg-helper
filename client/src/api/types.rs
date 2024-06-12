use chrono::prelude::*;
use std::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RegistrationError {
    UsernameTaken,
    EmailTaken,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginError {
    UnknownUsernameOrPassword,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserDataError {
    UserNotFound(uuid::Uuid)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthError {
    NotLoggedIn,
    InvalidToken,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NotLoggedIn => write!(f, "Unauthorized: User not logged in."),
            AuthError::InvalidToken => write!(f, "Unauthorized: Invalid token."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataResponse {
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub auth_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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