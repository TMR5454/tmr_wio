use wio_terminal as wio;

use wio::hal::gpio::*;
use wio::prelude::*;

pub struct Led {
    pin: Pa15<Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_push_pull_output(port),
        }
    }
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
