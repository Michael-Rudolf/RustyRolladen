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

    pub fn new() -> RolladenState {
        RolladenState{ should_be_open: true, current_light_value: 1f32}
    }
    pub fn retrieve_current_state(config: Config) -> Option<RolladenState> {
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

    pub fn light_significantly_different(&self, other: RolladenState, config: Config) -> bool {
        let difference = self.current_light_value - other.current_light_value;
        difference.abs() > config.debug.min_brightness_difference.parse().unwrap()
    }
}


