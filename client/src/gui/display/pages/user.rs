use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{api::schema::UserRegistrationSchema, gui::display::organisms::nav_bar::NavBar};
use validator::Validate;


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
        <NavBar />
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoginProps {
}

#[styled_component(LoginUser)]
pub fn login_user(_: &LoginProps) -> Html {
    html! {
        <NavBar />
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ProfileProps {
}


#[styled_component(UserProfile)]
pub fn user_profile(_: &ProfileProps) -> Html {
    html! {
        <NavBar />
    }
}