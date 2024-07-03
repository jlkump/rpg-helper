use chrono::prelude::*;
use std::{collections::HashSet, fmt::Debug};
use serde::{Deserialize, Serialize};

/////////////////////////////////////////
//              Errors                ///
/////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    pub error: ServerErrorType,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerErrorType {
    Authorization(AuthError),
    NotFound(NotFoundError),
    InsufficientStorage(InsufficientStorageError),
    FileTooLarge(FileTooLargeError),
    Conflict(ConflictError),
    Unsupported(UnsupportedError),
    InternalError(InternalError),
    NotImplemented, // Error for in-progress development
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthError {
    WrongPasswordOrUsername,
    NotLoggedIn,
    InvalidToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NotFoundError {
    UserId(uuid::Uuid),
    Username(String),
    File(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsufficientStorageError {
    pub current: i64,
    pub maximum: i64,
    pub given_increase: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileTooLargeError {
    pub given_file_size: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConflictError {
    Username,
    Email,
    FileName,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UnsupportedError {
    FileType, // Filetype given is not supported
}

#[derive(Debug, Deserialize, Serialize)]
pub enum InternalError {
    Database,
    Filesystem,
    Parse,
    Encrypt,
    Other(String), // General all-catch error, should primarly be used for development and replaced when a pattern emerges.
}

/////////////////////////////////////////
//              Data                  ///
/////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ImageUrl {
    ExternalPath(String),
    InternalServerPath(String)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ImageData {
    pub src: String,
    pub name: String,
    pub is_external: bool,
    pub dimen: (i64, i64),
    pub size: i64, // In Bytes
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