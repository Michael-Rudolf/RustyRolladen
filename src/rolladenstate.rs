use serde::{Deserialize};
use std::process::{Command};
use dirs::home_dir;
use toml;

#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub metadata: Metadata,
}

impl RolladenState {
    pub fn retrieve_current_state() -> Option<RolladenState> {
        // Get the API Location
        // 1. Get the home directories location
        if home_dir().is_none(){
            panic!("Could not find home directory");
        }
        let home_dir = home_dir().unwrap();

        // 2. Get the full URL
        let file_location = home_dir.join(".config/rustyrolladen.toml");

        // 3. cat it
        let output = Command::new("cat")
            .arg(file_location.to_str().unwrap())
            .output()
            .expect("Config file (~/.config/rustyrolladen.toml) missing.");


        // 4. Turn TOML into something readable
        let config: Config = toml::from_str(&String::from_utf8_lossy(&output.stdout)).expect("Could not parse toml");



        println!("API-Address: {}", config.api_location);
        println!("Output2: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

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

    pub api_location: String,
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

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub should_be_true: bool
}