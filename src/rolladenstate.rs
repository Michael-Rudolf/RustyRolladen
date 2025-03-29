use serde::{Deserialize};
use serde_json;
use std::process::{Command};
use dirs::home_dir;
use toml;


use crate::config::Config;


#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub should_be_open: bool,
    pub current_light_value: f32,
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

        // Now call the API and retrieve its data
        // 1. Start a process with the API call
        let api_result = Command::new("curl")
            .arg(format!("{}", config.api_address))
            .output()
            .expect("API call failed");

        let api_result_output = String::from_utf8_lossy(&*api_result.stdout);;
        let json_data: serde_json::Value = serde_json::from_str(&*api_result_output).expect("JSON parse error");

        let should_be_open = json_data[config.rolladen_target_name].as_bool().unwrap();

        Some(RolladenState{ should_be_open, current_light_value: 0.0})
    }
}


