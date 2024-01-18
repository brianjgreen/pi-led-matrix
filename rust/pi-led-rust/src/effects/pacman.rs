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
    let pacman_right_frame_4: Vec<Rgba> = pacman_right
        .iter()
        .map(|val| match val {
            'a' => color("yellow"),
            'b' => color("yellow"),
            'c' => color("black"),
            _ => color("black"),
        })
        .collect();
    let pac_img_r1: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_1);
    let pac_img_r2: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_2);
    let pac_img_r3: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_3);
    let pac_img_r4: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_4);
    let mut pac_img_l1: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_1);
    let mut pac_img_l2: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_2);
    let mut pac_img_l3: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_3);
    let mut pac_img_l4: Image<Rgba> = Image::from_pixels(PACMAN_COLUMNS, &pacman_right_frame_4);
    pac_img_l1.mirror();
    pac_img_l2.mirror();
    pac_img_l3.mirror();
    pac_img_l4.mirror();

    let mut x: i32 = 0;
    let mut y: u32 = 2;
    if rows > PACMAN_ROWS {
        y = rows / 2 - PACMAN_ROWS / 2;
    }
    let mut pacman_right = true;
    let mut play_clock = get_config().effects.playtime / 2;
    let mut i = 1;
    while play_clock > 0 {
        play_clock -= 1;
        let mut image: Image<Rgba> = Image::new(columns, rows, color("black"));
        if pacman_right {
            match i {
                1 => image.paste(x as u32, y, &pac_img_r1),
                2 => image.paste(x as u32, y, &pac_img_r2),
                3 => image.paste(x as u32, y, &pac_img_r3),
                4 => image.paste(x as u32, y, &pac_img_r4),
                _ => i = 1,
            }
            x += 1;
        } else {
            match i {
                1 => image.paste(x as u32, y, &pac_img_l1),
                2 => image.paste(x as u32, y, &pac_img_l2),
                3 => image.paste(x as u32, y, &pac_img_l3),
                4 => image.paste(x as u32, y, &pac_img_l4),
                _ => i = 1,
            }
            x -= 1;
        }
        if x <= 0 {
            pacman_right = true;
        }
        if x > columns as i32 - PACMAN_COLUMNS as i32 {
            pacman_right = false;
        }
        i += 1;
        if i > 4 {
            i = 1;
        }
        render(image);
    }
    Ok(())
}
