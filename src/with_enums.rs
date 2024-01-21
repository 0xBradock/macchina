#[derive(Debug)]
pub struct Machine {
    state: TrafficLight,
}

#[derive(Debug)]
pub enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            state: TrafficLight::Red,
        }
    }

    pub fn next(self) -> Machine {
        match self.state {
            TrafficLight::Green => Machine {
                state: TrafficLight::Yellow,
            },
            TrafficLight::Yellow => Machine {
                state: TrafficLight::Red,
            },
            TrafficLight::Red => Machine {
                state: TrafficLight::Green,
            },
        }
    }
}
