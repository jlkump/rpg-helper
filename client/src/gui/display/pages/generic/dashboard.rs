use gloo::console::error;
use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{api::user_api::api_user_info, gui::{contexts::theme::use_theme, display::{atoms::loading::Loader, organisms::nav_bar::NavBar}}, router, store::{set_auth_user, AuthUser}};

#[derive(Clone, Properties, PartialEq)]
pub struct Props;

#[styled_component(Dashboard)]
pub fn dashboard(_: &Props) -> Html {
    let (store, dispatch) = use_store::<AuthUser>();
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
                        navigator.push(&e.route_based_on_err());
                    }
                }
            });
        }
    );

    let theme = use_theme();

    html! {
        <NavBar content_class={css!("display: flex; align-items: center; justify-content: center;")}>
            if let Some(user) = user {
                <div>
                    <h3>{format!("Welcome {}!", user.profile_name)}</h3>
                </div>
            } else {
                <div>
                    <Loader color={theme.text_colored.clone()} size="5em" />
                </div>
            }
        </NavBar>
    }
}