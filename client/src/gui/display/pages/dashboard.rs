use gloo::console::error;
use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{api::user_api::api_user_info, gui::display::organisms::nav_bar::NavBar, router, store::{set_auth_user, GlobalStore}};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {

}

#[styled_component(Dashboard)]
pub fn dashboard(props: &Props) -> Html {
    let (store, dispatch) = use_store::<GlobalStore>();
    let page_loading = use_state(|| false);
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    use_effect_with((),
        move |_| {
            let dispatch = dispatch.clone();
            let page_loading = page_loading.clone();
            spawn_local(async move {
                page_loading.set(true);
                let response = api_user_info().await;

                match response {
                    Ok(user) => {
                        set_auth_user(Some(user), dispatch);
                        page_loading.set(false);
                    }
                    Err(e) => {
                        page_loading.set(false);
                        match e {
                            crate::api::user_api::Error::Standard(data_err) => {
                                match data_err {
                                    crate::api::types::UserDataError::UserNotFound(_) => {
                                        navigator.push(&router::Route::Login);
                                    },
                                }
                            },
                            crate::api::user_api::Error::API => {
                                error!("User Data Error: API Error");
                                navigator.push(&router::Route::Login);
                            },
                            crate::api::user_api::Error::RequestFailed => {
                                error!("User Data Error: Request Failed Error");
                            },
                            crate::api::user_api::Error::ParseFailed => {
                                error!("User Data Error: Parse Failed Error");
                            },
                        }
                    }
                }
            });
        },
    );

    html! {
        <NavBar>
            <div>
                {"TODO"}
            </div>
        </NavBar>
    }
}