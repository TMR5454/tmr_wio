#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    let button1 = pins.button1.into_floating_input(&mut pins.port);
    let button2 = pins.button2.into_floating_input(&mut pins.port);
    let button3 = pins.button3.into_floating_input(&mut pins.port);

    loop {
        if button1.is_low().unwrap() || button2.is_low().unwrap() || button3.is_low().unwrap() {
            led.set_high().unwrap()
        } else {
            led.set_low().unwrap()
        }
    }
}
