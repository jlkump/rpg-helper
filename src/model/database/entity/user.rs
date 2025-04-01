use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{core::Error, database::Database};

use super::EntityID;

pub type UserID = EntityID;

pub const ROOT_ID: UserID = uuid::uuid!("11111111-1111-1111-1111-111111111111");

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct User
{
    pub secure: Option<UserSecureData>,     // None when sent to client without permission to view
    pub private: Option<UserPrivateData>,   // None when sent to client without permission to view
    pub public: Option<UserPublicData>,     // None when sent to client without permission to view
    pub created_at: Option<DateTime<Utc>>, 
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct UserSecureData {
    pub id: UserID,                         // UUID for user
    pub username: String,                   // Username for the user profile
    pub email: String,                      // Email of the user
    pub password: String,                   // The Password hash and salt
    pub is_admin: bool,                     // User or admin
    pub verified: bool,                     // Email Verification. We delete old users that aren't active for 2 years and have not been verified
    pub donated: i64,                       // Number of cents donated in USD
    pub monthly_donor: bool,                // If the user is currently an active monthly donor
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct UserPrivateData
{
    pub storage_used: i64,                  // Measured in bytes
    pub friends: HashSet<UserID>,           // Set of friends for the user
    pub blocked_users: HashSet<UserID>,     // Set of blocked users for the user

    // pub friend_requests: HashMap<uuid::Uuid, FriendRequest>,  // Friend requests sent to this user
    // pub sent_requests: HashSet<uuid::Uuid>,    // Friend requests sent by this user
    // pub sent_invites: HashMap<uuid::Uuid, GameInvite>,     // Set of users this user has invited to play
    // pub game_invites: HashSet<uuid::Uuid>,     // Set of games this user is invited to play in
    // pub joined_games: HashSet<uuid::Uuid>,
    // pub owned_games: HashSet<uuid::Uuid>,      // Games are globally seen as long as they are public. Otherwise only friends
    // pub owned_rulesets: HashSet<uuid::Uuid>,   // The rulesets this user has created. These will last even if the user is deleted as long as it is followed by at least one user that is not deleted.
    // pub owned_settings: HashSet<uuid::Uuid>,   // The settings this user has created. These will last even if the user is deleted as long as it is followed by at least one user that is not deleted.
    // pub owned_characters: HashSet<uuid::Uuid>,
    // pub favorited_rulesets: HashSet<uuid::Uuid>,
    // pub favorited_settings: HashSet<uuid::Uuid>,
    // pub last_read_news: Option<DateTime<Utc>>, // Keeps track of if there is news that the user has not seen on the website. Will only display on the dashboard, as not to be annoying.
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct UserPublicData {
    pub profile_name: String,               // Starts as username, can be changed
    // pub profile_photo: ImageUrl,               // Has default photo for new users
    // pub profile_banner: ImageUrl,              // Has default photo for new users
    pub profile_text: String,
    pub profile_catchphrase: String,
    pub showcase: Option<EntityID>,         // The user can showcase a character, game, ruleset, or setting they own
}

impl User
{
    pub fn create_user<T>(db: &mut T, username: String, email: String, password: String) -> Result<(), Error>
        where T: Database
    {
        let s = UserSecureData
        {
            id: uuid::Uuid::new_v4(),
            username: username.clone(),
            email,
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
            verified: false,
            is_admin: false,
            donated: 0,
            monthly_donor: false,
        };

        let p = UserPrivateData
        {
            storage_used: 0,
            friends: HashSet::new(),
            blocked_users: HashSet::new(),
        };

        let pu = UserPublicData
        {
            profile_name: username,
            profile_text: "Default Text".to_owned(),
            profile_catchphrase: "Default Catchphrase".to_owned(),
            showcase: None,
        };

        let u = User
        {
            secure: Some(s),
            private: Some(p),
            public: Some(pu),
            created_at: Some(chrono::offset::Utc::now()),
            updated_at: Some(chrono::offset::Utc::now()),
        };

        Ok(db.insert_entity(super::Entity::User(u))?)
    }
}