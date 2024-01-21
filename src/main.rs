mod simple;

fn main() {
    simple_fsm();
}

fn simple_fsm() {
    let state = simple::State::new(); // <- green
    let state = state.next(); // <- yellow
    let state = state.next(); // <- red
    let state = state.next(); // <- green
    let state = state.next(); // <- yellow

    println!("Simple FSM: {:?}", state); // <- yellow
}
