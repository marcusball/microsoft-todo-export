
pub type DateTimeOffset = String;
pub type StringCollection = Vec<String>;

/// Represents a `User` resource type.
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#properties
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// A freeform text entry field for the user to describe themselves.
    pub about_me: Option<String>,

    /// `true` if the account is enabled; otherwise, `false`. This property is required when a user is created. 
    pub account_enabled: Option<bool>,

    /// Sets the age group of the user. 
    /// 
    /// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#legal-age-group-property-definitions
    pub age_group: Option<AgeGroup>,

    // TODO: assignedLicenses
    // TODO: assignedPlans

    /// The birthday of the user. The Timestamp type represents date and time information using ISO 8601 format and is always in UTC time. 
    /// For example, midnight UTC on Jan 1, 2014 is `2014-01-01T00:00:00Z`.
    pub birthday: Option<String>,

    /// The telephone numbers for the user. NOTE: Although this is a string collection, 
    /// only one number can be set for this property.
    /// 
    /// Read-only for users synced from on-premises directory. Returned by default.
    pub business_phones: Option<Vec<String>>,

    /// The city in which the user is located. Maximum length is 128 characters.
    pub city: Option<String>,

    /// The company name which the user is associated. This property can be useful for describing the company that an external user comes from. 
    /// The maximum length of the company name is 64 characters.
    ///
    /// Returned only on $select.
    pub company_name: Option<String>,

    /// Sets whether consent has been obtained for minors.
    /// 
    /// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#legal-age-group-property-definitions
    pub consent_provided_for_minor: Option<ConsentProvidedForMinor>,

    /// The country/region in which the user is located; for example, "US" or "UK". Maximum length is 128 characters.
    pub country: Option<String>,

    /// The created date of the user object. 
    pub created_date_time: Option<String>,

    /// Indicates whether the user account was created as a regular school or work account (`None`), an external account (`Invitation`), 
    /// a local account for an Azure Active Directory B2C tenant (`LocalAccount`) or self-service sign-up using email verification (`EmailVerified`). Read-only.
    pub creation_type: Option<String>,

    /// The date and time the user was deleted. 
    pub deleted_date_time: Option<String>,

    /// The name for the department in which the user works. Maximum length is 64 characters. 
    pub department: Option<String>,

    /// The name displayed in the address book for the user. This is usually the combination of the user's first name, middle initial and last name. 
    /// This property is required when a user is created and it cannot be cleared during updates. Maximum length is 256 characters.
    ///
    /// Returned by default.
    pub display_name: String,

    /// The date and time when the user was hired or will start work in case of a future hire.
    /// 
    /// Returned only on $select. 
    pub employee_hire_date: Option<String>,

    /// The employee identifier assigned to the user by the organization.
    /// 
    /// Returned only on $select. 
    pub employee_id: Option<String>,

    // TODO: employeeOrgData

    /// Captures enterprise worker type. For example, `Employee`, `Contractor`, `Consultant`, or `Vendor`. 
    /// 
    /// Returned only on $select. 
    pub employee_type: Option<String>,

    /// For an external user invited to the tenant using the invitation API, this property represents the invited user's invitation status. 
    /// For invited users, the state can be `PendingAcceptance` or `Accepted`, or `None` for all other users.
    /// 
    /// Returned only on $select. 
    pub external_user_state: Option<String>,

    /// Shows the timestamp for the latest change to the `external_user_state` property.
    /// 
    /// Returned only on $select.
    pub external_user_state_change_date_time: Option<String>,

    /// The fax number of the user.
    pub fax_number: Option<String>,

    /// The given name (first name) of the user. Returned by default. Maximum length is 64 characters. 
    pub given_name: String,

    /// The hire date of the user. The Timestamp type represents date and time information using ISO 8601 format and is always in UTC time. 
    /// For example, midnight UTC on Jan 1, 2014 is `2014-01-01T00:00:00Z`.
    /// 
    /// Returned only on $select. 
    /// 
    /// *Note:* This property is specific to SharePoint Online. We recommend using the native `employeeHireDate` property to set and update hire date values using Microsoft Graph APIs.
    pub hire_date: Option<String>,

    /// The unique identifier for the user. Should be treated as an opaque identifier. Inherited from directoryObject. Key. 
    pub id: String,

    /// Represents the identities that can be used to sign in to this user account. An identity can be provided by Microsoft (also known as a local account), 
    /// by organizations, or by social identity providers such as Facebook, Google, and Microsoft, and tied to a user account. 
    /// May contain multiple items with the same signInType value. 
    pub identities: Option<Vec<ObjectIdentity>>,

    /// The instant message voice over IP (VOIP) session initiation protocol (SIP) addresses for the user. Read-only.
    pub im_addresses: Option<Vec<String>>,

    /// A list for the user to describe their interests.
    pub interests: Option<Vec<String>>,

    /// Do not use – reserved for future use.
    pub is_resource_account: Option<bool>,

    /// The user's job title. Maximum length is 128 characters. Returned by default.
    pub job_title: Option<String>,

    /// The time when this Azure AD user last changed their password. The date and time information uses ISO 8601 format and is always in UTC time. For example, midnight UTC on Jan 1, 2014 is `2014-01-01T00:00:00Z`.
    pub last_password_change_date_time: Option<String>,

    /// Used by enterprise applications to determine the legal age group of the user. This property is read-only and calculated based on `age_group` and `consent_provided_for_minor` properties.
    pub legal_age_group_classification: Option<Option<LegalAgeGroupClassification>>,

    // TODO: licenseAssignmentStates

    /// The SMTP address for the user, for example, "jeff@contoso.onmicrosoft.com".
    /// NOTE: While this property can contain accent characters, using them can cause access issues with other Microsoft applications for the user. 
    pub mail: Option<String>,

    // TODO: mailboxSettings 

    /// The mail alias for the user. This property must be specified when a user is created. Maximum length is 64 characters.
    pub mail_nickname: Option<String>,

    /// The primary cellular telephone number for the user. Read-only for users synced from on-premises directory. Maximum length is 64 characters. Returned by default.
    pub mobile_phone: Option<String>,

    /// The URL for the user's personal site.
    pub my_site: Option<String>,

    /// The office location in the user's place of business. Returned by default.
    pub office_location: Option<String>,

    /// Contains the on-premises Active Directory distinguished name or DN. The property is only populated for customers who are synchronizing their on-premises directory to Azure Active Directory via Azure AD Connect. Read-only.
    pub on_premises_distinguished_name: Option<String>,

    /// Contains the on-premises domainFQDN, also called dnsDomainName synchronized from the on-premises directory. The property is only populated for customers who are synchronizing their on-premises directory to Azure Active Directory via Azure AD Connect. Read-only.
    pub on_premises_domain_name: Option<String>,

    // TODO: onPremisesExtensionAttributes

    /// This property is used to associate an on-premises Active Directory user account to their Azure AD user object. This property must be specified when creating a new user account in the Graph if you are using a federated domain for the user's userPrincipalName (UPN) property. Important: The $ and _ characters cannot be used when specifying this property.
    pub on_premises_immutable_id: Option<String>,

    /// Indicates the last time at which the object was synced with the on-premises directory; for example: "2013-02-16T03:04:54Z". The Timestamp type represents date and time information using ISO 8601 format and is always in UTC time. For example, midnight UTC on Jan 1, 2014 is `2014-01-01T00:00:00Z`. Read-only.
    pub on_premises_last_sync_date_time: Option<String>,

    // TODO: onPremisesProvisioningErrors

    /// Contains the on-premises samAccountName synchronized from the on-premises directory. The property is only populated for customers who are synchronizing their on-premises directory to Azure Active Directory via Azure AD Connect. Read-only.
    pub on_premises_sam_account_name: Option<String>,

    /// Contains the on-premises security identifier (SID) for the user that was synchronized from on-premises to the cloud. Read-only.
    pub on_premises_security_identifier: Option<String>,

    /// true if this object is synced from an on-premises directory; false if this object was originally synced from an on-premises directory but is no longer synced; null if this object has never been synced from an on-premises directory (default). Read-only. 
    pub on_premises_sync_enabled: Option<bool>,

    /// Contains the on-premises userPrincipalName synchronized from the on-premises directory. The property is only populated for customers who are synchronizing their on-premises directory to Azure Active Directory via Azure AD Connect. Read-only.
    pub on_premises_user_principal_name: Option<String>,

    /// A list of additional email addresses for the user; for example: ["bob@contoso.com", "Robert@fabrikam.com"].
    /// NOTE: While this property can contain accent characters, they can cause access issues to first-party applications for the user. 
    pub other_mails: Option<Vec<String>>,

    /// Specifies password policies for the user. This value is an enumeration with one possible value being “DisableStrongPassword”, which allows weaker passwords than the default policy to be specified. “DisablePasswordExpiration” can also be specified. The two may be specified together; for example: "DisablePasswordExpiration, DisableStrongPassword".
    /// 
    /// TODO: Convert to enum
    pub password_policies: Option<String>,

    // TODO: passwordProfile

    /// A list for the user to enumerate their past projects.
    pub past_projects: Option<Vec<String>>,

    /// The postal code for the user's postal address. The postal code is specific to the user's country/region. In the United States of America, this attribute contains the ZIP code. Maximum length is 40 characters.
    pub postal_code: Option<String>,

    /// The preferred language for the user. Should follow ISO 639-1 Code; for example "en-US". Returned by default.
    pub preferred_language: Option<String>,

    /// The preferred name for the user.
    pub preferred_name: Option<String>,

    // TODO: provisionedPlans 

    /// For example: ["SMTP: bob@contoso.com", "smtp: bob@sales.contoso.com"] The any operator is required for filter expressions on multi-valued properties. Read-only, Not nullable.
    pub proxy_addresses: Option<Vec<String>>,

    /// Any refresh tokens or sessions tokens (session cookies) issued before this time are invalid, and applications will get an error when using an invalid refresh or sessions token to acquire a delegated access token (to access APIs such as Microsoft Graph). If this happens, the application will need to acquire a new refresh token by making a request to the authorize endpoint. 
    pub refresh_tokens_valid_from_date_time: Option<String>,

    /// A list for the user to enumerate their responsibilities.
    pub responsibilities: Option<Vec<String>>,

    /// A list for the user to enumerate the schools they have attended.
    pub schools: Option<Vec<String>>,

    /// true if the Outlook global address list should contain this user, otherwise false. If not set, this will be treated as true. For users invited through the invitation manager, this property will be set to false.
    pub show_in_address_list: Option<bool>,

    /// A list for the user to enumerate their skills.
    pub skills: Option<Vec<String>>,

    /// Any refresh tokens or sessions tokens (session cookies) issued before this time are invalid, and applications will get an error when using an invalid refresh or sessions token to acquire a delegated access token (to access APIs such as Microsoft Graph). If this happens, the application will need to acquire a new refresh token by making a request to the authorize endpoint. Read-only. Use revokeSignInSessions to reset.
    pub sign_in_sessions_valid_from_date_time: Option<DateTimeOffset>,

    /// The state or province in the user's address. Maximum length is 128 characters.
    pub state: Option<String>,

    /// The street address of the user's place of business. Maximum length is 1024 characters.
    pub street_address: Option<String>,

    /// The user's surname (family name or last name). Returned by default. Maximum length is 64 characters
    pub surname: String,

    /// A two letter country code (ISO standard 3166). Required for users that will be assigned licenses due to legal requirement to check for availability of services in countries. Examples include: "US", "JP", and "GB". Not nullable.
    pub usage_location: Option<String>,

    /// The user principal name (UPN) of the user. The UPN is an Internet-style login name for the user based on the Internet standard RFC 822. By convention, this should map to the user's email name. 
    /// The general format is alias@domain, where domain must be present in the tenant's collection of verified domains. This property is required when a user is created. 
    /// The verified domains for the tenant can be accessed from the verifiedDomains property of organization.
    /// NOTE: While this property can contain accent characters, they can cause access issues to first-party applications for the user.
    /// 
    /// Returned by default.
    pub user_principal_name: String,

    /// A string value that can be used to classify user types in your directory, such as "Member" and "Guest". 
    pub user_type: Option<String>,
}

/// See: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0#agegroup-values
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AgeGroup {
    Minor,
    NotAdult,
    Adult
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConsentProvidedForMinor {
    Granted,
    Denied,
    NotRequired
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LegalAgeGroupClassification {
    MinorWithOutParentalConsent, 
    MinorWithParentalConsent, 
    MinorNoParentalConsentRequired, 
    NotAdult,
    Adult,
}

/// See: https://docs.microsoft.com/en-us/graph/api/resources/objectidentity?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectIdentity {
    pub sign_in_type: String,
    pub issuer: String,
    pub issuer_assigned_id: String,
}