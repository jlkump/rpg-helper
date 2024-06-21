use stylist::yew::styled_component;
use yew::{platform::spawn_local, prelude::*};
use yew_icons::{Icon, IconId};

use crate::{api::{types::PublicUserData, user_api::api_public_user_info}, gui::{contexts::theme::use_theme, display::atoms::{loading::{SkeletonTextArea, SkeletonPane}, profile::ProfilePortrait}}};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub user: uuid::Uuid,
    #[prop_or(false)]
    pub edit: bool,
}

#[styled_component(ProfileCard)]
pub fn profile_card(props: &Props) -> Html {
    let profile_data = use_state(|| None);
    let loading = use_state(|| false);
    let err_str = use_state(|| "".to_string());

    let loading_cloned = loading.clone();
    let props_cloned = props.clone();
    let profile_data_cloned = profile_data.clone();
    use_effect_with((), move |_| {
        spawn_local(async move {
            loading_cloned.set(true);
            let response = api_public_user_info(props_cloned.user).await;

            match response {
                Ok(data) => {
                    loading_cloned.set(false);
                    profile_data_cloned.set(Some(data))
                },
                Err(e) => {
                    loading_cloned.set(false);
                    err_str.set(e.to_string());
                }
            }

        });
    });

    let theme = use_theme();
    let style = css!(
        r#"
            background: ${bg};
            border: 5px solid ${border};
            border-radius: 0px;
            min-width: 250px;
            min-height: 350px;
            padding-bottom: 20px;
            box-shadow: 10px 0px 10px ${shadow}, -10px 0px 10px ${shadow};

            .profile-content {
                padding-top: 20px;
                position: relative;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: flex-start;
            }

            .profile-banner {
                width: 100%;
                height: 100px;
                margin: 0px;
            }

            .profile {
                position: absolute;
                top: 0;
                transform: translate(0, -80%);
            }

            .profile-banner.loading {
                background: ${banner_loading};
            }

            .text-area {
                min-width: 230px;
                text-align: center;
            }

            & > div {
                margin-top: 10px;
            }
        "#,
        bg = theme.paper, 
        border = theme.border_tooltip_light, 
        shadow = theme.hover_dropshadow,
        banner_loading = theme.panel_secondary
    );

    let is_friend = use_state(|| false);
    let sent_friend = use_state(|| false);

    let default = PublicUserData::default();
    let profile_data = profile_data.as_ref().unwrap_or(&default);
    let loading_cloned = loading.clone();
    let sent_friend_cloned = sent_friend.clone();
    let is_friend_cloned = is_friend.clone();
    html! {
        <div class={style}>
            <div class={if *loading { 
                    classes!("profile-banner", "loading")
                } else { 
                    classes!("profile-banner", css!("background-image: url(\"${bg}\"); background-size: contain;", bg=profile_data.profile_banner.clone()))
            }}>
            </div>
            <div class="profile-content">
                <div class="profile">
                    <ProfilePortrait width="5em" height="5em" loading={*loading} src={profile_data.profile_photo.clone()}/>
                </div>
                <h1 style="margin: 5px;">
                    if *loading {
                        <SkeletonPane style="width: 180px; height: 1em; margin: 5px;"/> 
                    } else {
                        {profile_data.profile_name.clone()}
                    }
                <hr/></h1>
                <h2 style="font-style: italic; font-size: 1em;">
                    if *loading {
                        <SkeletonPane style=" width: 120px; height: 1em; margin: 2px;"/>
                    } else {
                        {profile_data.profile_catchphrase.clone()}
                    }
                </h2>
                <div class={"text-area"}>
                    if *loading {
                        <SkeletonTextArea />
                    } else {
                        {profile_data.profile_text.clone()}
                    }
                </div>
                <div style="display: flex; width: 100%; justify-content: space-evenly;">
                    if *loading {
                        <SkeletonPane style=" width: 85px; height: 2.5em; margin: 5px;"/>
                        <SkeletonPane style=" width: 85px; height: 2.5em; margin: 5px;"/>
                    } else {
                        // TODO: Check active auth user and see if this profile is in friends. 
                        // If it is, have option to un-friend. If not, have option to friend.
                        // Also will require Inbox / Notfication system per user
                        if *is_friend {
                            <button style="width: 85px; height: 2.5em;" onclick={Callback::from(move |_| {sent_friend_cloned.set(true);})} disabled={*sent_friend}>
                                if *sent_friend {
                                    <Icon icon_id={IconId::BootstrapCheckAll} width={"1.5em".to_owned()} height={"1.5em".to_owned()}/>
                                } else {
                                    <Icon icon_id={IconId::BootstrapPersonPlus} width={"1.5em".to_owned()} height={"1.5em".to_owned()}/>
                                }
                            </button>
                            <button style="width: 85px; height: 2.5em;">
                                <Icon icon_id={IconId::LucideCircleSlashed} width={"1.5em".to_owned()} height={"1.5em".to_owned()}/>
                            </button>
                        } else {
                            <button style="width: 85px; height: 2.5em;">
                                <Icon icon_id={IconId::BootstrapPersonDashFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()}/>
                            </button>
                            <button style="width: 85px; height: 2.5em;">
                                <Icon icon_id={IconId::LucideCircleSlashed} width={"1.5em".to_owned()} height={"1.5em".to_owned()}/>
                            </button>
                        }

                    }
                </div>
                <button onclick={Callback::from(move |_| { loading_cloned.set(!*loading_cloned); })}>{"Swap Loading"}</button>
                <button onclick={Callback::from(move |_| { is_friend_cloned.set(!*is_friend_cloned); })}>{"Swap Friend"}</button>
            </div>
        </div>
    }
}