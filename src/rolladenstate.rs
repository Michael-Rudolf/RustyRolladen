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
    pub current_temperature: f32,
}

impl RolladenState {

    pub fn new() -> RolladenState {
        RolladenState{ should_be_open: true, current_light_value: 1f32, current_temperature: -100.5f32 }
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
        let current_light_value = json_data[config.current_light_value_name].parse::<f32>().unwrap();
        let current_temperature = json_data[config.current_temperature_name].parse::<f32>().unwrap();

        Some(RolladenState{ should_be_open, current_light_value: 0.0})
    }

    pub fn light_significantly_different(&self, other: RolladenState, config: Config) -> bool {
        let difference = self.current_light_value - other.current_light_value;
        difference.abs() > config.debug.min_brightness_difference.parse().unwrap()
    }

    /// Publishes the temperature and the light level on the backend
    pub fn publish_state(&self, config: Config) {
        let parantacie_open = 123 as char; // {
        let parantacie_closed = 125 as char; // }
        let _ = Command::new("curl")
            .arg("-X")
            .arg("PATCH")
            .arg("-d")
            .arg(format!("'{}\"{}\": {}{}'", parantacie_open, config.current_light_value_name, self.current_light_value, parantacie_closed))
            .arg(format!("{}", config.api_address))
            .output()
            .expect("API call failed");

        let _ = Command::new("curl")
            .arg("-X")
            .arg("PATCH")
            .arg("-d")
            .arg(format!("'{}\"{}\": {}{}'", parantacie_open, config.current_temperature_name, self.current_temperature, parantacie_closed))
            .arg(format!("{}", config.api_address))
            .output()
            .expect("API call failed");




    }
}


