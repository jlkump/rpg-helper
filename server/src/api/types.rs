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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ImageData {
    ExternalPath(String),
    InternalUpload(UploadImageData),
}

impl ImageData {
    pub fn to_src(&self) -> &str {
        match self {
            ImageData::ExternalPath(s) => s,
            ImageData::InternalUpload(data) => &data.src,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UploadImageData {
    pub src: String,
    pub name: String,
    pub size: i64, // In Bytes
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ImageUrl {
    External(String), // The external src link
    Internal(String), // Just the path to the uploaded file. 
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

//////////////////////////////////////
///     Data Model for Tool        ///
//////////////////////////////////////

pub type Name = String;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Value {
    t: Type,
    d: Data,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum Type { // Important to note. Changing types in-game will be very difficult. Might be best to restrict it
    Num,
    List(Box<Type>),
    Enum(Vec<String>),
    Meta(Name),     // Name of the meta-type
    // Equation(Equation),   // Equation is owned by the type and thus not named
    MetaRef(Name), // The name of the meta-type the ref refers to
    // Input(RestrictedInput),
    // DieRoll(DieRoll),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Data {
    Num(f32),
    Text(String),
    List(Vec<Value>),
    Enum(String),
    Meta(MetaInst), // The meta type is accessed by the field name
    Equation,
    Input, // Maybe store last input and whether it has been used?
    DieRoll, // IDK, but can store stuff for later
    MetaRef(Name) // Name of the actual reference to the meta instance
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaType {
    pub name: Name,
    pub fields: Vec<Type>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct MetaInst {
    pub name: Name,
    pub fields: Vec<Value>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaRef { // MetaRef could also be MetaInst
    // Hold data on the ruleset / setting it came from?
    pub type_name: Name,
    pub ref_name: Name,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Event {
    // TODO: Also define ordering trait based on time
    year: Value,    // Defined specifically by a Year  meta-type required to be placed in the rule-set. Must be a num
    month: Value,   // Defined specifically by a Month meta-type required to be placed in the rule-set. Must be a num
    day: Value,     // Defined specifically by a Day   meta-type required to be placed in the rule-set. Must be a num
    event_type: EventType,  // Defined by a EventType meta-type. The event type holds the reference to the effect
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EventType {
    name: Name,
    effect: Effect,
    // TODO: Restrictions
    // Also, display img?
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Effect {
    target: MetaRef,
    old_value: Value,
    new_value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Timeline {
    events: Vec<Event>,
}