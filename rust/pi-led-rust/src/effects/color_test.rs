use super::super::libs::colors::color;
use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;
use ril::text::{TextLayout, TextSegment};

// Turn all LEDs the same color and display the color name
pub fn color_test() -> ril::Result<()> {
    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;
    let font = Font::open(get_config().effects.fontpath, 14.0).unwrap();
    let all_colors = vec![
        "white", "blue", "red", "green", "yellow", "orange", "purple", "brown", "gold", "gray",
        "pink", "silver", "black",
    ];
    for c in all_colors {
        let mut play_clock = get_config().effects.playtime;
        println!("{}", c);
        let mut layout = TextLayout::<Rgba>::new()
            .with_wrap(WrapStyle::Word)
            .with_width(columns);
        let segment = TextSegment::new(&font, c, color("black"));
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
