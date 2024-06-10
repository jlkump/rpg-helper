use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::components::Link;

use crate::{api::schema::UserRegistrationSchema, gui::display::{atoms::{button::SubmitButton, form_input::FormInput, scroll_div::ScrollDiv}, organisms::nav_bar::NavBar}, router::Route};
use validator::{Validate, ValidationErrors};


#[derive(Properties, Clone, PartialEq)]
pub struct RegisterProps {
}


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

enum RegistrationFormUpdate {
    Username(String),
    Email(String),
    Password(String),
    PasswordConfirm(String),
}
// TODO: Split up and create Input component
fn get_registration_input_callback() -> Callback<String> {
    todo!()
}

#[styled_component(RegisterUser)]
pub fn register_user(_: &RegisterProps) -> Html {
    // Display changes based on whether logged-in or not
    // TODO: Complete based on tutorial here: https://codevoweb.com/rust-yew-frontend-jwt-access-and-refresh-tokens/
    html! {
        <NavBar content_class={css!("display: flex; justify-content: center; align-items: center;")}>
            <ScrollDiv class={css!("display: flex; flex-direction: column; justify-content: center; align-items: center;")} style="padding: 20px;">
                <h1 class={css!("font-size: 2em;")}>{"Sign Up"}</h1>
                <FormInput input_type="text" placeholder="Username" label="" name="username" input_ref={NodeRef::default()} onchange={Callback::from(|_| ())} onblur={Callback::from(|_| ())} errors={Rc::new(RefCell::new(ValidationErrors::default()))} />
                <FormInput input_type="text" placeholder="Email" label="" name="email" input_ref={NodeRef::default()} onchange={Callback::from(|_| ())} onblur={Callback::from(|_| ())} errors={Rc::new(RefCell::new(ValidationErrors::default()))} />
                <FormInput input_type="password" placeholder="Password" label="" name="password" input_ref={NodeRef::default()} onchange={Callback::from(|_| ())} onblur={Callback::from(|_| ())} errors={Rc::new(RefCell::new(ValidationErrors::default()))} />
                <FormInput input_type="password" placeholder="Confirm Password" label="" name="confirm password" input_ref={NodeRef::default()} onchange={Callback::from(|_| ())} onblur={Callback::from(|_| ())} errors={Rc::new(RefCell::new(ValidationErrors::default()))} />
                <SubmitButton loading=false> {"Submit"} </SubmitButton>
                
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