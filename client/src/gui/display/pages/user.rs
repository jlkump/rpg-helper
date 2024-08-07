use std::{borrow::{Borrow, Cow}, cell::RefCell, ops::Deref, rc::Rc};

use gloo::console::log;
use html::IntoPropValue;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use stylist::{css, yew::styled_component};
use yew_router::{components::Link, hooks::use_navigator, navigator::{self, Navigator}};
use yewdux::{dispatch, use_store, Dispatch};

use crate::{api::user_api::{api_login_user, api_public_user_info, api_register_user, api_user_info, api_user_upload}, error::Error, gui::{contexts::theme::use_theme, display::{atoms::{button::SubmitButton, form_input::FormInput, loading::{SkeletonPane, SkeletonTextArea}, profile::ProfilePortrait, scroll_div::ScrollDiv}, molecules::profile_card::ProfileCard, organisms::nav_bar::NavBar}}, model::{schema::{UserLoginSchema, UserRegistrationSchema}, types::PublicUserData}, router::Route, store::{set_auth_token, set_auth_user, AuthUser}};
use validator::{Validate, ValidationError, ValidationErrors};


#[derive(Properties, Clone, PartialEq)]
pub struct RegisterProps;

#[derive(Validate, Default, Clone, PartialEq)]
struct RegistrationFormData {
    #[validate(length(min = 1, message = "Username is required"))]
    username: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    password_confirm: String,
}

impl From<RegistrationFormData> for UserRegistrationSchema {
    fn from(value: RegistrationFormData) -> Self {
        UserRegistrationSchema {
            username: value.username,
            email: value.email,
            password: value.password,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum RegistrationFormUpdate {
    Username(String),
    Email(String),
    Password(String),
    PasswordConfirm(String),
}

fn registration_input_callback(form: UseStateHandle<RegistrationFormData>) -> Callback<RegistrationFormUpdate> {
    Callback::from(move |data: RegistrationFormUpdate| {
        let mut prev = form.deref().clone();
        match data {
            RegistrationFormUpdate::Username(s) => prev.username = s,
            RegistrationFormUpdate::Email(s) => prev.email = s,
            RegistrationFormUpdate::Password(s) => prev.password = s,
            RegistrationFormUpdate::PasswordConfirm(s) => prev.password_confirm = s,
        }
        form.set(prev);
    })
}

fn registration_blur_callback(form: UseStateHandle<RegistrationFormData>, vald_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>) -> Callback<(String, RegistrationFormUpdate)> {
    Callback::from(move |(name, value): (String, RegistrationFormUpdate)| {
        let mut data = form.deref().clone();
        match value {
            RegistrationFormUpdate::Email(s) => data.email = s,
            RegistrationFormUpdate::Password(s) => data.password = s,
            _ => (),
        }
        form.set(data);

        match form.validate() {
            Ok(_) => {
                vald_errors
                    .borrow_mut()
                    .errors_mut()
                    .remove(name.as_str());
            }
            Err(errors) => {
                vald_errors
                    .borrow_mut()
                    .errors_mut()
                    .retain(|key, _| key != &name);
                for (field_name, error) in errors.errors() {
                    if field_name == &name {
                        vald_errors
                            .borrow_mut()
                            .errors_mut()
                            .insert(field_name, error.clone());
                    }
                }
            }
        }
    })
}

fn registration_onsubmit_callback(
    form: UseStateHandle<RegistrationFormData>, 
    vald_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>,
    navigator: Navigator,
    loading: UseStateHandle<bool>,
    username_ref: NodeRef,
    email_ref: NodeRef,
    password_ref: NodeRef,
    password_confirm_ref: NodeRef,
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        let form = form.clone();
        let vald_errors = vald_errors.clone();
        let navigator = navigator.clone();
        let loading = loading.clone();
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let password_confirm_ref = password_confirm_ref.clone();

        event.prevent_default();
        spawn_local(async move {
            match form.validate() {
                Ok(_) => {
                    let form_data = form.deref().clone();
                    loading.set(true);
                    let res = api_register_user(&form_data.into()).await;
                    
                    username_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    email_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    password_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    password_confirm_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));

                    match res {
                        Ok(_) => {
                            loading.set(false);
                            navigator.push(&Route::Login);
                        },
                        Err(e) => {
                            loading.set(false);
                            match &e {
                                Error::Server(se) => match &se.error {
                                    crate::model::types::ServerErrorType::Conflict(c) => {
                                        match &c {
                                            crate::model::types::ConflictError::Username => {
                                                let err = ValidationError::new("UsernameTaken").with_message(Cow::from("Username is taken"));
                                                let key = "username";
                                                vald_errors
                                                    .borrow_mut()
                                                    .errors_mut()
                                                    .insert(key, validator::ValidationErrorsKind::Field(vec![err]));
                                            },
                                            crate::model::types::ConflictError::Email => {
                                                let err = ValidationError::new("EmailTaken").with_message(Cow::from("Email is taken"));
                                                let key = "email";
                                                vald_errors
                                                    .borrow_mut()
                                                    .errors_mut()
                                                    .insert(key, validator::ValidationErrorsKind::Field(vec![err]));
                                            },
                                            _ => navigator.push(&e.route_based_on_err()),
                                        }
                                    },
                                    _ => navigator.push(&e.route_based_on_err()),
                                },
                                _ => navigator.push(&e.route_based_on_err()),
                            }
                        },
                    }
                },
                Err(e) => {
                    vald_errors.set(Rc::new(RefCell::new(e)));
                },
            }
        });
    })
}

