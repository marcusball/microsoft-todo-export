
use serde::de::Deserialize;

pub type DateTimeOffset = String;
pub type StringCollection = Vec<String>;

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

/// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#properties
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub about_me: Option<String>,

    pub account_enabled: Option<bool>,

    pub age_group: Option<Option<AgeGroup>>,

    // TODO: assignedLicenses
    // TODO: assignedPlans

    pub birthday: Option<Option<String>>,

    pub business_phones: Option<Vec<String>>,

    pub city: Option<String>,

    pub company_name: Option<String>,

    pub consent_provided_for_minor: Option<Option<ConsentProvidedForMinor>>,

    pub country: Option<String>,

    pub created_date_time: Option<String>,

    pub creation_time: Option<String>,

    pub deleted_date_time: Option<String>,

    pub department: Option<String>,

    pub display_name: String,

    pub employee_hire_date: Option<String>,

    pub employee_id: Option<String>,

    // TODO: employeeOrgData

    pub employee_type: Option<String>,

    pub external_user_state: Option<String>,

    pub external_user_state_change_date_time: Option<String>,

    pub fax_number: Option<String>,

    pub given_name: String,

    pub hire_date: Option<String>,

    pub id: String,

    pub identities: Option<Vec<ObjectIdentity>>,

    pub im_addresses: Option<Vec<String>>,

    pub interests: Option<Vec<String>>,

    pub is_resource_account: Option<bool>,

    pub job_title: Option<String>,

    pub last_password_change_date_time: Option<String>,

    pub legal_age_group_classification: Option<Option<LegalAgeGroupClassification>>,

    // TODO: licenseAssignmentStates

    pub mail: Option<String>,

    // TODO: mailboxSettings 

    pub mail_nickname: Option<String>,

    pub mobile_phone: Option<String>,

    pub my_site: Option<String>,

    pub office_location: Option<String>,

    pub on_premises_distinguished_name: Option<String>,

    pub on_premises_domain_name: Option<String>,

    // TODO: onPremisesExtensionAttributes

    pub on_premises_immutable_id: Option<String>,

    pub on_premises_last_sync_date_time: Option<String>,

    // TODO: onPremisesProvisioningErrors

    pub on_premises_sam_account_name: Option<String>,

    pub on_premises_security_identifier: Option<String>,

    pub on_premises_sync_enabled: Option<bool>,

    pub on_premises_user_principal_name: Option<String>,

    pub other_mails: Option<Vec<String>>,

    pub password_policies: Option<String>,

    // TODO: passwordProfile

    pub past_projects: Option<Vec<String>>,

    pub postal_code: Option<String>,

    pub preferred_language: Option<String>,

    pub preferred_name: Option<String>,

    // TODO: provisionedPlans 

    pub proxy_addresses: Option<Vec<String>>,

    pub refresh_tokens_valid_from_date_time: Option<String>,

    pub responsibilities: Option<Vec<String>>,

    pub schools: Option<Vec<String>>,

    pub show_in_address_list: Option<bool>,

    pub skills: Option<Vec<String>>,

    pub sign_in_sessions_valid_from_date_time: Option<DateTimeOffset>,

    pub state: Option<String>,

    pub street_address: Option<String>,

    pub surname: String,

    pub usage_location: Option<String>,

    pub user_principal_name: String,

    pub user_type: Option<String>,
}

/// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#agegroup-values
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AgeGroup {
    Minor,
    NotAdult,
    Adult
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ConsentProvidedForMinor {
    Granted,
    Denied,
    NotRequired
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LegalAgeGroupClassification {
    MinorWithOutParentalConsent, 
    MinorWithParentalConsent, 
    MinorNoParentalConsentRequired, 
    NotAdult,
    Adult,
}

/// See: https://docs.microsoft.com/en-us/graph/api/resources/objectidentity?view=graph-rest-1.0
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectIdentity {
    pub sign_in_type: String,
    pub issuer: String,
    pub issuer_assigned_id: String,
}