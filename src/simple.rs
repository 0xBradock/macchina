#[derive(Debug)]
pub struct Green;

#[derive(Debug)]
pub struct Yellow;

#[derive(Debug)]
pub struct Red;

#[derive(Debug)]
pub struct State<S> {
    _state: S,
}

impl State<Green> {
    pub fn new() -> State<Green> {
        State { _state: Green {} }
    }
    pub fn next(self) -> State<Yellow> {
        State { _state: Yellow {} }
    }
}

impl State<Yellow> {
    pub fn next(self) -> State<Red> {
        State { _state: Red {} }
    }
}

impl State<Red> {
    pub fn next(self) -> State<Green> {
        State { _state: Green {} }
    }
}
