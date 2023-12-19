#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::pwm::Channel;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

#[entry]
fn main() -> ! {
    /* initialize clock */
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let core = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    /* initialize buzzer */
    let mut sets = Pins::new(peripherals.PORT).split();
    let mut buzzer = sets.buzzer.init(
        &mut clocks,
        peripherals.TCC0,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    let freqs = [261, 294, 329, 349, 392, 440, 494, 523]; /* C, D, E, F, G, A, B, C */
    loop {
        for freq in freqs.iter() {
            buzzer.set_period(freq.hz());
            let max_duty = buzzer.get_max_duty();
            buzzer.set_duty(Channel::_4, max_duty / 2);

            buzzer.enable(Channel::_4);
            delay.delay_ms(1000u16);
            buzzer.disable(Channel::_4);
        }
    }
}
