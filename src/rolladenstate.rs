use serde::{Deserialize};
use serde_json;
use std::process::{Command};
use log::{info, error};


use crate::config::Config;


#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub should_be_open: bool,
    pub current_temperature: f32,
    pub current_humidity: f32,
    pub current_pressure: f32,
    pub current_gas_resistance: f32,
}

impl RolladenState {

    pub fn new() -> RolladenState {
        RolladenState{ should_be_open: true, current_temperature: -100.5f32, current_humidity: -100.5f32, current_pressure: -100.5f32, current_gas_resistance: -98.5f32 }
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
        //let current_temperature = json_data[config.current_temperature_name].as_f64().unwrap() as f32;

        Some(RolladenState{ should_be_open, current_temperature: -98.5f32, current_humidity: -98.5f32, current_pressure: -98.5f32, current_gas_resistance: -98.5f32}) 
    }


    /// Publishes the temperature and the light level on the backend
    pub fn publish_state(&self, config: Config) {
        let json_data = format!(r#"{{"{}": {}, "{}": {}, "{}": {}, "{}": {}}}"#, config.current_temperature_name, self.current_temperature, config.current_gas_resistance_name, self.current_gas_resistance, config.current_humidity_name, self.current_humidity, config.current_pressure_name, self.current_pressure);
        info!("Trying to update server data (sending json: {}).", json_data);

        let output = Command::new("curl")
            .arg("-X")
            .arg("PATCH")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(json_data)
            .arg(config.data_address.clone())
            .output()
            .expect("API call failed");

        if !output.status.success(){
            error!("Requesting change of data failed with error: {:?}", output.stderr);
        }else{
            info!("Update seems to have succeeded");
        }

    }
}
