use gloo::console::log;
use reqwasm::http;

use crate::{api::handle_response, error::Error, model::{data_model::network::ruleset::RulesetDataRaw, schema::{RulesetCreateSchema}, types::RulesetId}};

use super::API_URL;

pub async fn fetch_ruleset_data(ruleset_id: RulesetId) -> Result<RulesetDataRaw, Error> {
    log!("Fetching ruleset data");
    let url = format!("{}/data/ruleset/fetch/{}", API_URL, ruleset_id);
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    let res_json = response.json::<RulesetDataRaw>().await;
    match res_json {
        Ok(data) => return Ok(data.into()),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn update_ruleset_data(data: RulesetDataRaw) -> Result<(), Error> {
    log!("Updating ruleset data");
    let url = format!("{}/data/ruleset/update", API_URL);
    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    let res_json = response.json::<()>().await;
    match res_json {
        Ok(_) => return Ok(()),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}

pub async fn create_new_ruleset(schema: RulesetCreateSchema) -> Result<(), Error> {
    log!("Creating ruleset");
    let url = format!("{}/data/ruleset/create", API_URL);
    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(serde_json::to_string(&schema).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(Error::RequestFailed(e.to_string())),
    };

    handle_response(&response).await?;

    let res_json = response.json::<()>().await;
    match res_json {
        Ok(_) => return Ok(()),
        Err(e) => Err(Error::ParseFailed(e.to_string())),
    }
}