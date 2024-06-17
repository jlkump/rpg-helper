use super::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{LoginError, PublicUserData, RegistrationError, UserData, UserDataError, UserDataResponse, UserLoginResponse}, API_URL};
use reqwasm::http;

pub enum Error<T> {
    Standard(T),
    API(String),
    RequestFailed,
    ParseFailed
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
        Err(_) => return Err(Error::RequestFailed),
    };

    if response.status() != 200 {
        let error_response = response.json::<RegistrationError>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }

    let res_json = response.json::<UserDataResponse>().await;
    match res_json {
        Ok(response) => {
            match response {
                UserDataResponse::Private(data) => return Ok(data),
                UserDataResponse::Public(_) => return Err(Error::API("Got public data for private profile".to_string())),
            }
        },
        Err(_) => Err(Error::ParseFailed),
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
        Err(_) => return Err(Error::RequestFailed),
    };

    if response.status() != 200 {
        let error_response = response.json::<LoginError>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err(Error::ParseFailed),
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
        Err(_) => return Err(Error::RequestFailed),
    };

    if response.status() != 200 {
        let error_response = response.json::<UserDataError>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }

    let res_json = response.json::<UserDataResponse>().await;
    match res_json {
        Ok(response) => {
            match response {
                UserDataResponse::Private(data) => return Ok(data),
                UserDataResponse::Public(_) => return Err(Error::API("Got public data for private profile".to_string())),
            }
        },
        Err(_) => Err(Error::ParseFailed),
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
        Err(_) => return Err(Error::RequestFailed),
    };

    if response.status() != 200 {
        let error_response = response.json::<UserDataError>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }

    let res_json = response.json::<UserDataResponse>().await;
    match res_json {
        Ok(response) => {
            match response {
                UserDataResponse::Private(_) => return Err(Error::API("Got private data as unauthorized".to_string())),
                UserDataResponse::Public(data) => Ok(data),
            }
        },
        Err(_) => Err(Error::ParseFailed),
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
        Err(_) => return Err(Error::RequestFailed),
    };

    if response.status() != 200 {
        let error_response = response.json::<String>().await;
        match error_response {
            Ok(error_response) => return Err(Error::Standard(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }

    Ok(())
}