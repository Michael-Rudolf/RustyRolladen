use crate::rolladenstate::RolladenState;
use rppal::gpio::Gpio;
use crate::config::Config;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
use systemd_journal_logger::JournalLog;
use log::{info, error, warn};
use std::env::args;

mod rolladenstate;
mod config;

fn main() {
    let mut config = Config::get_global_config();
    make_config_fit_args(&mut config);
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    info!("Loaded configuartion.");

    /// The expected current state
    // Especially in the beginning, this might be wrong if the generated state != the actual state
    let mut current_state = RolladenState::new();

    loop{
        // 0. Retrieve the state and update
        info!("Starting check at {}!", String::from_utf8_lossy(&*Command::new("date").output().unwrap().stdout));
        let mut did_change = false;
        let target_rolladen_state = RolladenState::retrieve_current_state(config.clone()).unwrap();
        // 1. Handle target state changes
        if target_rolladen_state.should_be_open != current_state.should_be_open {
            did_change = true;
            if target_rolladen_state.should_be_open {
                open_rolladen(config.clone(), &mut current_state);
                info!("Opened rolladen at {}.", String::from_utf8_lossy(&*Command::new("date").output().unwrap().stdout));
            }else{
                close_rolladen(config.clone(), &mut current_state);
                println!("Closed rolladen at {}.", String::from_utf8_lossy(&*Command::new("date").output().unwrap().stdout));
            }
        }
        // 2. Handle light & temperature changes
        // 3. Wait however long required
        if did_change{
            sleep(Duration::from_secs(config.get_profile().unwrap().make_default_delay() as u64));
        }else {
            sleep(Duration::from_secs(config.get_profile().unwrap().standard_request_delay.parse().unwrap()));
        }
    }
}

fn make_config_fit_args(config: &mut Config) {
    let args: Vec<String> = args().collect();
    if args.contains(&"--autostart".to_string()){
        config.set_autostart_as_default();
    }else if args.contains(&"--debug".to_string()){
        config.default_profile = String::from("debug");
    }else if args.contains(&"--release".to_string()){
        config.default_profile = String::from("release");
    }
}

fn open_rolladen(config:  Config, current_state:  &mut RolladenState) {
    let pin_number = config.get_profile().unwrap().open_pin;

    toggle_gpio_pin(pin_number.parse().unwrap(), config.get_profile().unwrap().gpio_press_pin_duration.parse().unwrap());
    current_state.should_be_open = true;
}

fn close_rolladen(config:  Config, current_state:  &mut RolladenState) {
    let pin_number = config.get_profile().unwrap().close_pin;

    toggle_gpio_pin(pin_number.parse().unwrap(), config.get_profile().unwrap().gpio_press_pin_duration.parse().unwrap());
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