use wio_terminal as wio;

use wio::hal::gpio::*;
use wio::prelude::*;

pub trait Button {
    fn is_pressed(&self) -> bool;
    fn is_released(&self) -> bool;
}

pub struct Button1 {
    pin: Pc26<Input<Floating>>,
}

impl Button1 {
    pub fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_floating_input(port),
        }
    }
}

impl Button for Button1 {
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

pub struct Button2 {
    pin: Pc27<Input<Floating>>,
}

impl Button2 {
    pub fn new(pin: Pc27<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_floating_input(port),
        }
    }
}

impl Button for Button2 {
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

pub struct Button3 {
    pin: Pc28<Input<Floating>>,
}

impl Button3 {
    pub fn new(pin: Pc28<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_floating_input(port),
        }
    }
}

impl Button for Button3 {
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}
