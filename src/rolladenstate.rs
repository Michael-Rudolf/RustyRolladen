use serde::{Deserialize};
use std::process::{Command};

#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub metadata: Metadata,
}

impl RolladenState {
    pub fn retrieve_current_state() -> Option<RolladenState> {
        // Get the API Location
        let output = Command::new("cat")
            .arg("~/.config/rustyrolladen.json")
            .output()
            .expect("Config file (~/.config/rustyrolladen.json) missing.");


        let out2 = Command::new("sleep")
            .arg("0.1")
            .output();

        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub should_be_true: bool
}