#[styled_component(RegisterUser)]
pub fn register_user(_: &RegisterProps) -> Html {
    // Display changes based on whether logged-in or not
    // Based on tutorial here: https://codevoweb.com/rust-yew-frontend-jwt-access-and-refresh-tokens/
    let loading = use_state(|| false);
    let form = use_state(|| RegistrationFormData::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let username_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let password_confirm_input_ref = NodeRef::default();

    let onchange = registration_input_callback(form.clone());
    let onblur_validate = registration_blur_callback(form.clone(), validation_errors.clone());
    let on_submit = registration_onsubmit_callback(form.clone(), validation_errors.clone(), navigator.clone(), loading.clone(), username_input_ref.clone(), email_input_ref.clone(), password_input_ref.clone(), password_confirm_input_ref.clone());


    html! {
        <NavBar content_class={css!("display: flex; justify-content: center; align-items: center;")}>
            <ScrollDiv class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} style="padding: 20px;">
                <h1 class={css!("font-size: 2em;")}>{"Sign Up"}</h1>
                <form class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} onsubmit={on_submit}>
                    <FormInput<RegistrationFormUpdate> 
                        input_type="text" placeholder="Username"  name="username" input_ref={username_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Username(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="text" placeholder="Email"  name="email" input_ref={email_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Email(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="password" placeholder="Password"  name="password" input_ref={password_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Password(s))}
                        autocomplete={Some("new-password".to_string())}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="password" placeholder="Confirm Password"  name="password_confirm" input_ref={password_confirm_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::PasswordConfirm(s))}
                        autocomplete={Some("new-password".to_string())}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <SubmitButton loading={*loading}> {"Submit"} </SubmitButton>
                </form>

                <div class={css!("margin-top: 14px; display: flex; flex-direction: column; justify-content: center; align-items: center; font-size: 1em;")}>
                    {"Already have an account?"}
                    <Link<Route> to={Route::Login}>
                        {" Login Here"}
                    </Link<Route>>
                </div>
            </ScrollDiv>
        </NavBar>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoginProps;

#[derive(Validate, Default, Clone, PartialEq)]
struct LoginFormData {
    #[validate(length(min = 1, message = "Username is required"))]
    username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
}

impl From<LoginFormData> for UserLoginSchema {
    fn from(value: LoginFormData) -> Self {
        UserLoginSchema {
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum LoginFormUpdate {
    Username(String),
    Password(String)
}

fn login_input_callback(form: UseStateHandle<LoginFormData>) -> Callback<LoginFormUpdate> {
    Callback::from(move |data: LoginFormUpdate| {
        let mut prev = form.deref().clone();
        match data {
            LoginFormUpdate::Username(s) => prev.username = s,
            LoginFormUpdate::Password(s) => prev.password = s,
        }
        form.set(prev);
    })
}

fn login_blur_callback(form: UseStateHandle<LoginFormData>, vald_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>) -> Callback<(String, LoginFormUpdate)> {
    Callback::from(move |(name, value): (String, LoginFormUpdate)| {
        let mut data = form.deref().clone();
        match value {
            LoginFormUpdate::Username(s) => data.username = s,
            LoginFormUpdate::Password(s) => data.password = s,
        }
        form.set(data);

        match form.validate() {
            Ok(_) => {
                vald_errors
                    .borrow_mut()
                    .errors_mut()
                    .remove(name.as_str());
            }
            Err(errors) => {
                vald_errors
                    .borrow_mut()
                    .errors_mut()
                    .retain(|key, _| key != &name);
                for (field_name, error) in errors.errors() {
                    if field_name == &name {
                        vald_errors
                            .borrow_mut()
                            .errors_mut()
                            .insert(field_name, error.clone());
                    }
                }
            }
        }
    })
}

fn login_onsubmit_callback(
    form: UseStateHandle<LoginFormData>, 
    vald_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>>,
    navigator: Navigator,
    loading: UseStateHandle<bool>,
    username_ref: NodeRef,
    password_ref: NodeRef,
    dispatch: Dispatch<AuthUser>,
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        let form = form.clone();
        let vald_errors = vald_errors.clone();
        let navigator = navigator.clone();
        let loading = loading.clone();
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let dispatch = dispatch.clone();

        event.prevent_default();
        spawn_local(async move {
            match form.validate() {
                Ok(_) => {
                    let form_data = form.deref().clone();
                    loading.set(true);
                    let res = api_login_user(&form_data.into()).await;
                    
                    username_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    password_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    match res {
                        Ok(res) => {
                            log!("got user login response");
                            set_auth_token(Some(res.auth_token.clone()), dispatch);
                            loading.set(false);
                            navigator.push(&Route::Dashboard);
                        },
                        Err(e) => {
                            loading.set(false);
                            log!(format!("got error response: {:?}", e));
                            if let Error::Server(se) = &e {
                                if let crate::model::types::ServerErrorType::Authorization(a) = &se.error {
                                    match a {
                                        crate::model::types::AuthError::WrongPasswordOrUsername => {
                                            let err = ValidationError::new("WrongPasswordOrUsername").with_message(Cow::from("Unknown username or incorrect password"));
                                                vald_errors
                                                    .borrow_mut()
                                                    .errors_mut()
                                                    .insert("password", validator::ValidationErrorsKind::Field(vec![err]));
                                        },
                                        crate::model::types::AuthError::NotLoggedIn => todo!(),
                                        crate::model::types::AuthError::InvalidToken => todo!(),
                                    }
                                } else {
                                    navigator.push(&e.route_based_on_err());
                                }
                            } else {
                                navigator.push(&e.route_based_on_err());
                            }
                        },
                    }
                },
                Err(e) => {
                    vald_errors.set(Rc::new(RefCell::new(e)));
                },
            }
        });
    })
}

#[styled_component(LoginUser)]
pub fn login_user(_: &LoginProps) -> Html {
    let loading = use_state(|| false);
    let form = use_state(|| LoginFormData::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<AuthUser>();

    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let onchange = login_input_callback(form.clone());
    let onblur_validate = login_blur_callback(form.clone(), validation_errors.clone());
    let on_submit = login_onsubmit_callback(form.clone(), validation_errors.clone(), navigator.clone(), loading.clone(), username_input_ref.clone(), password_input_ref.clone(), dispatch);


    html! {
        <NavBar content_class={css!("display: flex; justify-content: center; align-items: center;")}>
            <ScrollDiv class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} style="padding: 20px;">
                <h1 class={css!("font-size: 2em;")}>{"Login"}</h1>
                <form class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} onsubmit={on_submit}>
                    <FormInput<LoginFormUpdate> 
                        input_type="text" placeholder="Username"  name="username" input_ref={username_input_ref} 
                        to_type={Callback::from(|s| LoginFormUpdate::Username(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<LoginFormUpdate>
                        input_type="password" placeholder="Password"  name="password" input_ref={password_input_ref} 
                        to_type={Callback::from(|s| LoginFormUpdate::Password(s))}
                        autocomplete={Some("current-password".to_string())}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <SubmitButton loading={*loading}> {"Submit"} </SubmitButton>
                </form>

                <div class={css!("margin-top: 14px; display: flex; flex-direction: column; justify-content: center; align-items: center; font-size: 1em;")}>
                    {"Don't have an account?"}
                    <Link<Route> to={Route::Register}>
                        {" Signup Here"}
                    </Link<Route>>
                </div>
            </ScrollDiv>
        </NavBar>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ProfileProps {
    pub id: uuid::Uuid
}


#[styled_component(UserProfile)]
pub fn user_profile(props: &ProfileProps) -> Html {
    let loading = use_state(|| false);
    let user_data = use_state(|| None);
    let navigator = use_navigator().unwrap();
    
    let props_cloned = props.clone();
    let loading_cloned = loading.clone();
    let user_data_cloned = user_data.clone();
    use_effect_with((), 
        move |_| {
            // let loading = .clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                loading_cloned.set(true);
                let response = api_public_user_info(props_cloned.id).await;

                match response {
                    Ok(data) => {
                        loading_cloned.set(false);
                        user_data_cloned.set(Some(data));
                    },
                    Err(e) => {
                        loading_cloned.set(false);
                        navigator.push(&e.route_based_on_err());
                    },
                }

            });
        }
    );

    let theme = use_theme();
    let profile_style = css!(
        r#"
            display: flex;
            flex-direction: column;
            height: 100%;

            .banner-img {
                flex: 25%;
                border-bottom: 3px solid ${profile_header_border};
                background-size: auto; 
                background-position: center; 
                background-repeat: no-repeat;
                background-size: cover;
                min-width: 400px;
                min-height: 300px;
            }

            .banner-img.loading {
                background: ${img_load};
            }

            .profile-img {
                position: absolute;
                top: 0;
                transform: translate(0, -70%);
            }

            .profile-content {
                flex: 75%;
                position: relative;
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            .text-area {
                width: 50%;
                word-wrap: break-word; 
                text-align: center;
                line-height: 1.5;
            }

            @media screen and (max-width: 800px) {
                .text-area {
                    width: 80%;
                }
            }
        "#,
        img_load = theme.panel_secondary,
        profile_header_border = theme.border_colored
    );

    let default = &PublicUserData::default();
    let user_data = user_data.deref().as_ref().unwrap_or(default);
    html! {
        <NavBar>
            <div class={profile_style}>
                <div class={if *loading {classes!("banner-img", "loading")} else { classes!("banner-img", css!("background-image: url(\"${src}\");", src=user_data.profile_banner))}}>
                </div>
                <div class="profile-content">
                    <ProfilePortrait class={"profile-img"} width="10em" height="10em" loading={*loading} src={user_data.profile_photo.clone()} />
                    
                    <h1 style="margin-top: 60px;">
                        if *loading { 
                            <SkeletonPane class={css!("margin: 0px 10px 10px 10px; width: 230px; height: 1em;")}/> 
                        } else { 
                            {user_data.profile_name.clone()} 
                        }
                    <hr/></h1>
                    <h2 style="font-style: italic; font-size: 1em;">
                        if *loading { 
                            <SkeletonPane class={css!("margin: 0px 10px 10px 10px; width: 100px; height: 1em;")}/> 
                        } else { 
                            {user_data.profile_catchphrase.clone()} 
                        }
                    </h2>

                    <div class="text-area">
                        if *loading { 
                            <SkeletonTextArea style="width: 100%;"/> 
                        } else { 
                            {user_data.profile_text.clone()} 
                        }
                    </div>
                </div>
            </div>
        </NavBar>
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct UserPreferncesProps;

#[styled_component(UserPrefernces)]
pub fn user_profile(_: &UserPreferncesProps) -> Html {
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<AuthUser>();
    let user = &state.auth_user;

    let loading_cloned = loading.clone();
    let navigator_cloned = navigator.clone();
    use_effect_with((), 
        move |_| {
            let navigator = navigator_cloned.clone();
            spawn_local(async move {
                loading_cloned.set(true);
                let response = api_user_info().await;

                match response {
                    Ok(data) => {
                        loading_cloned.set(false);
                        set_auth_user(Some(data), dispatch);
                    },
                    Err(e) => {
                        loading_cloned.set(false);
                        navigator.push(&e.route_based_on_err());
                    }
                }

            });
        }
    );

    let theme = use_theme();
    let profile_style = css!(
        r#"
            display: flex;
            flex-direction: column;
            height: 100%;

            @media screen and (max-width: 800px) {

            }
        "#
    );

    // TODO: Define a file upload component
    // let file_input_ref = use_node_ref();
    // let metadata_ref = use_node_ref();

    // let onsubmit = {
    //     let file_input_ref = file_input_ref.clone();
    //     let metadata_ref = metadata_ref.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         let navigator = navigator.clone();
    //         e.prevent_default();
    //         if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
    //             if let Some(files) = input.files() {
    //                 let file = files.get(0).unwrap(); // Get the first file
    //                 if let Some(metadata_input) = metadata_ref.cast::<HtmlInputElement>() {
    //                     let metadata = FileUploadMetadata {
    //                         name: metadata_input.value(),
    //                     };
    //                     spawn_local(async move {
    //                         let res = api_user_upload(metadata, &file).await;
    //                         match res {
    //                             Ok(_) => {},
    //                             Err(e) => if let Some(e) = e.route_based_on_err(&navigator) {
    //                                 // TODO:
    //                             },
    //                         }
    //                     });
    //                 }
    //             }
    //         }
    //     })
    // };

    html! {
        <NavBar content_class={css!("display: flex; align-items: center; justify-content: center;")}>
            <div>
                if let Some(data) = user {
                    <ProfileCard user={data.id}/>
                }
                // <form onsubmit={onsubmit}>
                //     <input type="file" ref={file_input_ref} />
                //     <input type="name" ref={metadata_ref} placeholder="Enter name" />
                //     <button type="submit">{ "Upload File" }</button>
                // </form>
                // <div>
                //     {"Left Side - Account Info"}
                //     <h2>{"Account"}</h2>
                //     <div>
                //         {"Email"}
                //     </div>
                //     <div>
                //         {"Username"}
                //     </div>
                // </div>
                // <div>
                //     {"Right Side - Profile Info w/ preview card"}
                //     <h2>{"Profile"}</h2>
                //     <div>
                //         {"Profile Name"}
                //     </div>
                //     <div>
                //         {"Profile Photo"}
                //     </div>
                //     <div>
                //         {"Profile Banner"}
                //     </div>
                // </div>
            </div>
        </NavBar>
    }
}