use std::process::Command;
use dirs::home_dir;
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

    /// The name the API uses to communicate the amount of temperature
    #[serde(rename = "CURRENT_TEMPERATURE_NAME")]
    pub current_temperature_name: String,
}

impl Config {
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
        let file_location = home_dir.join(".config/rustyrolladen.toml");

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
        Config { api_address: self.api_address.clone(), rolladen_target_name: self.rolladen_target_name.clone(), current_light_value_name: self.current_light_value_name.clone(), debug: self.debug.clone(), release: self.release.clone()}
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile{
    /// The request delay after a request that didn't change the rolladens state.
    #[serde(rename = "STANDARD_REQUEST_DELAY")]
    pub standard_request_delay: String,

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

impl Clone for Profile{
    fn clone(&self) -> Profile{
        Profile{standard_request_delay: self.standard_request_delay.clone(), request_delay_change: self.request_delay_change.clone(), open_pin: self.open_pin.clone(), close_pin: self.close_pin.clone(), min_brightness_difference: self.min_brightness_difference.clone(), gpio_press_pin_duration: self.gpio_press_pin_duration.clone()}
    }
}