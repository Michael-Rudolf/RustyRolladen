use crate::rolladenstate::RolladenState;

mod rolladenstate;

fn main() {
    println!("Hello, world!");

    let current_rolladen_state = RolladenState::retrieve_current_state().unwrap();

    println!("{:?}", current_rolladen_state.should_be_open);//asdf
}
