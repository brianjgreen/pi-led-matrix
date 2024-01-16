use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;
use ril::text::{TextLayout, TextSegment};

pub fn displaytext() -> ril::Result<()> {
    let columns = get_config("columns") as u32;
    let rows = get_config("rows") as u32;
    let mut layout = TextLayout::<Rgba>::new()
        .with_wrap(WrapStyle::Word)
        .with_width(columns);

    let font = Font::open("../../fonts/dosis.ttf", 12.0).unwrap();
    let segment = TextSegment::new(&font, "The Beverly", Rgba::new(255, 0, 0, 255));
    layout.push_segment(&segment);

    let mut play_clock = get_config("playtime");
    while play_clock > 0 {
        play_clock -= 1;
        let mut image: Image<Rgba> = Image::new(columns, rows, Rgba::new(0, 0, 0, 255));
        image.draw(&layout);
        render(image);
    }
    Ok(())
}
