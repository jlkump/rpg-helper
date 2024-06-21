use chrono::prelude::*;
use std::{collections::HashSet, fmt::{Debug, Display}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    pub error: String,
    pub message: String,
}

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
    UserIdNotFound(uuid::Uuid),
    UsernameNotFound(String)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthError {
    NotLoggedIn,
    InvalidToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UploadError {
    UserNotFound(uuid::Uuid),
    FileTooLarge,
    UnsupportedFileType,
    InsufficientUserStorage(i64, i64), // The amount requested and the amount the user has left
    NameConflict(String), // Name conflict with existing user file upload
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NotLoggedIn => write!(f, "Unauthorized: User not logged in."),
            AuthError::InvalidToken => write!(f, "Unauthorized: Invalid token."),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ImageUrl {
    ExternalPath(String),
    InternalServerPath(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub auth_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserData {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub profile_name: String,        // Starts as username, can be changed
    pub profile_photo: String,       // Has default photo for new users
    pub profile_banner: String,      // Has default photo for new users
    pub storage_used: i64,
    pub storage_limit: i64,
    pub verified: bool,
    pub is_donor: bool,
    pub friends: HashSet<uuid::Uuid>,
    pub friend_requests: HashSet<FriendRequest>,
    pub sent_requests: HashSet<uuid::Uuid>,
    pub blocked_users: HashSet<uuid::Uuid>,
    pub game_invites: HashSet<uuid::Uuid>,
    pub sent_invites: HashSet<GameInvite>,
    pub joined_games: HashSet<uuid::Uuid>,
    pub favorited_rulesets: HashSet<uuid::Uuid>,
    pub favorited_settings: HashSet<uuid::Uuid>,
    pub owned_games: HashSet<uuid::Uuid>,
    pub owned_rulesets: HashSet<uuid::Uuid>,
    pub owned_settings: HashSet<uuid::Uuid>,
    pub owned_characters: HashSet<uuid::Uuid>,
    pub last_read_news: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct PublicUserData {
    pub id: uuid::Uuid,              // Used for backend, but no need to keep private
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub profile_name: String,        // Starts as username, can be changed
    pub profile_photo: String,       // Has default photo for new users
    pub profile_banner: String,      // Has default photo for new users
    pub profile_text: String,
    pub profile_catchphrase: String,
    pub is_donor: bool, // Happy bird wearing wizard hat svg icon on profile for those who donated
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct FriendRequest {
    pub sent_by: uuid::Uuid,
    pub game: uuid::Uuid,
    pub read: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct GameInvite {
    pub sent_to: uuid::Uuid,
    pub game: uuid::Uuid,
}