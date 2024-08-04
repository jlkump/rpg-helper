use crate::{error::Error, model::types::ServerError};

pub mod user_api;
pub mod data_api;

pub const API_URL: &str = "http://localhost:8090/api";

// Helper function to handle the general error http responses from the backend
async fn handle_response(response: &reqwasm::http::Response) -> Result<(), Error> {
    if response.status() != 200 {
        let error_response = response.json::<ServerError>().await;
        
        match error_response {
            Ok(error_response) => return Err(Error::Server(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }
    Ok(())
}

async fn handle_reqwest_response(response: reqwest::Response) -> Result<(), Error> {
    if response.status() != 200 {
        let error_response = response.json::<ServerError>().await;
        
        match error_response {
            Ok(error_response) => return Err(Error::Server(error_response)),
            Err(e) => return Err(Error::API(e.to_string()))
        }
    }
    Ok(())
}