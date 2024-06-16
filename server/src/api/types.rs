use chrono::prelude::*;
use std::{collections::HashSet, fmt::{Debug, Display}};
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
    pub storage_used: i64,
    pub storage_limit: i64,
    pub is_donor: bool,
    pub joined_games: HashSet<uuid::Uuid>,
    pub favorited_rulesets: HashSet<uuid::Uuid>,
    pub favorited_settings: HashSet<uuid::Uuid>,
    pub owned_games: HashSet<uuid::Uuid>,
    pub owned_rulesets: HashSet<uuid::Uuid>,
    pub owned_settings: HashSet<uuid::Uuid>,
    pub owned_characters: HashSet<uuid::Uuid>,
}