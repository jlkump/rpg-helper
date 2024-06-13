use std::{borrow::Cow, cell::RefCell, f64::consts::E, ops::Deref, rc::Rc};

use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use stylist::{css, yew::styled_component};
use yew_router::{components::Link, hooks::use_navigator, navigator::{self, Navigator}};
use yewdux::{dispatch, use_store, Dispatch};

use crate::{api::{schema::UserRegistrationSchema, user_api::api_register_user}, gui::{contexts::style::theme::use_theme, display::{atoms::{button::SubmitButton, form_input::FormInput, scroll_div::ScrollDiv}, organisms::nav_bar::NavBar}}, router::Route, store::{set_page_loading, GlobalStore}};
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
                            .insert(field_name.clone(), error.clone());
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
    dispatch: Dispatch<GlobalStore>,
    username_ref: NodeRef,
    email_ref: NodeRef,
    password_ref: NodeRef,
    password_confirm_ref: NodeRef,
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        let form = form.clone();
        let vald_errors = vald_errors.clone();
        let navigator = navigator.clone();
        let dispatch = dispatch.clone();
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let password_confirm_ref = password_confirm_ref.clone();

        event.prevent_default();
        spawn_local(async move {
            match form.validate() {
                Ok(_) => {
                    let form_data = form.deref().clone();
                    set_page_loading(true, dispatch.clone());
                    let res = api_register_user(&form_data.into()).await;
                    
                    username_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    email_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    password_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));
                    password_confirm_ref.cast::<HtmlInputElement>().map(|v| v.set_value(""));

                    match res {
                        Ok(u) => {
                            set_page_loading(false, dispatch.clone());
                            navigator.push(&Route::Login);
                        },
                        Err(e) => {
                            set_page_loading(false, dispatch.clone());
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
                                crate::api::user_api::Error::API => log!("Got API Error: API Failed"),crate::api::user_api::Error::RequestFailed =>  log!("Got API Error: Request Failed"),crate::api::user_api::Error::ParseFailed =>  log!("Got API Error: Parse Failed"),
                            }
                            
                            // TODO: Show error based on resultant error recieved from API
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
    let (store, dispatch) = use_store::<GlobalStore>();
    let form = use_state(|| RegistrationFormData::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let username_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let password_confirm_input_ref = NodeRef::default();

    let onchange = registration_input_callback(form.clone());
    let onblur_validate = registration_blur_callback(form.clone(), validation_errors.clone());
    let on_submit = registration_onsubmit_callback(form.clone(), validation_errors.clone(), navigator.clone(), dispatch.clone(), username_input_ref.clone(), email_input_ref.clone(), password_input_ref.clone(), password_confirm_input_ref.clone());


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
                    <SubmitButton loading={store.page_loading}> {"Submit"} </SubmitButton>
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
pub struct LoginProps {
}

#[styled_component(LoginUser)]
pub fn login_user(_: &LoginProps) -> Html {
    html! {
        <NavBar>
            <h1>{"TODO"}</h1>
        </NavBar>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ProfileProps {
    pub edit: bool,
}


#[styled_component(UserProfile)]
pub fn user_profile(_: &ProfileProps) -> Html {
    html! {
        <NavBar>
            <h1>{"TODO"}</h1>
        </NavBar>
    }
}