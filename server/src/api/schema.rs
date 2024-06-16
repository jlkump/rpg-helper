use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistrationSchema {
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The actual password, won't be stored in DB
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginSchema {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserUpdateSchema {
    Email(String),
    Password(String),
    ProfileName(String),
    ProfilePicture(String)
}

#[derive(Debug, Deserialize)]
pub struct FileUploadMetadata {
    pub name: String,
}

