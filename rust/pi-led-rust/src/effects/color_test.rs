use super::super::libs::colors::color;
use super::super::libs::config::{get_config_i64, get_config_string};
use super::super::render;
use ril::prelude::*;
use ril::text::{TextLayout, TextSegment};

pub fn color_test() -> ril::Result<()> {
    let columns = get_config_i64("columns") as u32;
    let rows = get_config_i64("rows") as u32;
    let font = Font::open(get_config_string("fontpath"), 14.0).unwrap();
    let all_colors = vec![
        "white", "blue", "red", "green", "yellow", "orange", "purple", "brown", "gold",
        "gray", "pink", "silver", "black"
    ];
    for c in all_colors {
        let mut play_clock = get_config_i64("playtime") / 2;
        println!("{}", c);
        let mut layout = TextLayout::<Rgba>::new()
            .with_wrap(WrapStyle::Word)
            .with_width(columns);
        let segment = TextSegment::new(
            &font,
            c,
            color("black"),
        );
        layout.push_segment(&segment);
        while play_clock > 0 {
            play_clock -= 1;
            let mut image: Image<Rgba> = Image::new(columns, rows, color(c));
            image.draw(&layout);
            render(image);
        }
    }
    Ok(())
}
