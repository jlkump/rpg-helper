use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Store)]
pub struct AuthUser
{
    // TODO:
    // pub auth_user: Option<UserData>,
    pub auth_token: Option<String>,
}