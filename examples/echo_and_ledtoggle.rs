#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use cortex_m::peripheral::NVIC;
use tmr_wio::led::Led;
use wio::hal::clock::GenericClockController;
use wio::hal::hal::serial::*;
use wio::hal::timer::TimerCounter;
use wio::pac::{interrupt, Peripherals, TC3};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

struct Shared {
    led: Led,
    tc3: TimerCounter<TC3>,
}
static mut SHARED: Option<Shared> = None;

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

    /* initialize UART */
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut serial = sets.uart.init(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    /* generate 2MHz clock */
    let gclk5 = clocks
        .get_gclk(wio::pac::gclk::pchctrl::GEN_A::GCLK5)
        .unwrap();
    let timer_clock = clocks.tc2_tc3(&gclk5).unwrap();
    /* for interrupt */
    let mut tc3 = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);

    /* enable inerrupt */
    unsafe {
        NVIC::unmask(interrupt::TC3);
    }
    tc3.start(1.s());
    tc3.enable_interrupt();

    /* shared object for main and interrupt-handler */
    unsafe {
        SHARED = Some(Shared {
            led: Led::new(sets.user_led, &mut sets.port),
            tc3,
        });
    }

    loop {
        for c in b"$ ".iter() {
            nb::block!(serial.write(*c)).unwrap();
        }
        loop {
            if let Ok(c) = nb::block!(serial.read()) {
                nb::block!(serial.write(c)).unwrap();
                if c == *b"\n".get(0).unwrap() {
                    break;
                }
            }
        }
    }
}

fn led_toggle() {
    unsafe {
        let shared = SHARED.as_mut().unwrap();
        shared.tc3.wait().unwrap();
        shared.led.toggle();
    }
}

#[interrupt]
fn TC3() {
    led_toggle();
}
