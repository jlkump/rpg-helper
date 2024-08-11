use serde::{Deserialize, Serialize};

use super::types::{ImageData, RulesetId};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistrationSchema {
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The actual password, won't be stored in DB
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginSchema {
    pub username: String, // Probably want to change to email?
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserUpdateSchema {
    Email(String),
    Password(String),
    ProfileName(String),
    ProfileText(String),
    ProfileCatchphrase(String),
    ProfilePicture(ImageData),
    ProfileBanner(ImageData),
    FavoritedRuleset(uuid::Uuid),
    FavoritedSetting(uuid::Uuid),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchSchema {
    pub search_string: String,
    pub sorting: SortOptions,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum SortOptions {
    Alphabetical,
    ReverseAlphabetical,
    LastUpdated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RulesetCreateSchema {
    pub ruleset_name: String,
}