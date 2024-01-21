#![allow(dead_code)]
#![allow(unused_variables)]

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

impl Red {
    pub fn new() -> Self {
        Red {}
    }

    pub fn next(self) -> Green {
        Green {}
    }
}

impl Yellow {
    pub fn next(self) -> Red {
        Red {}
    }
}

impl Green {
    pub fn next(self) -> Yellow {
        Yellow {}
    }
}

impl From<Green> for Yellow {
    fn from(value: Green) -> Yellow {
        Yellow {}
    }
}

impl From<Yellow> for Red {
    fn from(value: Yellow) -> Red {
        Red {}
    }
}

impl From<Red> for Green {
    fn from(value: Red) -> Green {
        Green {}
    }
}
