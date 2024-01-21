#[derive(Debug)]
pub struct LightsMachine<S> {
    _state: S,
}

#[derive(Debug)]
pub struct Green {}
#[derive(Debug)]
pub struct Yellow {}
#[derive(Debug)]
pub struct Red {}

impl LightsMachine<Red> {
    pub fn new() -> Self {
        LightsMachine { _state: Red {} }
    }
}

impl From<LightsMachine<Green>> for LightsMachine<Yellow> {
    fn from(_: LightsMachine<Green>) -> LightsMachine<Yellow> {
        LightsMachine { _state: Yellow {} }
    }
}

impl From<LightsMachine<Yellow>> for LightsMachine<Red> {
    fn from(_: LightsMachine<Yellow>) -> LightsMachine<Red> {
        LightsMachine { _state: Red {} }
    }
}

impl From<LightsMachine<Red>> for LightsMachine<Green> {
    fn from(_: LightsMachine<Red>) -> LightsMachine<Green> {
        LightsMachine { _state: Green {} }
    }
}
