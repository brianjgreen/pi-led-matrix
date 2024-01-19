use super::super::libs::colors::color;
use super::super::libs::config::get_config;
use super::super::render;
use ril::prelude::*;

pub fn pacman() -> ril::Result<()> {
    let pacman_right = [
        ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'b', 'b', ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b',
        'b', 'b', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'b', ' ', 'a', 'a',
        'a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c', 'a', 'a', 'a', 'a', 'a', 'b', 'b',
        'c', 'c', 'c', 'c', 'c', 'c', 'a', 'a', 'a', 'a', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c',
        'c', 'a', 'a', 'a', 'a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'c', 'c', 'a', 'a', 'a', 'a',
        'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b',
        'b', 'b', 'b', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', ' ', ' ',
        ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'a',
        'a', 'a', 'a', ' ', ' ', ' ', ' ',
    ];

    const PACMAN_COLUMNS: u32 = 13;
    const PACMAN_ROWS: u32 = 13;

    let ghost_agro = [
        ' ', ' ', ' ', ' ', ' ', 'a', 'a', 'a', 'a', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ', ' ',
        ' ', ' ', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', ' ', ' ',
        ' ', 'a', 'a', 'a', 'b', 'b', 'a', 'a', 'a', 'a', 'b', 'b', 'a', ' ',
        ' ', 'a', 'a', 'b', 'b', 'b', 'b', 'a', 'a', 'b', 'b', 'b', 'b', ' ',
        ' ', 'a', 'a', 'b', 'b', 'c', 'c', 'a', 'a', 'b', 'b', 'c', 'c', ' ',
        'a', 'a', 'a', 'b', 'b', 'c', 'c', 'a', 'a', 'b', 'b', 'c', 'c', 'a',
        'a', 'a', 'a', 'a', 'b', 'b', 'a', 'a', 'a', 'a', 'b', 'b', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
        'a', 'a', 'e', 'a', 'f', 'a', 'e', 'e', 'a', 'f', 'a', 'e', 'a', 'a',
        'f', 'e', 'e', 'd', 'f', 'f', 'e', 'e', 'f', 'f', 'd', 'e', 'e', 'f'
    ];

    let ghost_scared = [
        ' ', ' ', ' ', ' ', ' ', 'c', 'c', 'c', 'c', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', ' ', ' ', ' ',
        ' ', ' ', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', ' ', ' ',
        ' ', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', ' ',
        ' ', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', ' ',
        ' ', 'c', 'c', 'b', 'b', 'c', 'c', 'c', 'c', 'b', 'b', 'c', 'c', ' ',
        'c', 'c', 'c', 'b', 'b', 'c', 'c', 'c', 'c', 'b', 'b', 'c', 'c', 'c',
        'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c',
        'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c',
        'c', 'c', 'b', 'b', 'c', 'c', 'b', 'b', 'c', 'c', 'b', 'b', 'c', 'c',
        'c', 'b', 'c', 'c', 'b', 'b', 'c', 'c', 'b', 'b', 'c', 'c', 'b', 'c',
        'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c',
        'c', 'c', 'g', 'c', 'h', 'c', 'g', 'g', 'c', 'h', 'c', 'g', 'c', 'c',
        'h', 'g', 'g', 'd', 'h', 'h', 'g', 'g', 'h', 'h', 'd', 'g', 'g', 'h',
    ];

    const GHOST_COLUMNS: u32 = 14;
    // const GHOST_ROWS: u32 = 14;

    let columns = get_config().hardware.columns as u32;
    let rows = get_config().hardware.rows as u32;

    let pacman_right_frame_1: Vec<Rgba> = pacman_right
        .iter()
        .map(|val| match val {
            'a' => color("yellow"),
            'b' => color("yellow"),
            'c' => color("yellow"),
            _ => color("black"),
        })
        .collect();
    let pacman_right_frame_2: Vec<Rgba> = pacman_right
        .iter()
        .map(|val| match val {
            'a' => color("yellow"),
            'b' => color("yellow"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();
    let pacman_right_frame_3: Vec<Rgba> = pacman_right
        .iter()
        .map(|val| match val {
            'a' => color("yellow"),
            'b' => color("black"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();

    let ghost_agro_frame_1: Vec<Rgba> = ghost_agro
        .iter()
        .map(|val| match val {
            'a'=> color("red"),
            'b' => color("white"),
            'c' => color("blue"),
            'd' => color("black"),
            'e' => color("black"),
            'f' => color("red"),
            'g' => color("black"),
            'h' => color("blue"),
            _ => color("black"),
        })
        .collect();

        let ghost_agro_frame_2: Vec<Rgba> = ghost_agro
        .iter()
        .map(|val| match val {
            'a'=> color("red"),
            'b' => color("white"),
            'c' => color("blue"),
            'd' => color("black"),
            'e' => color("red"),
            'f' => color("black"),
            'g' => color("blue"),
            'h' => color("black"),
            _ => color("black"),
        })
        .collect();

        let ghost_scared_frame_1: Vec<Rgba> = ghost_scared
        .iter()
        .map(|val| match val {
            'a'=> color("red"),
            'b' => color("white"),
            'c' => color("blue"),
            'd' => color("black"),
            'e' => color("black"),
            'f' => color("red"),
            'g' => color("black"),
            'h' => color("blue"),
            _ => color("black"),
        })
        .collect();

        let ghost_scared_frame_2: Vec<Rgba> = ghost_scared
        .iter()
        .map(|val| match val {
            'a'=> color("red"),
            'b' => color("white"),
            'c' => color("blue"),
            'd' => color("black"),
            'e' => color("red"),
            'f' => color("black"),
            'g' => color("blue"),
            'h' => color("black"),
            _ => color("black"),
        })
        .collect();

    let pac_img_right_1: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_1);
    let pac_img_right_2: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_2);
    let pac_img_right_3: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_3);
    let mut pac_img_left_1: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_1);
    let mut pac_img_left_2: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_2);
    let mut pac_img_left_3: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_3);
    pac_img_left_1.mirror();
    pac_img_left_2.mirror();
    pac_img_left_3.mirror();

    let ghost_img_agro_1: Image<Rgba> = Image::from_pixels(GHOST_COLUMNS, &ghost_agro_frame_1);
    let ghost_img_agro_2: Image<Rgba> = Image::from_pixels(GHOST_COLUMNS, &ghost_agro_frame_2);
    let ghost_img_scared_1: Image<Rgba> = Image::from_pixels(GHOST_COLUMNS, &ghost_scared_frame_1);
    let ghost_img_scared_2: Image<Rgba> = Image::from_pixels(GHOST_COLUMNS, &ghost_scared_frame_2);

    let mut x: i32 = 0;
    let mut y: u32 = 2;
    if rows > PACMAN_ROWS {
        y = rows / 2 - PACMAN_ROWS / 2;
    }
    let mut pacman_right = true;
    let mut play_clock = get_config().effects.playtime;
    let mut frame = [1, 2, 3, 4].iter().cycle();
    while play_clock > 0 {
        play_clock -= 1;
        let mut image: Image<Rgba> = Image::new(columns, rows, color("black"));
        if pacman_right {
            match frame.next().unwrap() {
                1 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_right_1); image.paste(x as u32, y, &ghost_img_agro_1)},
                2 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_right_2); image.paste(x as u32, y, &ghost_img_agro_2)},
                3 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_right_3); image.paste(x as u32, y, &ghost_img_agro_1)},
                4 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_right_2); image.paste(x as u32, y, &ghost_img_agro_2)},
                _ => println!("Unknown animation frame"),
            }
            x += 1;
        } else {
            match frame.next().unwrap() {
                1 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_left_1); image.paste(x as u32, y, &ghost_img_scared_1)},
                2 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_left_2); image.paste(x as u32, y, &ghost_img_scared_2)},
                3 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_left_3); image.paste(x as u32, y, &ghost_img_scared_1)},
                4 => {image.paste(x as u32 + 2 + GHOST_COLUMNS, y, &pac_img_left_2); image.paste(x as u32, y, &ghost_img_scared_2)},
                _ => println!("Unknown animation frame"),
            }
            x -= 1;
        }
        if x <= 0 {
            pacman_right = true;
        }
        if x > columns as i32 - PACMAN_COLUMNS as i32 {
            pacman_right = false;
        }
        render(image);
    }
    Ok(())
}
