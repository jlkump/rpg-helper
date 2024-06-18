use std::{borrow::BorrowMut, collections::{HashMap, HashSet}};

use actix_web::web::Buf;
use bcrypt::DEFAULT_COST;
use chrono::prelude::*;
use log::{error, trace};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

use crate::{api::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{FriendRequest, GameInvite, PublicUserData, UserData}}, config::Config, database::get_data};

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

pub enum DataResponse<T> {
    Success(T),
    NotFound,
    DatabaseErr,
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
        // TODO: Also remove all friends and from active games
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
            let storage_limit = secure.get_storage_limit();
            let UserSecureData {
                id,
                username,
                email,
                verified,
                donated,
                monthly_donor,
                created_at,
                storage_used,
                ..
            } = secure;
            if let Some(general) = self.get_general_data(&user) {
                let UserGeneralData {
                    profile_name,
                    profile_photo,
                    profile_banner,
                    friends,
                    friend_requests,
                    sent_requests,
                    blocked_users,
                    sent_invites,
                    game_invites,
                    joined_games,
                    owned_games,
                    owned_rulesets,
                    owned_settings,
                    owned_characters,
                    favorited_rulesets,
                    favorited_settings,
                    last_read_news,
                    ..
                } = general;
                Some(UserData {
                    id,
                    username,
                    email,
                    created_at,
                    verified,
                    profile_name,
                    profile_photo: profile_photo.to_string(&config),
                    profile_banner: profile_banner.to_string(&config),
                    favorited_rulesets,
                    favorited_settings,
                    sent_invites: sent_invites.into_values().collect(),
                    joined_games,
                    owned_games,
                    owned_rulesets,
                    owned_settings,
                    owned_characters,
                    storage_used,
                    storage_limit,
                    is_donor: donated.is_some() || monthly_donor,
                    friends,
                    friend_requests: friend_requests.into_values().collect(),
                    sent_requests,
                    blocked_users,
                    game_invites,
                    last_read_news,
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
            let UserSecureData {
                id,
                username,
                donated,
                monthly_donor,
                created_at,
                ..
            } = secure;
            if let Some(general) = self.get_general_data(&user) {
                let UserGeneralData {
                    profile_name,
                    profile_photo,
                    profile_banner,
                    profile_text,
                    profile_catchphrase,
                    ..
                } = general;
                Some(PublicUserData {
                    username,
                    created_at,
                    profile_name,
                    profile_photo: profile_photo.to_string(&config),
                    profile_banner: profile_banner.to_string(&config),
                    profile_text,
                    profile_catchphrase,
                    is_donor: donated.is_some() || monthly_donor,
                    id,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_user_by_username(&self, username: &String) -> Option<User> {
        // TODO: have sub-tree in database that maps <Username, ID>, makes things faster
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
    profile_name: String,                  // Starts as username, can be changed
    profile_photo: ImageUrl,               // Has default photo for new users
    profile_banner: ImageUrl,              // Has default photo for new users
    profile_text: String,
    profile_catchphrase: String,
    friends: HashSet<uuid::Uuid>,          // Set of friends for the user
    friend_requests: HashMap<uuid::Uuid, FriendRequest>,  // Friend requests sent to this user
    sent_requests: HashSet<uuid::Uuid>,    // Friend requests sent by this user
    blocked_users: HashSet<uuid::Uuid>,    // Set of blocked users for the user
    sent_invites: HashMap<uuid::Uuid, GameInvite>,     // Set of users this user has invited to play
    game_invites: HashSet<uuid::Uuid>,     // Set of games this user is invited to play in
    joined_games: HashSet<uuid::Uuid>,
    owned_games: HashSet<uuid::Uuid>,      // Games are globally seen as long as they are public. Otherwise only friends
    owned_rulesets: HashSet<uuid::Uuid>,   // The rulesets this user has created. These will last even if the user is deleted as long as it is followed by at least one user that is not deleted.
    owned_settings: HashSet<uuid::Uuid>,   // The settings this user has created. These will last even if the user is deleted as long as it is followed by at least one user that is not deleted.
    owned_characters: HashSet<uuid::Uuid>,
    favorited_rulesets: HashSet<uuid::Uuid>,
    favorited_settings: HashSet<uuid::Uuid>,
    updated_at: Option<DateTime<Utc>>,
    last_read_news: Option<DateTime<Utc>>, // Keeps track of if there is news that the user has not seen on the website. Will only display on the dashboard, as not to be annoying.
}

// This will be good for general storing of images and their paths
#[derive(Debug, Deserialize, Serialize, Clone)]
enum ImageUrl {
    ExternalPath(String),
    InternalServerPath(String)
}

impl ImageUrl {
    fn to_string(self, config: &Config) -> String {
        match self {
            ImageUrl::ExternalPath(path) => path,
            ImageUrl::InternalServerPath(path) => format!("http://{}:{}/{}", config.server.host, config.server.port, path),
        }
    }
}

impl UserGeneralData {
    fn new(username: &str) -> UserGeneralData {
        UserGeneralData {
            profile_name: username.to_string(),
            profile_photo: ImageUrl::InternalServerPath(String::from("files/default_profile.png")),
            profile_banner: ImageUrl::InternalServerPath(String::from("files/default_banner.png")),
            profile_text: "Lorem Ipsum".to_string(),
            profile_catchphrase: "Best DM in the West".to_string(),
            friends: HashSet::new(),
            friend_requests: HashMap::new(),
            sent_requests: HashSet::new(),
            blocked_users: HashSet::new(),
            sent_invites: HashMap::new(),
            game_invites: HashSet::new(),
            owned_games: HashSet::new(),
            owned_rulesets: HashSet::new(),
            owned_settings: HashSet::new(),
            owned_characters: HashSet::new(),
            joined_games: HashSet::new(),
            favorited_rulesets: HashSet::new(),
            favorited_settings: HashSet::new(),
            updated_at: Some(chrono::offset::Utc::now()),
            last_read_news: None,
        }
    }

    fn update_profile_name(&mut self, profile_name: String) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_name = profile_name;
        self
    }

    fn update_profile_text(&mut self, text: String) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_text = text;
        self
    }

    fn update_profile_photo(&mut self, photo_url: ImageUrl) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_photo = photo_url;
        self
    }

    fn update_banner_photo(&mut self, photo_url: ImageUrl) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_banner = photo_url;
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

    fn block_user(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if !self.blocked_users.contains(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.blocked_users.insert(user_id);
        }
        self
    }

    fn unblock_user(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if self.blocked_users.contains(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.blocked_users.remove(&user_id);
        }
        self
    }

    fn accept_friend_request(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if !self.friends.contains(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.friends.insert(user_id);
            self.friend_requests.remove(&user_id);
        }
        self
    }

    fn reject_friend_request(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if self.friend_requests.contains_key(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.friend_requests.remove(&user_id);
        }
        self
    }

    fn add_friend(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if !self.friends.contains(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.friends.insert(user_id);
        }
        self
    }

    fn remove_friend(&mut self, user_id: uuid::Uuid) -> &mut Self {
        if self.friends.contains(&user_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.friends.remove(&user_id);
        }
        self
    }

    fn accept_game_invite(&mut self, game_id: uuid::Uuid) -> &mut Self {
        if !self.joined_games.contains(&game_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.joined_games.insert(game_id);
            self.game_invites.remove(&game_id);
        }
        self
    }

    fn reject_game_invite(&mut self, game_id: uuid::Uuid) -> &mut Self {
        if self.game_invites.contains(&game_id) {
            self.updated_at = Some(chrono::offset::Utc::now());
            self.game_invites.remove(&game_id);
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