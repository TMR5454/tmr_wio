use wio_terminal as wio;

use wio::hal::gpio::*;
use wio::prelude::*;

pub struct Button {
    pin: Pc26<Input<Floating>>,
}

impl Button {
    pub fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_floating_input(port),
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    pub fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}
