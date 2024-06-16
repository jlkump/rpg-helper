use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::types::UserData;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct GlobalStore {
    pub auth_user: Option<UserData>,
}

pub fn set_auth_user(user: Option<UserData>, dispatch: Dispatch<GlobalStore>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}