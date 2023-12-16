#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::pac::Peripherals;

use tmr_wio::button::*;
use tmr_wio::led::Led;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = Led::new(pins.user_led, &mut pins.port);
    let button1 = Button1::new(pins.button1, &mut pins.port);
    let button2 = Button2::new(pins.button2, &mut pins.port);
    let button3 = Button3::new(pins.button3, &mut pins.port);

    loop {
        if button1.is_pressed() || button2.is_pressed() || button3.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}
