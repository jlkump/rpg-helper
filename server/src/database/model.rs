use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Format properly for Sled database
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,        // UUID for user
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The Password hash and salt
    pub role: String,          // Admin?
    pub profile_photo: String, // Has default photo for new users
    pub verified: bool,        // Not 100% sure what this is for, perhaps email verification? Then we delete old users that haven't been verified?
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Users will a need a list of games they are participating in
// and a list of characters they own. The characters are linked to a 
// game by the game ID. They also specify the Ruleset / Setting they require. 
// Characters can be moved between games if the ruleset / setting matches.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDetails {
    pub id: uuid::Uuid, // Key 
}