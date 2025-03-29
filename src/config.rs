use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config{
    /// The address of the API to call
    #[serde(rename = "API_LOCATION")]
    pub api_address: String,

    /// The name the API uses to convey the target position of the rolladen
    #[serde(rename = "ROLLADEN_TARGET_NAME")]
    pub rolladen_target_name: String,

    /// The name the API uses to communicate the amount of light
    #[serde(rename = "CURRENT_LIGHT_VALUE_NAME")]
    pub current_light_value_name: String,

    #[serde(rename = "DEBUG")]
    pub debug: Profile,
    #[serde(rename = "RELEASE")]
    pub release: Profile,
}

#[derive(Debug, Deserialize)]
struct Profile{
    /// The request delay after a request that didn't change the rolladens state.
    #[serde(rename = "STANDARD_REQUEST_DELAY")]
    pub standard_request_delay: String,

    /// The request delay after a request that did change the rolladens state.
    #[serde(rename = "REQUEST_DELAY_CHANGE")]
    pub request_delay_change: String,
}