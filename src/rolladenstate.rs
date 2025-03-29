use serde::{Deserialize};
use std::process::{Command};
use dirs::home_dir;

#[derive(Deserialize, Debug)]
pub struct RolladenState {
    pub metadata: Metadata,
}

impl RolladenState {
    pub fn retrieve_current_state() -> Option<RolladenState> {
        // Get the API Location
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



        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Output2: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub should_be_true: bool
}