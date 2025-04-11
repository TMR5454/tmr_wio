use embedded_graphics::{
    geometry::Size,
    pixelcolor::Rgb565,
    prelude::*,
    image::ImageRawLE,
};
use embedded_graphics_simulator::*;
use wio_splash::WioSplash;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw test", &output_settings);

    let image_data = include_bytes!("assets/ferris.raw");
    let image = ImageRawLE::new(image_data, 86);
    let splash = WioSplash::new(Rgb565::GREEN, image);
    splash.draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
