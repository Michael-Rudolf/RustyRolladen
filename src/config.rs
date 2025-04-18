use std::process::Command;
use dirs::home_dir;
use serde::Deserialize;
use rand::rng;
use rand::prelude::IndexedRandom;
use log::{info};

#[derive(Debug, Deserialize)]
pub struct Config{
    /// The address of the API to call
    #[serde(rename = "API_MAIN_LOCATION")]
    pub api_address: String,

    /// The address of the API where the data is located
    #[serde(rename = "API_DATA_LOCATION")]
    pub data_address: String,

    /// The default profile if no other profile is specified, and it's not started by an autostart.
    #[serde(rename = "DEFAULT_PROFILE")]
    pub default_profile: String,

    /// The default profile for autostart
    #[serde(rename = "AUTOSTART_PROFILE")]
    pub autostart_profile: String,

    /// The name the API uses to convey the target position of the rolladen
    #[serde(rename = "ROLLADEN_TARGET_NAME")]
    pub rolladen_target_name: String,

    /// The name the API uses to communicate the temperature
    #[serde(rename = "CURRENT_TEMPERATURE_NAME")]
    pub current_temperature_name: String,

    /// The name the API uses to communicate the air pressure
    #[serde(rename = "CURRENT_PRESSURE_NAME")]
    pub current_pressure_name: String,

    /// The name the API uses to communicate the humidity
    #[serde(rename = "CURRENT_HUMIDITY_NAME")]
    pub current_humidity_name: String,

    /// The name the API uses to communicate the gas resistance
    #[serde(rename = "CURRENT_RESISTANCE_NAME")]
    pub current_gas_resistance_name: String,

    /// The name the API uses to communicate the last time the above values have been updated
    #[serde(rename = "UPDATE_DATE_NAME")]
    pub last_update_date_name: String,

    #[serde(rename = "DEBUG")]
    debug: Profile,
    #[serde(rename = "RELEASE")]
    release: Profile,
}

impl Config {
    pub fn get_profile(&self) -> Option<Profile> {
        if let Some(profile) = self.get_profile_by_name(self.default_profile.to_lowercase()){
            return Some(profile);
        }
        println!("Config named '{}' was not found", self.default_profile.to_lowercase());
        None
    }

    pub fn set_autostart_as_default(&mut self){
        self.default_profile = self.autostart_profile.to_lowercase();
    }

    pub fn get_profile_by_name(&self, name: String) -> Option<Profile>
    {
        match name.as_ref() {
            "debug" => Some(self.debug.clone()),
            "release" => Some(self.release.clone()),
            _ => None
        }
    }

    pub fn get_global_config() -> Config{
        // 1. Get the home directories location
        if home_dir().is_none(){
            panic!("Could not find home directory");
        }
        let home_dir = home_dir().unwrap();

        // 2. Get the full URL
        let file_location = home_dir.join(".config/rustyrolladen.toml");//pas

        // 3. cat it
        let output = Command::new("cat")
            .arg(file_location.to_str().unwrap())
            .output()
            .expect("Config file (~/.config/rustyrolladen.toml) missing.");


        // 4. Turn TOML into something readable
        let config: Config = toml::from_str(&String::from_utf8_lossy(&output.stdout)).expect("Could not parse toml");
        config
    }
}

impl Clone for Config {
    fn clone(&self) -> Config{
        Config { last_update_date_name: self.last_update_date_name.clone(), api_address: self.api_address.clone(), data_address: self.data_address.clone(), default_profile: self.default_profile.clone(), autostart_profile: self.autostart_profile.clone(), rolladen_target_name: self.rolladen_target_name.clone(), debug: self.debug.clone(), release: self.release.clone(), current_temperature_name: self.current_temperature_name.clone(), current_pressure_name: self.current_pressure_name.clone(), current_humidity_name: self.current_humidity_name.clone(), current_gas_resistance_name: self.current_gas_resistance_name.clone(),}
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile{
    /// The request delay after a request that didn't change the rolladens state.
    #[serde(rename = "STANDARD_REQUEST_DELAY")]
    pub standard_request_delay: String,

    /// The amount of iterations it takes until sensor data is uploaded
    #[serde(rename = "ITERATIONS_SEND_DATA")]
    pub iterations_send_data: u32,
    /// The maximal difference from the default delay between the cycles
    // This is for appearing more randomly
    #[serde(rename = "RANDOM_DELAY_DIFFERENCE_MAX")]
    pub random_delay_difference_max: String,

    /// The request delay after a request that did change the rolladens state.
    #[serde(rename = "REQUEST_DELAY_CHANGE")]
    pub request_delay_change: String,

    /// The GPIO pin number corresponding with the opening action
    #[serde(rename = "OPEN_PIN_NUMBER")]
    pub open_pin: String,

    /// The GPIO pin number corresponding with the closing action
    #[serde(rename = "CLOSE_PIN_NUMBER")]
    pub close_pin: String,

    /// Says the minimum difference in brightness required to update it.
    #[serde(rename = "BRIGHTNESS_DIFFERENCE_UPDATE")]
    pub min_brightness_difference: String,

    /// Says how long a GPIO pin will be high when a state should toggle (seconds)
    #[serde(rename = "GPIO_PIN_PRESS_DURATION")]
    pub gpio_press_pin_duration: String,
}

impl Profile{
    pub fn make_default_delay(&self) -> i32{
        let mut new_rng = rng();
        let max_change: i32 = self.random_delay_difference_max.parse::<i32>().unwrap();
        let min_change: i32 = max_change * (-1);
        let possibilities: Vec<i32> = (min_change..max_change).collect();
        let random_delay_difference = possibilities.choose(&mut new_rng);
        let default_change: i32 = self.standard_request_delay.parse::<i32>().unwrap();
        info!("Random delay difference: {}.", random_delay_difference.unwrap());
        default_change + random_delay_difference.unwrap()
    }
}

impl Clone for Profile{
    fn clone(&self) -> Profile{
        Profile{standard_request_delay: self.standard_request_delay.clone(), request_delay_change: self.request_delay_change.clone(), iterations_send_data: self.iterations_send_data, random_delay_difference_max: self.random_delay_difference_max.clone(), open_pin: self.open_pin.clone(), close_pin: self.close_pin.clone(), min_brightness_difference: self.min_brightness_difference.clone(), gpio_press_pin_duration: self.gpio_press_pin_duration.clone()}
    }
}
