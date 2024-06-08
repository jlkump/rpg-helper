use actix_web::web::Buf;
use bcrypt::DEFAULT_COST;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

use crate::{config::Config, database::get_data};

pub struct UserDB {
    users: Db,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct User {
    pub id: uuid::Uuid
}

impl From<uuid::Uuid> for User {
    fn from(id: uuid::Uuid) -> Self {
        User {
            id
        }
    }
}

impl From<&uuid::Uuid> for User {
    fn from(value: &uuid::Uuid) -> Self {
        User {
            id: value.clone(),
        }
    }
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

#[derive(Debug, Deserialize)]
pub struct UserRegistrationSchema {
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The actual password, won't be stored in DB
}

pub enum RegistrationResponse {
    Success(User),
    UsernameTaken,
    EmailTaken,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub username: String,
    pub password: String,
}

pub enum LoginResponse {
    Success(User),
    UnknownUsername,
    WrongPassword,
}

impl UserDB {
    pub fn open(config: &Config) -> Self {
        UserDB {
            users: sled::open(format!("{}/users", config.database.root_path)).unwrap()
        }
    }

    pub fn register_user(&self, registration_data: UserRegistrationSchema) -> RegistrationResponse {
        if self.get_user_by_username(&registration_data.username).is_some() {
            RegistrationResponse::UsernameTaken
        } else if self.get_user_by_email(&registration_data.email).is_some() {
            RegistrationResponse::EmailTaken
        } else {
            let data_tree = self.open_secure_data_tree();
            let details_tree = self.open_general_data_tree();

            let v = UserSecureData::default(registration_data.username, registration_data.email, registration_data.password);
            data_tree.insert(v.id, bincode::serialize(&v).unwrap()).unwrap();
            details_tree.insert(v.id, bincode::serialize(&UserGeneralData::new(&v.username)).unwrap()).unwrap();
            RegistrationResponse::Success(User {id: v.id})
        }
    }

    pub fn delete_user(&self, user: User) {
        let data_tree = self.open_secure_data_tree();
        let details_tree = self.open_general_data_tree();

        // Fails when db error occurs, not when the db doesn't contain the user to remove, so fail case is rare
        data_tree.remove(user.id).unwrap();
        details_tree.remove(user.id).unwrap();
    }

    pub fn login_user(&self, login_data: UserLoginSchema) -> LoginResponse {
        if let Some(user_id) = self.get_user_by_username(&login_data.username) {
            let data = self.get_secure_data(&user_id).unwrap();
            if bcrypt::verify(login_data.password, &data.password).unwrap() {
                LoginResponse::Success(user_id)
            } else {
                LoginResponse::WrongPassword
            }
        } else {
            LoginResponse::UnknownUsername
        }
    }

    pub fn update_user(&self, user: User, new_user_data: UserData) {
        if let Some(secure) = self.get_secure_data(&user) {
            if secure.email.ne(&new_user_data.email) || secure.username.ne(&new_user_data.username) {
                // Update secure data
                let updated = secure.update(&new_user_data);
                self.open_secure_data_tree().insert(user.id, bincode::serialize(&updated).unwrap()).unwrap();
            }
        }
        if let Some(general) = self.get_general_data(&user) {
            let updated = general.update(&new_user_data);
            self.open_general_data_tree().insert(user.id, bincode::serialize(&updated).unwrap()).unwrap();
        }
    }

    pub fn get_data(&self, user: User) -> Option<UserData> {
        if let Some(secure) = self.get_secure_data(&user) {
            if let Some(general) = self.get_general_data(&user) {
                Some(UserData {
                    username: secure.username,
                    email: secure.email,
                    created_at: secure.created_at,
                    profile_name: general.profile_name,
                    profile_photo: general.profile_photo,
                    games: general.games,
                    rulesets: general.rulesets,
                    settings: general.settings,
                    characters: general.characters
                })
            }
            else {
                None
            }
        }
        else {
            None
        }
    }


