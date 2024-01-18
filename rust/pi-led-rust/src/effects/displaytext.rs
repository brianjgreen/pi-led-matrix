use super::super::libs::colors::color;
use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;
use ril::text::{TextLayout, TextSegment};

pub fn displaytext() -> ril::Result<()> {
    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;
    let mut layout = TextLayout::<Rgba>::new()
        .with_wrap(WrapStyle::Word)
        .with_width(columns);
    let font = Font::open(get_config().effects.fontpath, 12.0).unwrap();
    let segment = TextSegment::new(&font, get_config().effects.message, color("blue"));
    layout.push_segment(&segment);

    let mut play_clock = get_config().effects.playtime;
    while play_clock > 0 {
        play_clock -= 1;
        let mut image: Image<Rgba> = Image::new(columns, rows, color("black"));
        image.draw(&layout);
        render(image);
    }
    Ok(())
}
