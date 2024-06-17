use std::collections::{HashMap, HashSet};

use actix_web::web::Buf;
use bcrypt::DEFAULT_COST;
use chrono::prelude::*;
use log::{error, trace};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

use crate::{api::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{PublicUserData, UserData}}, config::Config, database::get_data};

pub struct UserDB {
    users: Db,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct User {
    pub id: uuid::Uuid
}

impl User {
    pub fn from_username(user_db: &UserDB, username: &String) -> Option<User> {
        user_db.get_user_by_username(username)
    }
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

#[derive(Debug)]
pub enum RegistrationResponse {
    Success(User),
    UsernameTaken,
    EmailTaken,
}

#[derive(Debug)]
pub enum LoginResponse {
    Success(User),
    UnknownUsername,
    WrongPassword,
}

#[derive(Debug)]
pub enum UpdateResponse {
    Success,
    UserNotFound,
    DatabaseErr
}

impl UserDB {
    pub(super) fn open(config: &Config) -> Self {
        UserDB {
            users: sled::open(format!("{}/users", config.database.database_path)).unwrap()
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

            let v = UserSecureData::new(registration_data.username, registration_data.email, registration_data.password);
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

    pub fn update_user_email(&self, user: User, new_email: String) -> UpdateResponse {
        let tree = self.open_secure_data_tree();
        if let Some(mut secure) = get_data::<UserSecureData, uuid::Uuid>(&tree, &user.id) {
            secure.update_email(new_email);
            tree.insert(user.id, bincode::serialize(&secure).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub fn update_user_password(&self, user: User, new_password: String) -> UpdateResponse {
        let tree = self.open_secure_data_tree();
        if let Some(mut secure) = get_data::<UserSecureData, uuid::Uuid>(&tree, &user.id) {
            secure.update_password(new_password);
            tree.insert(user.id, bincode::serialize(&secure).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub fn update_user_verified(&self, user: User, verified: bool) -> UpdateResponse {
        let tree = self.open_secure_data_tree();
        if let Some(mut secure) = get_data::<UserSecureData, uuid::Uuid>(&tree, &user.id) {
            secure.update_verified(verified);
            tree.insert(user.id, bincode::serialize(&secure).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub(super) fn update_user_storage(&self, user: User, change: i64) -> UpdateResponse {
        let tree = self.open_secure_data_tree();
        if let Some(mut secure) = get_data::<UserSecureData, uuid::Uuid>(&tree, &user.id) {
            secure.update_storage(change);
            tree.insert(user.id, bincode::serialize(&secure).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub fn update_user_donation(&self, user: User, amount: i64) -> UpdateResponse {
        let tree = self.open_secure_data_tree();
        if let Some(mut secure) = get_data::<UserSecureData, uuid::Uuid>(&tree, &user.id) {
            secure.update_donation(amount);
            tree.insert(user.id, bincode::serialize(&secure).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub fn update_user_profile_name(&self, user: User, profile_name: String) -> UpdateResponse {
        let tree = self.open_general_data_tree();
        if let Some(mut general) = get_data::<UserGeneralData, uuid::Uuid>(&tree, &user.id) {
            general.update_profile_name(profile_name);
            tree.insert(user.id, bincode::serialize(&general).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub(super) fn user_join_game(&self, user: User, game_id: uuid::Uuid) -> UpdateResponse {
        let tree = self.open_general_data_tree();
        if let Some(mut general) = get_data::<UserGeneralData, uuid::Uuid>(&tree, &user.id) {
            general.join_game(game_id);
            tree.insert(user.id, bincode::serialize(&general).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub(super) fn user_leave_game(&self, user: User, game_id: uuid::Uuid) -> UpdateResponse {
        let tree = self.open_general_data_tree();
        if let Some(mut general) = get_data::<UserGeneralData, uuid::Uuid>(&tree, &user.id) {
            general.leave_game(game_id);
            tree.insert(user.id, bincode::serialize(&general).unwrap()).unwrap();
            UpdateResponse::Success
        } else {
            UpdateResponse::UserNotFound
        }
    }

    pub fn user_exists(&self, user: &User) -> bool {
        self.get_secure_data(&user).is_some()
    }

    pub fn get_data(&self, user: User, config: &Config) -> Option<UserData> {
        if let Some(secure) = self.get_secure_data(&user) {
            if let Some(general) = self.get_general_data(&user) {
                Some(UserData {
                    username: secure.username.clone(),
                    email: secure.email.clone(),
                    created_at: secure.created_at,
                    profile_name: general.profile_name,
                    profile_photo: general.profile_photo.to_string(&config),
                    favorited_rulesets: general.favorited_rulesets,
                    favorited_settings: general.favorited_settings,
                    joined_games: general.joined_games,
                    owned_games: general.owned_games,
                    owned_rulesets: general.owned_rulesets,
                    owned_settings: general.owned_settings,
                    owned_characters: general.owned_characters,
                    storage_used: secure.storage_used,
                    storage_limit: secure.get_storage_limit(),
                    is_donor: secure.donated.is_some() || secure.monthly_donor,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_public_data(&self, user: User, config: &Config) -> Option<PublicUserData> {
        if let Some(secure) = self.get_secure_data(&user) {
            if let Some(general) = self.get_general_data(&user) {
                Some(PublicUserData {
                    username: secure.username.clone(),
                    created_at: secure.created_at,
                    profile_name: general.profile_name,
                    profile_photo: general.profile_photo.to_string(&config),
                    profile_text: general.profile_text,
                    profile_catchphrase: general.profile_catchphrase,
                    is_donor: secure.donated.is_some() || secure.monthly_donor,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_user_by_username(&self, username: &String) -> Option<User> {
        for row_data in &self.open_secure_data_tree() {
            match row_data {
                Ok((_, r)) => {
                    let data: UserSecureData = bincode::deserialize_from(r.reader()).unwrap();
                    let id = data.id;
                    if data.username.eq(username) {
                        return Some(User { id });
                    }
                },
                Err(e) => error!("[ERROR]: {:?} for username: {}", e, username),
            }
        }
        None
    }

    fn get_user_by_email(&self, email: &String) -> Option<User> {
        for row_data in &self.open_secure_data_tree() {
            match row_data {
                Ok((_, r)) => {
                    let data: UserSecureData = bincode::deserialize_from(r.reader()).unwrap();
                    let id = data.id;
                    if data.email.eq(email) {
                        return Some(User { id });
                    }
                },
                Err(e) => error!("[ERROR]: {:?} for email: {}", e, email),
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
    verified: bool,        // Not 100% sure what this is for, perhaps email verification? Then we delete old users that haven't been verified?
    is_admin: bool,        // User or admin
    donated: Option<i64>,  // Number of cents donated in USD
    monthly_donor: bool,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    storage_used: i64,
}

const DEFAULT_USER_STORAGE_LIMIT: i64 = 10 * 1024 * 1024; // 10 MB
const DONOR_USER_STORAGE_LIMIT: i64 = 1 * 1024 * 1024 * 1024; // 1 GB
const ADMIN_USER_STORAGE_LIMIT: i64 = 5 * 1024 * 1024 * 1024; // 5 GB

impl UserSecureData {
    fn new(username: String, email: String, password: String) -> UserSecureData {
        UserSecureData {
            id: uuid::Uuid::new_v4(),
            username,
            email,
            password: bcrypt::hash(password, DEFAULT_COST).unwrap(),
            verified: false,
            is_admin: false,
            donated: None,
            monthly_donor: false,
            created_at: Some(chrono::offset::Utc::now()),
            updated_at: Some(chrono::offset::Utc::now()),
            storage_used: 0,
        }
    }

    fn update_email(&mut self, new_email: String) -> &mut Self {
        self.email = new_email;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn update_password(&mut self, new_password: String) -> &mut Self {
        self.password = bcrypt::hash(new_password, DEFAULT_COST).unwrap();
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn update_verified(&mut self, verified: bool) -> &mut Self {
        self.verified = verified;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn update_donation(&mut self, amount: i64) -> &mut Self {
        if let Some(prev) = self.donated {
            self.donated = Some(prev + amount);
        } else {
            self.donated = Some(amount);
        }
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn update_storage(&mut self, change: i64) -> &mut Self {
        self.storage_used = self.storage_used + change;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn promote(&mut self) -> &mut Self {
        self.is_admin = true;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn demote(&mut self) -> &mut Self {
        self.is_admin = false;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    /// Returns the storage limit in bytes
    fn get_storage_limit(&self) -> i64 {
        if self.is_admin {
            ADMIN_USER_STORAGE_LIMIT
        } else if self.donated.is_some() || self.monthly_donor {
            DONOR_USER_STORAGE_LIMIT
        } else {
            DEFAULT_USER_STORAGE_LIMIT
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
    profile_photo: ProfilePhotoType,       // Has default photo for new users
    profile_text: String,
    profile_catchphrase: String,
    owned_games: HashSet<uuid::Uuid>,      // Games are globally seen in the server. These are the games the user owns
    owned_rulesets: HashSet<uuid::Uuid>,   // The rulesets this user has created
    owned_settings: HashSet<uuid::Uuid>,   // The settings this user has created
    owned_characters: HashSet<uuid::Uuid>, // Characters stored in a local per-user format. These are the character the user owns
    joined_games: HashSet<uuid::Uuid>,
    favorited_rulesets: HashSet<uuid::Uuid>,
    favorited_settings: HashSet<uuid::Uuid>,
    updated_at: Option<DateTime<Utc>>,
}

// This will be good for general storing of images and their paths
#[derive(Debug, Deserialize, Serialize, Clone)]
enum ProfilePhotoType {
    ExternalPath(String),
    InternalServerPath(String)
}

impl ProfilePhotoType {
    fn to_string(self, config: &Config) -> String {
        match self {
            ProfilePhotoType::ExternalPath(path) => path,
            ProfilePhotoType::InternalServerPath(path) => format!("http://{}:{}/{}", config.server.host, config.server.port, path),
        }
    }
}

impl UserGeneralData {
    fn new(username: &str) -> UserGeneralData {
        UserGeneralData {
            profile_name: username.to_string(),
            profile_photo: ProfilePhotoType::InternalServerPath(String::from("files/default_profile.png")),
            profile_text: "Lorem Ipsum".to_string(),
            profile_catchphrase: "Best DM in the West".to_string(),
            owned_games: HashSet::new(),
            owned_rulesets: HashSet::new(),
            owned_settings: HashSet::new(),
            owned_characters: HashSet::new(),
            joined_games: HashSet::new(),
            favorited_rulesets: HashSet::new(),
            favorited_settings: HashSet::new(),
            updated_at: Some(chrono::offset::Utc::now()),
        }
    }

    fn update_profile_name(&mut self, profile_name: String) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_name = profile_name;
        self
    }

    fn add_owned_ruleset(&mut self, ruleset_id: uuid::Uuid) -> &mut Self {
        if !self.owned_rulesets.contains(&ruleset_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.owned_rulesets.insert(ruleset_id);
        }
        self
    }

    fn remove_owned_ruleset(&mut self, ruleset_id: uuid::Uuid) -> &mut Self {
        if self.owned_rulesets.contains(&ruleset_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.owned_rulesets.remove(&ruleset_id);
        }
        self
    }

    fn join_game(&mut self, game_id: uuid::Uuid) -> &mut Self {
        if !self.joined_games.contains(&game_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.joined_games.insert(game_id);
        }
        self
    }

    fn leave_game(&mut self, game_id: uuid::Uuid) -> &mut Self {
        if self.joined_games.contains(&game_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.joined_games.remove(&game_id);
        }
        self
    }
}