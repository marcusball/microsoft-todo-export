
use serde::de::Deserialize;

pub type Date = String;
pub type DateTimeOffset = String;
pub type StringCollection = Vec<String>;

pub mod user;
pub mod tasks;

#[derive(Deserialize, Debug, Clone)]
pub struct OData {
    #[serde(rename = "@odata.context")] 
    pub context: Option<String>,

    #[serde(rename = "@odata.nextLink")] 
    pub next_link: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Collection<T> {
    pub value: Vec<T>,

    #[serde(flatten)]
    pub odata: OData,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeTimeZone {
    pub date_time: String,
    pub time_zone: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),

    Error(ErrorResponse)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: ErrorResponseError,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseError {
    pub code: String,

    pub message: String,

    inner_error: ErrorResponseErrorInnerError
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseErrorInnerError {
    date: String,

    request_id: String,

    client_request_id: String,
}

