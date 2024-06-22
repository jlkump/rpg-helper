use std::{borrow::BorrowMut, collections::{HashMap, HashSet}};

use actix_web::web::Buf;
use bcrypt::DEFAULT_COST;
use chrono::prelude::*;
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

use crate::{api::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{FriendRequest, GameInvite, ImageUrl, PublicUserData, UserData}}, config::Config, database::get_data};

use super::{Error, UpdateResponse, User};

pub struct UserDB {
    users: Db,
}

#[derive(Debug)]
pub enum RegistrationConflict {
    UsernameTaken,
    EmailTaken,
    Other
}

pub enum LoginResponse {
    Success(User),
    UnknownUsernameOrPassword
}

impl From<Error> for Error<RegistrationConflict> {
    fn from(value: Error) -> Self {
        match value {
            Error::DbConflict(e) => Error::DbConflict(RegistrationConflict::Other),
            Error::DbErr(e) => Error::DbErr(e),
            Error::ParseErr(e) => Error::ParseErr(e),
            Error::Bcrypt(e) => Error::Bcrypt(e),
            Error::Other(str) => Error::Other(str),
        }
    }
}

impl UserDB {
    pub(super) fn open(config: &Config) -> Result<Self, sled::Error> {
        Ok(UserDB {
            users: sled::open(format!("{}/users", config.database.database_path))?
        })
    }

    pub fn register_user(&self, registration_data: UserRegistrationSchema) -> Result<User, Error<RegistrationConflict>> {
        if self.get_user_by_username(&registration_data.username)?.is_some() {
            info!("Username for registration was taken");
            return Err(Error::DbConflict(RegistrationConflict::UsernameTaken));
        } else if self.get_user_by_email(&registration_data.email)?.is_some() {
            info!("email for registration was taken");
            return Err(Error::DbConflict(RegistrationConflict::EmailTaken));
        } else {

            let v = UserSecureData::new(registration_data.username, registration_data.email, registration_data.password);
            self.open_username_to_id()?.insert(v.username.clone(), bincode::serialize(&v.id)?)?;
            self.open_email_to_id()?.insert(v.email.clone(), bincode::serialize(&v.id)?)?;
            self.open_secure_data_tree()?.insert(v.id, bincode::serialize(&v)?)?;
            self.open_general_data_tree()?.insert(v.id, bincode::serialize(&UserGeneralData::new(&v.username))?)?;
            return Ok(v.id);
        }
    }

    pub fn delete_user(&self, user: User) -> Result<(), Error> {
        let data_tree = self.open_secure_data_tree()?;
        // TODO: Also remove all friends and from active games (remove from active games will have to be at the higher database level)
        if let Some(user_data) = get_data::<UserSecureData, uuid::Uuid>(&data_tree, &user)? {
            // TODO: Look at friends, remove from friends lists for other users. Do the same for games.
            self.open_username_to_id()?.remove(user_data.username)?;
            self.open_email_to_id()?.remove(user_data.email)?;
            self.open_general_data_tree()?.remove(user)?;
            data_tree.remove(user)?;
        }
        Ok(())
    }

    pub fn login_user(&self, login_data: UserLoginSchema) -> Result<LoginResponse, Error> {
        if let Some(user_id) = self.get_user_by_username(&login_data.username)? {
            if let Some(data) = self.get_secure_data(&user_id)? {
                if bcrypt::verify(login_data.password, &data.password)? {
                    return Ok(LoginResponse::Success(user_id));
                }
            }
        }
        Ok(LoginResponse::UnknownUsernameOrPassword)
    }

    pub fn user_exists(&self, user: User) -> Result<bool, Error> {
        Ok(self.get_secure_data(&user)?.is_some())
    }

    pub fn update_email(&self, user: User, new_email: String) -> Result<UpdateResponse, Error> {
        self.generic_secure_update(user, new_email, UserSecureData::update_email)
    }

    pub fn update_password(&self, user: User, new_password: String) -> Result<UpdateResponse, Error> {
        self.generic_secure_update(user, new_password, UserSecureData::update_password)
    }

    pub fn update_verified(&self, user: User, verified: bool) -> Result<UpdateResponse, Error> {
        self.generic_secure_update(user, verified, UserSecureData::update_verified)
    }

    pub fn update_storage_usage(&self, user: User, new_amount: i64) -> Result<UpdateResponse, Error> {
        self.generic_secure_update(user, new_amount, UserSecureData::update_storage)
    }

    pub fn update_donation_amount(&self, user: User, new_amount: i64) -> Result<UpdateResponse, Error> {
        self.generic_secure_update(user, new_amount, UserSecureData::update_donation)
    }

