use embedded_graphics::{
    geometry::Size,
    pixelcolor::Rgb565,
    prelude::*,
    image::{Image, ImageRawLE},
};
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    // Create a new simulator display with the specified size
    // and pixel color type
    // The display is 320x240 pixels and uses the Rgb565 color format
    // The simulator display is a mock display used for testing
    // and simulating embedded graphics applications
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw test", &output_settings);

    let image_data = include_bytes!("assets/ferris.raw");
    let image = ImageRawLE::new(image_data, 86);
    Image::new(&image, Point::new(240,77));
    image.draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
