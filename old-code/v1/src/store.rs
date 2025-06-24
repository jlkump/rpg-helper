use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::{error::Error, model::types::UserData};

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

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct Errors {
    pub errors: Vec<Error>,
}

pub fn push_error(error: Error, dispatch: Dispatch<Errors>) {
    dispatch.reduce_mut(move |s| {
        s.errors.push(error)
    })
}

pub fn remove_error(error: &Error, dispatch: Dispatch<Errors>) {
    dispatch.reduce_mut(move |s| {
        if let Some(i) = s.errors.iter().position(|e| e.eq(error)) {
            s.errors.remove(i);
        }
    })
}