use super::super::libs::colors::color;
use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;

// Turn off all LEDs
pub fn leds_off() -> ril::Result<()> {
    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;

    let image: Image<Rgba> = Image::new(columns, rows, color("black"));
    render(image);

    Ok(())
}
