use std::fmt::{Debug, Display};

use crate::router::Route;

use super::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{LoginError, PublicUserData, RegistrationError, UserData, UserDataError, UserLoginResponse}, API_URL};
use gloo::console::log;
use reqwasm::http::{self, Response};
use serde::de::DeserializeOwned;
use yew_router::navigator::Navigator;

pub enum Error<T> {
    Standard(T),
    Unauthorized,
    API(String),
    RequestFailed(String),
    ParseFailed(String)
}

impl<T> Display for Error<T> 
where 
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Standard(t) => write!(f, "{}", t),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::API(e) =>  write!(f, "API Failure: {}", e),
            Error::RequestFailed(e) => write!(f, "Request Failure: {}", e),
            Error::ParseFailed(e) => write!(f, "Parse Failure: {}", e),
        }
    }
}

impl Display for UserDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDataError::UserIdNotFound(i) => write!(f, "User id \"{}\" not found", i),
            UserDataError::UsernameNotFound(n) => write!(f, "Username \"{}\" not found", n),
        }
    }
}

impl<T> Error<T> 
where 
    T: Debug + ErrorRoute
{
    pub fn route_based_on_err(self, navigator: &Navigator) -> Option<T> {
        match self {
            Error::Standard(e) => return e.route(navigator),
            Error::API(mes) => navigator.push(&Route::Error { error: format!("API Failure: \"{}\"", mes)}),
            Error::RequestFailed(mes) => navigator.push(&Route::Error { error: format!("Request failed. Server may be down. \"{}\"", mes) }),
            Error::ParseFailed(mes) => navigator.push(&Route::Error { error: format!("Parse of Server Data failed. Model may be different. \"{}\"", mes) }),
            Error::Unauthorized => navigator.push(&Route::Home),
        }
        return None
    }
}

pub trait ErrorRoute { fn route(self, navigator: &Navigator) -> Option<Self> where Self: Sized; }

impl ErrorRoute for RegistrationError { fn route(self, _: &Navigator) -> Option<Self> { Some(self) } }

impl ErrorRoute for LoginError { fn route(self, _: &Navigator) -> Option<Self> { Some(self) } }

impl ErrorRoute for String { 
    fn route(self, navigator: &Navigator) -> Option<Self> {
        navigator.push(&Route::Error { error: format!("Got Error: {}", self) }); None
    }
}

impl ErrorRoute for UserDataError {
    fn route(self, navigator: &Navigator) -> Option<Self> {
        match self {
            UserDataError::UserIdNotFound(_) | UserDataError::UsernameNotFound(_) => 
                navigator.push(&Route::NotFound),
        }
        None
    }
}

// Helper function to handle the general error http responses from the backend
async fn handle_response<E>(response: &Response) -> Result<(), Error<E>> 
where 
    E: DeserializeOwned
{
    if response.status() == 401 {
        return Err(Error::Unauthorized);
    }

    if response.status() != 200 {
        let error_response = response.json::<E>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }
    Ok(())
}

pub async fn api_register_user(user_data: &UserRegistrationSchema) -> Result<UserData, Error<RegistrationError>> {
    let url = format!("{}/auth/register", API_URL);
    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(user_data).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response::<RegistrationError>(&response).await?;

    let res_json = response.json::<UserData>().await;
    match res_json {
        Ok(data) => return Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_login_user(credentials: &UserLoginSchema) -> Result<UserLoginResponse, Error<LoginError>> {
    let url = format!("{}/auth/login", API_URL);
    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(serde_json::to_string(credentials).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response::<LoginError>(&response).await?;

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}


pub async fn api_user_info() -> Result<UserData, Error<UserDataError>> {
    let url = format!("{}/user", API_URL);
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response::<UserDataError>(&response).await?;

    let res_json = response.json::<UserData>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_public_user_info(username: String) -> Result<PublicUserData, Error<UserDataError>> {
    let url = format!("{}/user/{}", API_URL, username);
    let response = match http::Request::get(&url)
        // .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response::<UserDataError>(&response).await?;

    let res_json = response.json::<PublicUserData>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_logout_user() -> Result<(), Error<String>> {
    let url = format!("{}/auth/logout", API_URL);
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response::<String>(&response).await?;

    Ok(())
}