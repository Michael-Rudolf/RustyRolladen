use crate::rolladenstate::RolladenState;
use rppal::gpio::Gpio;
use linux_embedded_hal::{I2cdev, Delay};
use crate::config::Config;
use std::thread::sleep;
use std::time::Duration;
use systemd_journal_logger::JournalLog;
use log::info;
use std::env::args;
use bme680::{Bme680, I2CAddress, PowerMode, SettingsBuilder};
mod rolladenstate;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut config = Config::get_global_config();
    make_config_fit_args(&mut config);
   
    {
        // Close the rolladen because something might go wrong aftwerwards
        // and the rolladen is better off closed than opened in case the config file corrupted /
        // the internet connection is bad or something similar.
        
        let mut throwaway = RolladenState::new();
        close_rolladen(config.clone(), &mut throwaway);
    }
 

    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    info!("Loaded configuration and selected {} profile.", config.default_profile);

    // Create an I2C interface and set the sensor address
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let mut delay = Delay;
    let mut sensor = Bme680::init(i2c, &mut delay, I2CAddress::Primary).unwrap();
    
    // Set up sensor configuration
    let settings = SettingsBuilder::new()
        .with_temperature_oversampling(bme680::OversamplingSetting::OS8x)
        .with_humidity_oversampling(bme680::OversamplingSetting::OS2x)
        .with_pressure_oversampling(bme680::OversamplingSetting::OS4x)
        .with_gas_measurement(Duration::from_millis(400), 320, 25)
        .build();

    let _ = sensor.set_sensor_settings(&mut delay, settings);
    info!("Set sensor (bme680) up");

    let mut did_change = false;
    let mut current_state = RolladenState::new();
    let mut iterations_since_data_send: u32 = 1;
    loop{
        // 0. Retrieve the state and update
        info!("Starting check");
        let target_rolladen_state = RolladenState::retrieve_current_state(config.clone()).unwrap();
        
        // 1. Handle target state changes
        if target_rolladen_state.should_be_open != current_state.should_be_open {
            did_change = true;
            if target_rolladen_state.should_be_open {
                open_rolladen(config.clone(), &mut current_state);
                info!("Opened rolladen at");
            }else{
                close_rolladen(config.clone(), &mut current_state);
                info!("Closed rolladen");
            }
        }
        
        // 2. Handle light & temperature changes
        let _ = sensor.set_sensor_mode(&mut delay, PowerMode::ForcedMode);
        sleep(Duration::from_millis(1000));
        let (data, _condition) = sensor.get_sensor_data(&mut delay).unwrap();
        current_state.current_temperature = data.temperature_celsius();
        current_state.current_pressure = data.pressure_hpa();
        current_state.current_humidity = data.humidity_percent();
        current_state.current_gas_resistance = data.gas_resistance_ohm() as f32;
        info!("G-Resistance: {}", current_state.current_gas_resistance);
        if iterations_since_data_send >= config.get_profile().unwrap().iterations_send_data{
            // Read the data whenever to keep sensor warm, only send sometimes to reduce bandwidth
            current_state.publish_state(config.clone());
            iterations_since_data_send = 0;
        }
        // 3. Wait however long required
        if did_change{
            sleep(Duration::from_secs(config.get_profile().unwrap().request_delay_change.parse::<u64>().unwrap()));
        }else {
            sleep(Duration::from_secs(config.get_profile().unwrap().make_default_delay() as u64));
        }
        
        // 4. Reset the did_change value & increment the counter
        did_change = false;
        iterations_since_data_send += 1;
    }
}

fn make_config_fit_args(config: &mut Config) {
    let args: Vec<String> = args().collect();
    if args.contains(&"--autostart".to_string()){
        config.set_autostart_as_default();
        // For some reason, parsing json only works later, so wait 10 secs.
        sleep(Duration::from_secs(10));
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
