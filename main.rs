use serde::{Deserialize};
use serde_json;
use std::process::{Command};
use log::error;


use crate::config::Config;


#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub should_be_open: bool,
    pub current_temperature: f32,
}

impl RolladenState {

    pub fn new() -> RolladenState {
        RolladenState{ should_be_open: true, current_temperature: -100.5f32 }
    }
    pub fn retrieve_current_state(config: Config) -> Option<RolladenState> {
        // Now call the API and retrieve its data
        // 1. Start a process with the API call
        let api_result = Command::new("curl")
            .arg(format!("{}", config.api_address))
            .output()
            .expect("API call failed");

        let api_result_output = String::from_utf8_lossy(&*api_result.stdout);
        let json_data: serde_json::Value = serde_json::from_str(&*api_result_output).expect("JSON parse error");

        let should_be_open = json_data[config.rolladen_target_name].as_bool().unwrap();
        let current_temperature = json_data[config.current_temperature_name].as_f64().unwrap() as f32;

        Some(RolladenState{ should_be_open, current_temperature})
    }

    pub fn light_significantly_different(&self, other: RolladenState, config: Config) -> bool {
        let difference = self.current_light_value - other.current_light_value;
        difference.abs() > config.get_profile().unwrap().min_brightness_difference.parse().unwrap()
    }

    /// Publishes the temperature and the light level on the backend
    pub fn publish_state(&self, config: Config) {
        let json_data = format!(r#"{{"{}": {}}}"#, config.current_temperature_name, self.current_temperature);
        let output = Command::new("curl")
            .arg("-X")
            .arg("PATCH")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(json_data)
            .arg(config.api_address.clone())
            .output()
            .expect("API call failed");

        if !output.status.success(){
            error!("Requesting change of data failed with error: {:?}", output.stderr);
        }

    }
}


