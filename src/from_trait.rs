#![allow(dead_code)]

#[derive(Debug)]
pub struct LightsMachine<S> {
    state: S,
}

#[derive(Debug)]
pub struct Green {}
#[derive(Debug)]
pub struct Yellow {}
#[derive(Debug)]
pub struct Red {}

pub enum State {
    Green(Green),
    Yellow(Yellow),
    Red(Red),
}

impl LightsMachine<Red> {
    pub fn new() -> Self {
        LightsMachine { state: Red {} }
    }
}

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
