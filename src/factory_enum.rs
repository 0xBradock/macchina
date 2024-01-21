#[derive(Debug)]
pub struct LightsMachine<S> {
    _state: S,
}

impl LightsMachine<Red> {
    fn new() -> Self {
        LightsMachine { _state: Red {} }
    }
}

#[derive(Debug)]
pub struct Green {}

#[derive(Debug)]
pub struct Yellow {}

#[derive(Debug)]
pub struct Red {}

// ✅
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

#[derive(Debug)]
pub enum LightsMachineWrapper {
    _Green(LightsMachine<Green>),
    _Yellow(LightsMachine<Yellow>),
    Red(LightsMachine<Red>),
}

impl LightsMachineWrapper {
    pub fn next(self) -> Self {
        match self {
            LightsMachineWrapper::_Green(val) => LightsMachineWrapper::_Green(val.into()),
            LightsMachineWrapper::_Yellow(val) => LightsMachineWrapper::_Yellow(val.into()),
            LightsMachineWrapper::Red(val) => LightsMachineWrapper::Red(val.into()),
        }
    }
}

#[derive(Debug)]
pub struct Factory {
    pub lights_machine: LightsMachineWrapper,
}

impl Factory {
    pub fn new() -> Self {
        Factory {
            lights_machine: LightsMachineWrapper::Red(LightsMachine::new()),
        }
    }
}
