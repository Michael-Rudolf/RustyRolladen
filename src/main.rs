use crate::rolladenstate::RolladenState;
use rppal::gpio::Gpio;
use std::process::Command;
mod rolladenstate;
mod config;

fn main() {
    println!("Hello, world!");

    let current_rolladen_state = RolladenState::retrieve_current_state().unwrap();

    println!("{:?}", current_rolladen_state.should_be_open);//asdf

    let pin_number = 17;

    // Get access to the GPIO pin
    let mut pin = Gpio::new().expect("Failed to access GPIO")
        .get(pin_number)
        .expect("Failed to get GPIO pin")
        .into_output();  // Set it as an output pin

    pin.set_high();
    let output = Command::new("sleep")
        .arg("5")
        .output();

    pin.set_low();
}
