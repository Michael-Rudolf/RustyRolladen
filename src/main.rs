use crate::rolladenstate::RolladenState;

mod rolladenstate;

fn main() {
    println!("Hello, world!");

    _ = RolladenState::retrieve_current_state();
}
