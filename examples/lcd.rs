#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use embedded_graphics::{
    fonts::*,
    image::{Image, ImageRawLE},
    pixelcolor::*,
    prelude::*,
    primitives::*,
    style::*,
};

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::prelude::*;
use wio::pac::{CorePeripherals, Peripherals};
use wio::{entry, Pins};

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

    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58_000_000.hz(),
            &mut delay,
        )
        .unwrap();

    // fill black
    let style = PrimitiveStyleBuilder::new()
        .fill_color(eg::pixelcolor::Rgb565::BLACK)
        .build();
    let background = Rectangle::new(Point::zero(), Point::new(320, 240)).into_styled(style);
    background.draw(&mut display).unwrap();

    // draw red square
    let style = PrimitiveStyleBuilder::new()
        .fill_color(eg::pixelcolor::Rgb565::RED)
        .build();

    Text::new("Hello World!", Point::new(0, 0))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    let image_data = include_bytes!("../graphics/examples/assets/ferris.raw");
    let image = ImageRawLE::new(image_data, 86, 64);
    let (w, h) = display.size().into();
    Image::new(
        &image,
        Point::new((w / 2) as i32 - 86 / 2, (h / 2) as i32 - 64 / 2),
    )
    .draw(&mut display)
    .unwrap();

    loop {}
}
