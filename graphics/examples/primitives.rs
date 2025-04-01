use embedded_graphics::{
    geometry::Size,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::*,
    text::Text,
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

    /* line */
    let start = Point::new(50, 20);
    let end = Point::new(270, 220);
    let style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
    Line::new(start, end)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    /*  */
    // Rectangle with red 3 pixel wide stroke and green fill with the top left corner at (30, 20) and
    // a size of (10, 15)
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(30)
        .fill_color(Rgb565::GREEN)
        .build();

    Rectangle::new(Point::new(30, 20), Size::new(10, 15))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // Rectangle with translation applied
    Rectangle::new(Point::new(30, 20), Size::new(10, 15))
        .translate(Point::new(100, -10))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // Circle with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
    Circle::new(Point::new(150, 120), 30)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)?;

    // Circle with styled stroke and fill with top-left point at (50, 20) with a diameter of 30
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .fill_color(Rgb565::GREEN)
        .build();

    Circle::new(Point::new(50, 200), 10)
        .into_styled(style)
        .draw(&mut display)?;

    // Circle with blue fill and no stroke with a translation applied
    Circle::new(Point::new(10, 200), 30)
        .translate(Point::new(20, 10))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display)?;

    // Ellipse with 1 pixel wide white stroke with top-left point at (10, 20) with a size of (30, 40)
    Ellipse::new(Point::new(80, 120), Size::new(30, 40))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)?;

    // Ellipse with styled stroke and fill with top-left point at (20, 30) with a size of (40, 30)
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .fill_color(Rgb565::GREEN)
        .build();

    Ellipse::new(Point::new(120, 90), Size::new(40, 30))
        .into_styled(style)
        .draw(&mut display)?;

    // Ellipse with blue fill and no stroke with a translation applied
    Ellipse::new(Point::new(10, 90), Size::new(20, 40))
        .translate(Point::new(10, -15))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display)?;

    // Arc with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
    Arc::new(Point::new(250, 20), 30, 0.0.deg(), 90.0.deg())
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)?;

    // Arc with styled stroke with top-left point at (15, 25) with a diameter of 20
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .build();

    Arc::new(Point::new(250, 125), 80, 180.0.deg(), -90.0.deg())
        .into_styled(style)
        .draw(&mut display)?;

    // Sector with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
    Sector::new(Point::new(10, 20), 30, 0.0.deg(), 90.0.deg())
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)?;

    // Sector with styled stroke and fill with top-left point at (10, 20) with a diameter of 30
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .fill_color(Rgb565::GREEN)
        .build();

    Sector::new(Point::new(10, 20), 30, 180.0.deg(), -90.0.deg())
        .into_styled(style)
        .draw(&mut display)?;

    // Sector with blue fill and no stroke with a translation applied
    Sector::new(Point::new(10, 20), 30, 0.0.deg(), 90.0.deg())
        .translate(Point::new(15, 5))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display)?;

    Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(&mut display)?;

    // Triangle with translation applied
    Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
        .translate(Point::new(-10, -20))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
        .draw(&mut display)?;

    // A "heartbeat" shaped polyline
    let points: [Point; 10] = [
        Point::new(10, 64),
        Point::new(50, 64),
        Point::new(60, 44),
        Point::new(70, 64),
        Point::new(80, 64),
        Point::new(90, 74),
        Point::new(100, 10),
        Point::new(110, 84),
        Point::new(120, 64),
        Point::new(300, 64),
    ];

    let line_style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 5);

    Polyline::new(&points)
        .into_styled(line_style)
        .draw(&mut display)?;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(5)
        .stroke_color(Rgb565::RED)
        .fill_color(Rgb565::GREEN)
        .build();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(5, 5), Size::new(40, 50)),
        Size::new(10, 10),
    )
    .into_styled(style)
    .draw(&mut display)?;

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::new("Hello Rust!", Point::new(180, 180), style).draw(&mut display)?;

    window.show_static(&display);
    Ok(())
}
