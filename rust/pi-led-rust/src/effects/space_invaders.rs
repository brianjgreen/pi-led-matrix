use super::super::libs::colors::color;
use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;

// Space Invaders march across your LEDs om their way to an invasion
pub fn space_invaders() -> ril::Result<()> {
    // Pixels of the aliens
    let squid = [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a',
        ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', 'a', 'a',
        'a', 'a', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ',
        ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        ' ', ' ', ' ', ' ', ' ', ' ', 'b', 'b', 'c', 'c', 'b', 'b', 'b', 'b', 'c', 'c', 'b', 'b',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b', 'b', 'c', 'c', 'b', 'b', 'b', 'b', 'c', 'c',
        'b', 'b', ' ', ' ', ' ', ' ', ' ', ' ', 'b', 'b', 'c', 'c', ' ', ' ', 'c', 'c', 'c', 'c',
        ' ', ' ', 'c', 'c', 'b', 'b', ' ', ' ', ' ', ' ', 'b', 'b', 'c', 'c', ' ', ' ', 'c', 'c',
        'c', 'c', ' ', ' ', 'c', 'c', 'b', 'b', ' ', ' ', ' ', ' ', 'c', 'c', 'b', 'b', 'c', 'c',
        ' ', ' ', ' ', ' ', 'c', 'c', 'b', 'b', 'c', 'c', ' ', ' ', ' ', ' ', 'c', 'c', 'b', 'b',
        'c', 'c', ' ', ' ', ' ', ' ', 'c', 'c', 'b', 'b', 'c', 'c', ' ', ' ',
    ];

    let crab = [
        ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ', 'a', 'a',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ',
        'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', 'c', 'c', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', 'c', 'c', 'c', 'c', ' ', ' ', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', 'c', 'c', 'c', 'c', 'a', 'a', 'a', ' ',
        ' ', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'c', 'c', 'c', 'c', 'a', 'a',
        'a', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'c', 'c', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'b', 'b', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        ' ', ' ', 'b', 'b', 'b', 'b', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', ' ', ' ', 'b', 'b', 'b', 'b', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', 'a', 'a', ' ', ' ', 'b', 'b', 'b', 'b', ' ', ' ', 'a', 'a', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', 'a', 'a', ' ', ' ', 'b', 'b', ' ', ' ', 'c', 'c', ' ', ' ', 'b', 'b',
        'b', ' ', ' ', 'b', 'b', 'b', ' ', ' ', 'c', 'c', ' ', ' ', ' ', ' ', 'c', 'c', ' ', ' ',
        'b', 'b', 'b', ' ', ' ', 'b', 'b', 'b', ' ', ' ', 'c', 'c', ' ', ' ',
    ];

    let octopus = [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ',
        ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', ' ', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', ' ', ' ', ' ', ' ', 'a', 'a', 'a', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', ' ', ' ', ' ', ' ', 'a', 'a',
        'a', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', ' ', ' ', 'a', 'a', 'a', 'a',
        ' ', ' ', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', ' ', ' ', 'a', 'a',
        'a', 'a', ' ', ' ', 'a', 'a', 'a', ' ', ' ', ' ', 'b', 'b', 'b', ' ', 'c', 'c', 'c', ' ',
        ' ', ' ', ' ', ' ', ' ', 'c', 'c', 'c', ' ', 'b', 'b', 'b', 'b', 'b', 'b', ' ', 'c', 'c',
        'c', ' ', ' ', ' ', ' ', ' ', ' ', 'c', 'c', 'c', ' ', 'b', 'b', 'b',
    ];

    const ALIEN_COLUMNS: u32 = 20;
    const ALIEN_ROWS: u32 = 16;

    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;

    // Add black and white to the alien pixels
    let squid_frame_1: Vec<Rgba> = squid
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("white"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();
    let squid_frame_2: Vec<Rgba> = squid
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("black"),
            'c' => color("white"),
            _ => color("black"),
        })
        .collect();
    let crab_frame_1: Vec<Rgba> = crab
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("white"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();
    let crab_frame_2: Vec<Rgba> = crab
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("black"),
            'c' => color("white"),
            _ => color("black"),
        })
        .collect();
    let octopus_frame_1: Vec<Rgba> = octopus
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("white"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();
    let octopus_frame_2: Vec<Rgba> = octopus
        .iter()
        .map(|val| match val {
            'a' => color("white"),
            'b' => color("black"),
            'c' => color("white"),
            _ => color("black"),
        })
        .collect();

    // Draw images of the alien animation frames
    let squid_1: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &squid_frame_1);
    let squid_2: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &squid_frame_2);
    let crab_1: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &crab_frame_1);
    let crab_2: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &crab_frame_2);
    let octopus_1: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &octopus_frame_1);
    let octopus_2: Image<Rgba> = Image::from_pixels(ALIEN_COLUMNS, &octopus_frame_2);

    let mut x: i32 = 0;
    let mut y: u32 = 0;

    let mut alien_right = true;
    let mut play_clock = get_config().effects.playtime;

    // The aliens move with only 2 animation frames, they can move in any direction with these frames
    let mut frame = [1, 2].iter().cycle();
    while play_clock > 0 {
        play_clock -= 1;
        let mut image: Image<Rgba> = Image::new(columns, rows, color("black"));
        match frame.next().unwrap() {
            1 => {
                image.paste(x as u32, y, &squid_1);
                image.paste(x as u32 + ALIEN_COLUMNS, y, &crab_1);
                image.paste(x as u32 + 2 + 2 * ALIEN_COLUMNS, y, &octopus_1)
            }
            2 => {
                image.paste(x as u32, y, &squid_2);
                image.paste(x as u32 + ALIEN_COLUMNS, y, &crab_2);
                image.paste(x as u32 + 2 + 2 * ALIEN_COLUMNS, y, &octopus_2)
            }
            _ => println!("Unknown animation frame"),
        }
        if alien_right {
            x += 1;
        } else {
            x -= 1;
        }

        // Change direction when alien reaches end of LED field
        if x <= 0 {
            alien_right = true;
            y += ALIEN_ROWS / 2;
        }
        if x > columns as i32 - ALIEN_COLUMNS as i32 {
            alien_right = false;
            y += ALIEN_ROWS / 2;
        }

        // Show next wave of aliens when previous wave drops below screen
        if y > rows {
            y = 0;
        }

        render(image);
    }
    Ok(())
}
