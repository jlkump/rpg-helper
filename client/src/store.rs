use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::types::UserData;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct AuthUser {
    pub auth_user: Option<UserData>,
    pub auth_token: Option<String>,
}

pub fn set_auth_user(user: Option<UserData>, dispatch: Dispatch<AuthUser>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}

pub fn set_auth_token(token: Option<String>, dispatch: Dispatch<AuthUser>) {
    dispatch.reduce_mut(move |store| {
        store.auth_token = token;
    })
}