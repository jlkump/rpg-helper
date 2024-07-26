use std::{fmt::{Debug, Display}, io::Bytes};

use crate::{model::types::ServerError, router::Route};

use gloo::console::{error, log};
use reqwasm::http::{self, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{js_sys::{Promise, Uint8Array}, Blob, File, FileReader, FormData};
use yew_router::navigator::Navigator;

// TODO: Expand Error types for each HTTP Response type
pub enum Error<T> {
    Standard(T),
    Unauthorized,
    API(String),
    Server(ServerError),
    RequestFailed(String),
    ParseFailed(String),
    Other(String)
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseFailed(value.to_string())
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(value: reqwest::Error) -> Self {
        Self::Other(value.to_string())
    }
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
            Error::Other(e) => write!(f, "{}", e),
            Error::Server(e) => write!(f, "[Server Error] {}: {}", e.error, e.message),
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
            Error::Other(e) => navigator.push(&Route::Error { error: format!("Other Error: \"{}\"", e) }),
            Error::Server(e) => navigator.push(&Route::Error { error: format!("Server Error. Type: {}, mes: \"{}\"", e.error, e.message) }),
        }
        return None
    }
}

pub trait ErrorRoute { fn route(self, navigator: &Navigator) -> Option<Self> where Self: Sized; }

impl ErrorRoute for RegistrationError { fn route(self, _: &Navigator) -> Option<Self> { Some(self) } }

impl ErrorRoute for LoginError { fn route(self, _: &Navigator) -> Option<Self> { Some(self) } }

impl ErrorRoute for UploadError { fn route(self, _: &Navigator) -> Option<Self> { Some(self) } }

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

    if response.status() == 500 {
        let e = response.json::<ServerError>().await;
        match e {
            Ok(server_err) => return Err(Error::Server(server_err)),
            Err(e) => return Err(Error::API(e.to_string())),
        }
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

async fn handle_reqwest_response<E>(response: reqwest::Response) -> Result<(), Error<E>> 
where 
    E: DeserializeOwned
{
    if response.status() == 401 {
        return Err(Error::Unauthorized);
    }

    if response.status() == 500 {
        let e = response.json::<ServerError>().await;
        match e {
            Ok(server_err) => return Err(Error::Server(server_err)),
            Err(e) => return Err(Error::API(e.to_string())),
        }
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

pub async fn api_public_user_info(user_id: uuid::Uuid) -> Result<PublicUserData, Error<UserDataError>> {
    let url = format!("{}/public/user/{}", API_URL, user_id);
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

pub async fn api_user_upload(meta_data: FileUploadMetadata, file: &File, auth_token: &str) -> Result<(), Error<UploadError>> {
    let mut form = reqwest::multipart::Form::new();
    
    let file_contents = read_file(file).await?;
    let file_name = file.name();
    let file_part = reqwest::multipart::Part::bytes(file_contents)
        .file_name(file_name)
        // .mime_str(&file.type_())?;
        .mime_str("application/octet-stream")?;

    form = form.part("file", file_part);
    form = form.text("name", meta_data.name);


    let url = format!("{}/user/upload", API_URL);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", auth_token)).map_err(|e| Error::Other(e.to_string()))?
    );

    let client = reqwest::Client::builder()
        .build()?;
    let response = match client.post(&url)
        .headers(headers)
        .multipart(form)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    log!("Response Status:", response.status().to_string());
    log!("Response Headers:", format!("{:?}", response.headers()));

    if !response.status().is_success() {
        let error_body = response.text().await?;
        error!("Error response body:", error_body);
        return Err(Error::Other("Failed".to_string()));
    }

    handle_reqwest_response::<UploadError>(response).await?;
    Ok(())
}

async fn read_file(file: &File) -> Result<Vec<u8>, Error<UploadError>> {
    let reader = FileReader::new().map_err(|_| Error::Other("Could not create FileReader".into()))?;
    let reader_ref = reader.clone();
    let file_blob: Blob = file.clone().dyn_into().map_err(|_| Error::Other("Could not cast File to Blob".into()))?;

    let promise = Promise::new(&mut |resolve, reject| {
        let onload = Closure::once_into_js(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).unwrap();
        });
        let onerror = Closure::once_into_js(move |_event: web_sys::Event| {
            reject.call0(&JsValue::NULL).unwrap();
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        reader.read_as_array_buffer(&file_blob).expect("Could not read file");
    });

    wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|_| Error::Other("File reading failed".into()))?;

    let array_buffer = reader_ref.result().map_err(|_| Error::Other("Could not get result from FileReader".into()))?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let vec = uint8_array.to_vec();

    Ok(vec)
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