    pub fn update_profile_name(&self, user: User, profile_name: String) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, profile_name, UserGeneralData::update_profile_name)
    }

    pub fn update_profile_catchphrase(&self, user: User, profile_catchphrase: String) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, profile_catchphrase, UserGeneralData::update_profile_catchphrase)
    }

    pub fn update_profile_text(&self, user: User, profile_text: String) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, profile_text, UserGeneralData::update_profile_text)
    }

    pub fn update_profile_photo(&self, user: User, profile_photo: ImageUrl) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, profile_photo, UserGeneralData::update_profile_photo)
    }

    pub fn update_profile_banner(&self, user: User, profile_banner: ImageUrl) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, profile_banner, UserGeneralData::update_profile_banner)
    }

    pub(super) fn join_game(&self, user: User, game_id: uuid::Uuid) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, game_id, UserGeneralData::join_game)
    }

    pub(super) fn leave_game(&self, user: User, game_id: uuid::Uuid) -> Result<UpdateResponse, Error> {
        self.generic_general_update(user, game_id, UserGeneralData::leave_game)
    }

    pub fn get_private_data(&self, user: User, config: &Config) -> Result<Option<UserData>, Error> {
        if let Some(secure) = self.get_secure_data(&user)? {
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
            if let Some(general) = self.get_general_data(&user)? {
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
                return Ok(Some(UserData {
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
                    is_donor: donated != 0 || monthly_donor,
                    friends,
                    friend_requests: friend_requests.into_values().collect(),
                    sent_requests,
                    blocked_users,
                    game_invites,
                    last_read_news,
                }));
            }
        }
        return Ok(None);
    }

    pub fn get_public_data(&self, user: User, config: &Config) -> Result<Option<PublicUserData>, Error> {
        if let Some(secure) = self.get_secure_data(&user)? {
            let UserSecureData {
                id,
                username,
                donated,
                monthly_donor,
                created_at,
                ..
            } = secure;
            if let Some(general) = self.get_general_data(&user)? {
                let UserGeneralData {
                    profile_name,
                    profile_photo,
                    profile_banner,
                    profile_text,
                    profile_catchphrase,
                    ..
                } = general;
                return Ok(Some(PublicUserData {
                    username,
                    created_at,
                    profile_name,
                    profile_photo: profile_photo.to_string(&config),
                    profile_banner: profile_banner.to_string(&config),
                    profile_text,
                    profile_catchphrase,
                    is_donor: donated != 0 || monthly_donor,
                    id,
                }));
            }
        }
        return Ok(None);
    }


    ////////////////////////////////////////////////////
    ////////////// Helper Methods //////////////////////
    ////////////////////////////////////////////////////


    fn generic_secure_update<T, F>(&self, user: User, data: T, update: F) -> Result<UpdateResponse, Error>
    where 
        F: FnOnce(&mut UserSecureData, T) -> &mut UserSecureData 
    {
        let tree = self.open_secure_data_tree()?;
        if let Some(mut secure) = get_data::<UserSecureData, User>(&tree, &user)? {
            update(&mut secure, data);
            tree.insert(user, bincode::serialize(&secure)?)?;
            Ok(UpdateResponse::Success)
        } else {
            Ok(UpdateResponse::NotFound)
        }
    }

    fn generic_general_update<T, F>(&self, user: User, data: T, update: F) -> Result<UpdateResponse, Error>
    where 
        F: FnOnce(&mut UserGeneralData, T) -> &mut UserGeneralData 
    {
        let tree = self.open_general_data_tree()?;
        if let Some(mut general) = get_data::<UserGeneralData, User>(&tree, &user)? {
            update(&mut general, data);
            tree.insert(user, bincode::serialize(&general)?)?;
            Ok(UpdateResponse::Success)
        } else {
            Ok(UpdateResponse::NotFound)
        }
    }

    fn get_user_by_username(&self, username: &String) -> Result<Option<User>, Error> {
        get_data(&self.open_username_to_id()?, username)
    }

    fn get_user_by_email(&self, email: &String) -> Result<Option<User>, Error> {
        get_data(&self.open_email_to_id()?, email)
    }

    fn get_secure_data(&self, user: &User) -> Result<Option<UserSecureData>, Error> {
        get_data(&self.open_secure_data_tree()?, &user)
    }

    fn get_general_data(&self, user: &User) -> Result<Option<UserGeneralData>, Error> {
        get_data(&self.open_general_data_tree()?, &user)
    }

    fn open_secure_data_tree(&self) -> Result<Tree, sled::Error> {
        self.users.open_tree(b"secure")
    }

    fn open_general_data_tree(&self) -> Result<Tree, sled::Error> {
        self.users.open_tree(b"general")
    }

    fn open_username_to_id(&self) -> Result<Tree, sled::Error> {
        self.users.open_tree(b"username-to-id")
    }

    fn open_email_to_id(&self) -> Result<Tree, sled::Error> {
        self.users.open_tree(b"email-to-id")
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
    donated: i64,  // Number of cents donated in USD
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
            donated: 0,
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

    fn update_donation(&mut self, new_amount: i64) -> &mut Self {
        self.donated = new_amount;
        self.updated_at = Some(chrono::offset::Utc::now());
        self
    }

    fn update_storage(&mut self, new_amount: i64) -> &mut Self {
        self.storage_used = new_amount;
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
        } else if self.donated != 0 || self.monthly_donor {
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

    fn update_profile_catchphrase(&mut self, catchphrase: String) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_catchphrase = catchphrase;
        self
    }

    fn update_profile_photo(&mut self, photo_url: ImageUrl) -> &mut Self {
        self.updated_at = Some(chrono::offset::Utc::now());
        self.profile_photo = photo_url;
        self
    }

    fn update_profile_banner(&mut self, photo_url: ImageUrl) -> &mut Self {
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