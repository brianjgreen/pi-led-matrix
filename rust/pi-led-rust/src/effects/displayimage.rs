use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;
use ril::ResizeAlgorithm::Hamming;

// Turn off all LEDs
pub fn displayimage() -> ril::Result<()> {
    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;

    let mut image = Image::<Rgba>::open(get_config().effects.imagepath)?;

    image.resize(columns, rows, Hamming);

    let mut play_clock = get_config().effects.playtime;
    while play_clock > 0 {
        play_clock -= 1;
        render(image.clone())
    }
    Ok(())
}
