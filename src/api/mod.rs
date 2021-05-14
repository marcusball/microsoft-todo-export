
use serde::de::Deserialize;

pub type DateTimeOffset = String;
pub type StringCollection = Vec<String>;

pub mod user;
pub mod tasks;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),

    Error(ErrorResponse)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: ErrorResponseError,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseError {
    pub code: String,

    pub message: String,

    inner_error: ErrorResponseErrorInnerError
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseErrorInnerError {
    date: String,

    request_id: String,

    client_request_id: String,
}

