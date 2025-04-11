#![no_std]
#![no_main]

extern crate panic_halt;

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::gpio::*;
use wio::hal::hal::spi;
use wio::hal::prelude::*;
use wio::hal::sercom::*;
use wio::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut sets = wio::Pins::new(peripherals.PORT).split();

    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    let gclk0 = &clocks.gclk0();
    let mut spi: SPIMaster7<
        Sercom7Pad2<Pb18<PfD>>,
        Sercom7Pad3<Pb19<PfD>>,
        Sercom7Pad1<Pb20<PfD>>,
    > = SPIMaster7::new(
        &clocks.sercom7_core(&gclk0).unwrap(),
        8_000_000.hz(),
        spi::MODE_0,
        peripherals.SERCOM7,
        &mut peripherals.MCLK,
        (
            sets.display.miso.into_pad(&mut sets.port),
            sets.display.mosi.into_pad(&mut sets.port),
            sets.display.sck.into_pad(&mut sets.port),
        ),
    );

    let mut cs = sets.display.cs.into_push_pull_output(&mut sets.port);
    let mut dc = sets.display.dc.into_push_pull_output(&mut sets.port);
    let mut reset = sets.display.reset.into_push_pull_output(&mut sets.port);

    reset.set_low().unwrap();
    delay.delay_ms(1u16);
    reset.set_high().unwrap();
    delay.delay_ms(120u16);
    cs.set_low().unwrap();
    dc.set_low().unwrap();
    spi.write(&[0x0A]).unwrap();

    let mut args = [0x00]; // dummy write
    dc.set_high().unwrap();
    let mode = spi.transfer(&mut args).unwrap();
    cs.set_high().unwrap();

    writeln!(&mut serial, "Display Power Mode: {:#04x}", mode[0]).unwrap();

    loop {}
}
