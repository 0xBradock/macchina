#![allow(dead_code)]
#![allow(private_interfaces)]

#[derive(Debug)]
struct LightsMachine<S> {
    state: S,
}

impl LightsMachine<Red> {
    fn new() -> Self {
        LightsMachine { state: Red {} }
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
        LightsMachine { state: Yellow {} }
    }
}

impl From<LightsMachine<Yellow>> for LightsMachine<Red> {
    fn from(_: LightsMachine<Yellow>) -> LightsMachine<Red> {
        LightsMachine { state: Red {} }
    }
}

impl From<LightsMachine<Red>> for LightsMachine<Green> {
    fn from(_: LightsMachine<Red>) -> LightsMachine<Green> {
        LightsMachine { state: Green {} }
    }
}

#[derive(Debug)]
pub enum LightsMachineWrapper {
    Green(LightsMachine<Green>),
    Yellow(LightsMachine<Yellow>),
    Red(LightsMachine<Red>),
}

impl LightsMachineWrapper {
    pub fn next(self) -> Self {
        match self {
            LightsMachineWrapper::Green(val) => LightsMachineWrapper::Green(val.into()),
            LightsMachineWrapper::Yellow(val) => LightsMachineWrapper::Yellow(val.into()),
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
