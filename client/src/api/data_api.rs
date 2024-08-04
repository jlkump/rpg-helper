use gloo::console::log;
use reqwasm::http;

use crate::{api::handle_response, error::Error, model::{data_model::network::ruleset::RulesetDataRaw, schema::RulesetRequestSchema}};

use super::API_URL;

pub async fn fetch_ruleset_data(schema: RulesetRequestSchema) -> Result<RulesetDataRaw, Error> {
    log!("Fetching ruleset data");
    let url = format!("{}/data/ruleset/{}", API_URL, schema.id);
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