    fn get_user_by_username(&self, username: &String) -> Option<User> {
        for row_data in &self.open_secure_data_tree() {
            let (l, r) = row_data.unwrap();
            let id: uuid::Uuid = bincode::deserialize_from(l.reader()).unwrap();
            let data: UserSecureData = bincode::deserialize_from(r.reader()).unwrap();
            if data.username.eq(username) {
                return Some(User { id });
            }
        }
        None
    }

    fn get_user_by_email(&self, email: &String) -> Option<User> {
        for row_data in &self.open_secure_data_tree() {
            let (l, r) = row_data.unwrap();
            let id: uuid::Uuid = bincode::deserialize_from(l.reader()).unwrap();
            let data: UserSecureData = bincode::deserialize_from(r.reader()).unwrap();
            if data.email.eq(email) {
                return Some(User { id });
            }
        }
        None
    }

    fn get_secure_data(&self, user: &User) -> Option<UserSecureData> {
        let tree = self.open_secure_data_tree();
        get_data(&tree, &user.id)
    }

    fn get_general_data(&self, user: &User) -> Option<UserGeneralData> {
        let tree = self.open_general_data_tree();
        get_data(&tree, &user.id)
    }

    fn open_secure_data_tree(&self) -> Tree {
        self.users.open_tree(b"secure").unwrap()
    }

    fn open_general_data_tree(&self) -> Tree {
        self.users.open_tree(b"general").unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserSecureData {
    id: uuid::Uuid,        // UUID for user
    username: String,      // Username for the user profile
    email: String,         // Email of the user
    password: String,      // The Password hash and salt
    role: String,          // Admin?
    verified: bool,        // Not 100% sure what this is for, perhaps email verification? Then we delete old users that haven't been verified?
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl UserSecureData {
    fn default(username: String, email: String, password: String) -> UserSecureData {
        UserSecureData {
            id: uuid::Uuid::new_v4(),
            username,
            email,
            password: bcrypt::hash(password, DEFAULT_COST).unwrap(),
            role: String::from("User"),
            verified: false,
            created_at: Some(chrono::offset::Utc::now()),
            updated_at: Some(chrono::offset::Utc::now()),
        }
    }
    fn update(&self, new_data: &UserData) -> UserSecureData {
        UserSecureData { 
            id: self.id, 
            username: new_data.username.clone(), 
            email: new_data.email.clone(), 
            password: self.password.clone(), 
            role: self.role.clone(), 
            verified: self.verified, 
            created_at: self.created_at, 
            updated_at: Some(chrono::offset::Utc::now()) 
        }
    }
}

// Users will a need a list of games they are participating in
// and a list of characters they own. The characters are linked to a 
// game by the game ID. They also specify the Ruleset / Setting they require. 
// Characters can be moved between games if the ruleset / setting matches.

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserGeneralData {
    profile_name: String,        // Starts as username, can be changed
    profile_photo: String,       // Has default photo for new users
    games: Vec<uuid::Uuid>,      // Games are globally seen in the server. These are the games the user owns
    rulesets: Vec<uuid::Uuid>,   // The rulesets this user has created
    settings: Vec<uuid::Uuid>,   // The settings this user has created
    characters: Vec<uuid::Uuid>, // Characters stored in a local per-user format. These are the character the user owns
}

impl UserGeneralData {
    fn new(username: &str) -> UserGeneralData {
        UserGeneralData {
            profile_name: username.to_string(),
            profile_photo: String::from("default_profile.png"),
            games: vec![],
            rulesets: vec![],
            settings: vec![],
            characters: vec![],
        }
    }

    fn update(&self, new_data: &UserData) -> UserGeneralData {
        UserGeneralData {
            profile_name: new_data.profile_name.clone(),
            profile_photo: new_data.profile_photo.clone(),
            games: new_data.games.clone(),
            rulesets: new_data.rulesets.clone(),
            settings: new_data.settings.clone(),
            characters: new_data.characters.clone()
        }
    }
}