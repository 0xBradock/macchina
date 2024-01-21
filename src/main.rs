use from_trait::Green;

use crate::from_trait::{LightsMachine, Red, Yellow};
mod from_trait;
mod simple;
mod with_enums;

fn main() {
    simple_fsm();
    with_enums();
    from_trait();
}

fn from_trait() {
    let red = LightsMachine::<Red>::new(); // <- red
    let green = LightsMachine::<Green>::from(red); // <- green
    let state = LightsMachine::<Yellow>::from(green); // <- yellow

    println!("From trait: {:?}", state); // <- yellow
}

fn with_enums() {
    let state = with_enums::Machine::new(); // <- Red
    let state = state.next(); // <- Green
    let state = state.next(); // <- Yellow
    let state = state.next(); // <- Red

    println!("With Enum FSM: {:?}", state); // <- yellow
}

fn simple_fsm() {
    let state = simple::State::new(); // <- green
    let state = state.next(); // <- yellow
    let state = state.next(); // <- red
    let state = state.next(); // <- green
    let state = state.next(); // <- yellow

    println!("Simple FSM: {:?}", state); // <- yellow
}
