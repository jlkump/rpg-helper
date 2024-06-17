use std::{borrow::{Borrow, Cow}, cell::RefCell, ops::Deref, rc::Rc};

use gloo::console::log;
use html::IntoPropValue;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use stylist::{css, yew::styled_component};
use yew_router::{components::Link, hooks::use_navigator, navigator::{self, Navigator}};
use yewdux::{dispatch, use_store, Dispatch};

use crate::{api::{schema::{UserLoginSchema, UserRegistrationSchema}, types::PublicUserData, user_api::{api_login_user, api_public_user_info, api_register_user}}, gui::{contexts::style::theme::use_theme, display::{atoms::{button::SubmitButton, form_input::FormInput, profile::ProfilePortrait, scroll_div::ScrollDiv, loading::{SkeletonPane, SkeletonTextArea}}, organisms::nav_bar::NavBar}}, router::Route, store::GlobalStore};
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
                            match e {
                                crate::api::user_api::Error::Standard(registration) => {
                                    let err;
                                    let key;
                                    match registration {
                                        crate::api::types::RegistrationError::UsernameTaken => {
                                            err = ValidationError::new("UsernameTaken").with_message(Cow::from("Username is taken"));
                                            key = "username";
                                        },
                                        crate::api::types::RegistrationError::EmailTaken => {
                                            err = ValidationError::new("EmailTaken").with_message(Cow::from("Email is taken"));
                                            key = "email";
                                        },
                                    }
                                    vald_errors
                                        .borrow_mut()
                                        .errors_mut()
                                        .insert(key, validator::ValidationErrorsKind::Field(vec![err]));
                                },
                                crate::api::user_api::Error::API(mes) => log!("Got API Error: API Failed {}", mes),crate::api::user_api::Error::RequestFailed =>  log!("Got API Error: Request Failed"),crate::api::user_api::Error::ParseFailed =>  log!("Got API Error: Parse Failed"),
                            }
                            
                            // TODO: Show error based on resultant error recieved from API
                            // Probably route to a Server-Down page
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
                        input_type="text" placeholder="Username" label="" name="username" input_ref={username_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Username(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="text" placeholder="Email" label="" name="email" input_ref={email_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Email(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="password" placeholder="Password" label="" name="password" input_ref={password_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::Password(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<RegistrationFormUpdate>
                        input_type="password" placeholder="Confirm Password" label="" name="password_confirm" input_ref={password_confirm_input_ref} 
                        to_type={Callback::from(|s| RegistrationFormUpdate::PasswordConfirm(s))}
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
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        let form = form.clone();
        let vald_errors = vald_errors.clone();
        let navigator = navigator.clone();
        let loading = loading.clone();
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

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
                        Ok(_) => {
                            loading.set(false);
                            navigator.push(&Route::Dashboard);
                            // Use auth token?
                        },
                        Err(e) => {
                            loading.set(false);
                            match e {
                                crate::api::user_api::Error::Standard(login_err) => {
                                    let err;
                                    let key;
                                    match login_err {
                                        crate::api::types::LoginError::UnknownUsernameOrPassword => {
                                            err = ValidationError::new("WrongPasswordOrUsername").with_message(Cow::from("Unknown username or incorrect password"));
                                            key = "password";
                                        },
                                    }
                                    vald_errors
                                        .borrow_mut()
                                        .errors_mut()
                                        .insert(key, validator::ValidationErrorsKind::Field(vec![err]));
                                },
                                crate::api::user_api::Error::API(mes) => log!("Got API Error: API Failed {}", mes),crate::api::user_api::Error::RequestFailed =>  log!("Got Login API Error: Request Failed"),crate::api::user_api::Error::ParseFailed =>  log!("Got API Error: Parse Failed"),
                            }
                            
                            // TODO: Show error based on resultant error recieved from API
                            // Probably route to a Server-Down page
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

    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let onchange = login_input_callback(form.clone());
    let onblur_validate = login_blur_callback(form.clone(), validation_errors.clone());
    let on_submit = login_onsubmit_callback(form.clone(), validation_errors.clone(), navigator.clone(), loading.clone(), username_input_ref.clone(), password_input_ref.clone());


    html! {
        <NavBar content_class={css!("display: flex; justify-content: center; align-items: center;")}>
            <ScrollDiv class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} style="padding: 20px;">
                <h1 class={css!("font-size: 2em;")}>{"Login"}</h1>
                <form class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} onsubmit={on_submit}>
                    <FormInput<LoginFormUpdate> 
                        input_type="text" placeholder="Username" label="" name="username" input_ref={username_input_ref} 
                        to_type={Callback::from(|s| LoginFormUpdate::Username(s))}
                        onchange={onchange.clone()} 
                        onblur={onblur_validate.clone()} 
                        errors={&*validation_errors} 
                    />
                    <FormInput<LoginFormUpdate>
                        input_type="password" placeholder="Password" label="" name="password" input_ref={password_input_ref} 
                        to_type={Callback::from(|s| LoginFormUpdate::Password(s))}
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
    pub name: String
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
                let response = api_public_user_info(props_cloned.name).await;

                match response {
                    Ok(data) => {
                        loading_cloned.set(false);
                        user_data_cloned.set(Some(data))
                    },
                    Err(e) => {
                        loading_cloned.set(false);
                        match e {
                            crate::api::user_api::Error::Standard(e) => {
                                match e {
                                    crate::api::types::UserDataError::UserIdNotFound(_) |crate::api::types::UserDataError::UsernameNotFound(_) => navigator.push(&Route::NotFound),
                                }
                            },
                            crate::api::user_api::Error::API(_) => todo!(),
                            crate::api::user_api::Error::RequestFailed => todo!(),
                            crate::api::user_api::Error::ParseFailed => todo!(),
                        }
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
                <div class={if *loading {classes!("banner-img", "loading")} else { classes!("banner-img", css!("background-image: url(\"${src}\");", src="/img/generic/Birb Wizard Transparent.png"))}}>
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

                    <div class="text-area">
                        if *loading { 
                            <SkeletonTextArea style="width: 100%;"/> 
                        } else { 
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras lacinia venenatis sapien, ac mollis ligula suscipit et. Vestibulum at porta magna, quis posuere metus. Nullam lorem mauris, vulputate quis libero quis, laoreet egestas diam. Vivamus feugiat, lacus ut iaculis dictum, massa erat tincidunt tellus, sit amet posuere neque erat sed neque. Nulla leo urna, consectetur quis nunc non, maximus bibendum sem. Ut in mi interdum, placerat lacus ut, aliquet erat. Morbi sed ultricies dolor. Fusce pellentesque massa nec finibus fringilla. Vivamus nec lobortis ligula. Vivamus augue justo, pretium sit amet nisi quis, consequat bibendum libero."} 
                        }
                    </div>
                </div>
            </div>
        </NavBar>
    }
}