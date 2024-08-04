use std::fmt::Display;

use crate::{api::{handle_reqwest_response, handle_response}, error::Error, model::{schema::{UserLoginSchema, UserRegistrationSchema}, types::{PublicUserData, UserData, UserLoginResponse}}, router::Route};

use gloo::console::{error, log};
use reqwasm::http;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{js_sys::{Promise, Uint8Array}, Blob, File, FileReader};

use super::API_URL;

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseFailed(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::API(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unauthorized => write!(f, "Unauthorized failure"),
            Error::API(e) =>  write!(f, "API Failure: {}", e),
            Error::RequestFailed(e) => write!(f, "Request Failure: {}", e),
            Error::ParseFailed(e) => write!(f, "Parse Failure: {}", e),
            Error::Server(e) => write!(f, "[Server Error] {:?}: {}", e, e.message),
            Error::QueryError(e) => write!(f, "[Query Error] {:?}", e),
        }
    }
}

impl Error {

    /// If the given error results in a route error, this simplifies the process by returning the route.
    /// If the error does not re-route, then the method returns None.
    /// Currently all errors re-route. 
    pub fn route_based_on_err(self) -> Route {
        match self {
            Error::Unauthorized => Route::Error { error: format!("Unauthorized error") },
            Error::API(mes) => Route::Error { error: format!("API Failure: \"{}\"", mes)},
            Error::RequestFailed(mes) => Route::Error { error: format!("Request failed. Server may be down. \"{}\"", mes) },
            Error::ParseFailed(mes) => Route::Error { error: format!("Parse of Server Data failed. Model may be different. \"{}\"", mes) },
            Error::Server(e) => Route::Error { error: format!("Server Error. Type: {:?}, mes: \"{}\"", e, e.message) },
            Error::QueryError(e) => Route::Error { error: format!("Query Error. Type: {:?}", e) },
        }
    }
}

pub async fn api_register_user(user_data: &UserRegistrationSchema) -> Result<UserData, Error> {
    log!("Calling api register user");
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

    handle_response(&response).await?;

    let res_json = response.json::<UserData>().await;
    match res_json {
        Ok(data) => return Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_login_user(credentials: &UserLoginSchema) -> Result<UserLoginResponse, Error> {
    log!("Calling api login user");
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

    handle_response(&response).await?;

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}


pub async fn api_user_info() -> Result<UserData, Error> {
    log!("Calling api fetch logged-in user");
    let url = format!("{}/user", API_URL);
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    let res_json = response.json::<UserData>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_public_user_info(user_id: uuid::Uuid) -> Result<PublicUserData, Error> {
    log!("Calling get public user data");
    let url = format!("{}/public/user/{}", API_URL, user_id);
    let response = match http::Request::get(&url)
        // .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    let res_json = response.json::<PublicUserData>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn api_user_upload(name: String, file: &File, auth_token: &str) -> Result<(), Error> {
    log!("Calling api upload file");
    let mut form = reqwest::multipart::Form::new();
    
    let file_contents = read_file(file).await?;
    let file_name = file.name();
    let file_part = reqwest::multipart::Part::bytes(file_contents)
        .file_name(file_name)
        // .mime_str(&file.type_())?;
        .mime_str("application/octet-stream")?;

    form = form.part("file", file_part);
    form = form.text("name", name);

    let url = format!("{}/user/upload", API_URL);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", auth_token)).map_err(|e| Error::API(e.to_string()))?
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
        return Err(Error::API("Failed".to_string()));
    }

    handle_reqwest_response(response).await?;
    Ok(())
}

async fn read_file(file: &File) -> Result<Vec<u8>, Error> {
    let reader = FileReader::new().map_err(|_| Error::API("Could not create FileReader".into()))?;
    let reader_ref = reader.clone();
    let file_blob: Blob = file.clone().dyn_into().map_err(|_| Error::API("Could not cast File to Blob".into()))?;

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

    wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|_| Error::API("File reading failed".into()))?;

    let array_buffer = reader_ref.result().map_err(|_| Error::API("Could not get result from FileReader".into()))?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let vec = uint8_array.to_vec();

    Ok(vec)
}

pub async fn api_logout_user() -> Result<(), Error> {
    log!("Calling api logout user");
    let url = format!("{}/auth/logout", API_URL);
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    Ok(())
}