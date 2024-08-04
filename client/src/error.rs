use crate::model::{data_model::storage::QueryError, types::ServerError};

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    API(String),
    Server(ServerError),
    RequestFailed(String),
    ParseFailed(String),
    QueryError(QueryError)
}