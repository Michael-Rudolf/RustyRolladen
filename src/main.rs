use crate::rolladenstate::RolladenState;
use rppal::gpio::Gpio;
use crate::config::Config;
use std::thread::sleep;
use std::time::Duration;

mod rolladenstate;
mod config;

fn main() {
    let config = Config::get_global_config();

    /// The expected current state
    // Especially in the beginning, this might be wrong if the generated state != the actual state
    let mut current_state = RolladenState::new();

    // For testing purposes only: change some values on the api
    let mut testing_state = RolladenState::new();
    testing_state.current_temperature = 16.16;
    testing_state.current_light_value = 32.32;
    testing_state.publish_state(config.clone());

    loop{
        // 0. Retrieve the state and update
        let mut did_change = false;
        let target_rolladen_state = RolladenState::retrieve_current_state(config.clone()).unwrap();
        // 1. Handle target state changes
        if target_rolladen_state.should_be_open != current_state.should_be_open {
            did_change = true;
            if target_rolladen_state.should_be_open {
                open_rolladen(config.clone(), &mut current_state);
            }else{
                close_rolladen(config.clone(), &mut current_state);
            }
        }
        // 2. Handle light value changes
        // 3. Wait however long required
        if did_change{
            sleep(Duration::from_secs(config.debug.request_delay_change.parse().unwrap()));
        }else {
            sleep(Duration::from_secs(config.debug.standard_request_delay.parse().unwrap()));
        }
    }
}

fn open_rolladen(config:  Config, current_state:  &mut RolladenState) {
    let pin_number = config.debug.open_pin;

    toggle_gpio_pin(pin_number.parse().unwrap(), config.debug.gpio_press_pin_duration.parse().unwrap());
    current_state.should_be_open = true;
}

fn close_rolladen(config:  Config, current_state:  &mut RolladenState) {
    let pin_number = config.debug.close_pin;

    toggle_gpio_pin(pin_number.parse().unwrap(), config.debug.gpio_press_pin_duration.parse().unwrap());
    current_state.should_be_open = false;
}

fn toggle_gpio_pin(pin_number: u8, seconds: u8){
    let mut pin = Gpio::new().expect("Failed to access GPIO")
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_output();

    pin.set_high();

    sleep(Duration::from_secs(seconds as u64));

    pin.set_low();
}