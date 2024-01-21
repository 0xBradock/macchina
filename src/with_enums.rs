#[derive(Debug)]
pub struct Machine {
    _state: TrafficLight,
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
            _state: TrafficLight::Red,
        }
    }

    pub fn next(self) -> Machine {
        match self._state {
            TrafficLight::Green => Machine {
                _state: TrafficLight::Yellow,
            },
            TrafficLight::Yellow => Machine {
                _state: TrafficLight::Red,
            },
            TrafficLight::Red => Machine {
                _state: TrafficLight::Green,
            },
        }
    }
}
