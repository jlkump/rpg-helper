use bcrypt::DEFAULT_COST;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

use crate::{config::Config, database::get_data};

// TODO: Add support for multi-threaded access. 
//      Sled says that multi-threaded access is managed by open_tree, so
//      maybe just have only one UserDB open by the server at a time. Should be easy enough,
//      especially if using a singular DB object to manage DB operations.
pub struct UserDB {
    users: Db,
}

// NOTE: Add deletion response in case of failure?
//       Maybe add a Result<Response, DatabaseErr>?
pub enum RegistrationResponse {
    Success(User),
    UsernameTaken,
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

    pub fn create_user(&self, registration_data: UserRegistrationSchema) -> RegistrationResponse {
        if self.get_user(&registration_data.username).is_some() {
            RegistrationResponse::UsernameTaken
        } else {
            let data_tree = self.open_data_tree();
            let id_tree = self.open_id_tree();
            let details_tree = self.open_details_tree();

            let v = UserData::default(registration_data.username, registration_data.email, registration_data.password);
            data_tree.insert(v.id, bincode::serialize(&v).unwrap()).unwrap();
            details_tree.insert(v.id, bincode::serialize(&UserDetails::new(&v.username)).unwrap()).unwrap();
            id_tree.insert(v.username, bincode::serialize(&v.id).unwrap()).unwrap();
            RegistrationResponse::Success(User {id: v.id})
        }
    }

    pub fn delete_user(&self, user: User) {
        let data_tree = self.open_data_tree();
        let id_tree = self.open_id_tree();
        let details_tree = self.open_details_tree();

        let mut remove_username = None;
        for row in &id_tree {
            let (key, bytes) = row.unwrap();
            let id: uuid::Uuid = bincode::deserialize(&bytes).unwrap();
            if user.id.eq(&id) {
                remove_username = Some(key);
                break;
            }
        }
        if let Some(key) = remove_username {
            id_tree.remove(key).unwrap();
        }
        data_tree.remove(user.id).unwrap();
        details_tree.remove(user.id).unwrap();
    }

    pub fn login_user(&self, login_data: UserLoginSchema) -> LoginResponse {
        if let Some(user_id) = self.get_user(&login_data.username) {
            let data = self.get_data(&user_id).unwrap();
            if bcrypt::verify(login_data.password, &data.password).unwrap() {
                LoginResponse::Success(user_id)
            } else {
                LoginResponse::WrongPassword
            }
        } else {
            LoginResponse::UnknownUsername
        }
    }

    fn get_user(&self, username: &String) -> Option<User> {
        let users_to_id = self.open_id_tree();
        if let Some(user_id) = get_data(&users_to_id, username) {
            Some(User { id: user_id })
        } else {
            None
        }
    }

    pub fn get_data(&self, user: &User) -> Option<UserData> {
        let tree = self.open_data_tree();
        get_data(&tree, &user.id)
    }

    pub fn get_details(&self, user: &User) -> Option<UserDetails> {
        let tree = self.open_details_tree();
        get_data(&tree, &user.id)
    }

    fn open_data_tree(&self) -> Tree {
        self.users.open_tree(b"data").unwrap()
    }

    fn open_id_tree(&self) -> Tree {
        self.users.open_tree(b"ids").unwrap()
    }

    fn open_details_tree(&self) -> Tree {
        self.users.open_tree(b"details").unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct User {
    id: uuid::Uuid
}

#[derive(Debug, Deserialize)]
pub struct UserRegistrationSchema {
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The actual password, won't be stored in DB
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub id: uuid::Uuid,        // UUID for user
    pub username: String,      // Username for the user profile
    pub email: String,         // Email of the user
    pub password: String,      // The Password hash and salt
    pub role: String,          // Admin?
    pub verified: bool,        // Not 100% sure what this is for, perhaps email verification? Then we delete old users that haven't been verified?
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserData {
    pub fn default(username: String, email: String, password: String) -> UserData {
        UserData {
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
}

// Users will a need a list of games they are participating in
// and a list of characters they own. The characters are linked to a 
// game by the game ID. They also specify the Ruleset / Setting they require. 
// Characters can be moved between games if the ruleset / setting matches.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDetails {
    pub profile_name: String,        // Starts as username, can be changed
    pub profile_photo: String,       // Has default photo for new users
    pub games: Vec<uuid::Uuid>,      // Games are globally seen in the server. These are the games the user owns
    pub rulesets: Vec<uuid::Uuid>,   // The rulesets this user has created
    pub settings: Vec<uuid::Uuid>,   // The settings this user has created
    pub characters: Vec<uuid::Uuid>, // Characters stored in a local per-user format. These are the character the user owns
}

impl UserDetails {
    fn new(username: &str) -> UserDetails {
        UserDetails {
            profile_name: username.to_string(),
            profile_photo: String::from("default_profile.png"),
            games: vec![],
            rulesets: vec![],
            settings: vec![],
            characters: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    use super::{RegistrationResponse, UserDB};
    
    #[test]
    fn create_user() {
        let db = UserDB::open(&Config::from_file("./Config.toml").unwrap());
        if let RegistrationResponse::Success(user) = db.create_user( super::UserRegistrationSchema { 
            username: String::from("JLKump1"), 
            email: String::from("landon2002@gmail.com"), 
            password: String::from("password")
        }) {
            if let Some(user_data) = db.get_data(&user) {
                assert_eq!(user_data.username, String::from("JLKump1"));
                assert_eq!(user_data.email, String::from("landon2002@gmail.com"));
                assert!(bcrypt::verify(String::from("password"), &user_data.password).unwrap());
            } else {
                panic!("Couldn't find user after creation!");
            }
        } else {
            panic!("Couldn't create user! Username already exists!");
        }
    }

    #[test]
    fn delete_user() {
        let db = UserDB::open(&Config::from_file("./Config.toml").unwrap());
        let mut user = None;
        if let Some(u) = db.get_user(&String::from("JLKump1")) {
            user = Some(u);
        } else {
            if let RegistrationResponse::Success(u) = db.create_user( super::UserRegistrationSchema { 
                username: String::from("JLKump1"), 
                email: String::from("landon2002@gmail.com"), 
                password: String::from("password")
            }) {
                user = Some(u);
            } else {
                panic!("Could not register user! Name already exists");
            }
            
        }
        if let Some (u) = user {
            db.delete_user(u);
            assert!(db.get_user(&String::from("JLKump1")).is_none());
            assert!(db.get_data(&u).is_none());
        } else {
            panic!("Couldn't find user to delete!");
        }
    }
}