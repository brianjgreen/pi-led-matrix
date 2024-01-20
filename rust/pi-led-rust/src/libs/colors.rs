use ril::prelude::*;

// Name a color and return an Rgba pixel with that color
pub fn color(color_name: &str) -> Rgba {
    let mut color: Rgba = Rgba::new(0, 0, 0, 255);
    match color_name {
        "black" => color = Rgba::new(0, 0, 0, 255),
        "white" => color = Rgba::new(255, 255, 255, 255),
        "red" => color = Rgba::new(0, 0, 255, 255),
        "green" => color = Rgba::new(0, 128, 0, 255),
        "blue" => color = Rgba::new(255, 0, 0, 255),
        "orange" => color = Rgba::new(0, 165, 255, 255),
        "purple" => color = Rgba::new(128, 0, 128, 255),
        "brown" => color = Rgba::new(42, 42, 165, 255),
        "gold" => color = Rgba::new(0, 215, 255, 255),
        "gray" => color = Rgba::new(128, 128, 128, 255),
        "pink" => color = Rgba::new(203, 192, 255, 255),
        "silver" => color = Rgba::new(192, 192, 192, 255),
        "yellow" => color = Rgba::new(0, 255, 255, 255),
        _ => println!("Unknown color {}", color_name),
    }
    color
}
