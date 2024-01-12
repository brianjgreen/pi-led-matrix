use super::super::{finish, init, render};
use rand::prelude::*;
use ril::draw::{Line, Rectangle};
use ril::prelude::*;

fn move_paddle(paddle_y: u32, ball_y: u32) -> u32 {
    let mut new_paddle_y: u32 = paddle_y;
    let paddle_midpoint = 3;
    let paddle_height = 5; // should always be an odd number
    let y_max: u32 = 30 - 1;

    if ball_y > paddle_y + paddle_midpoint {
        if paddle_y < y_max - paddle_height - 1 {
            new_paddle_y += 1;
        }
    } else if paddle_y > 1 {
        new_paddle_y -= 1;
    }
    new_paddle_y
}

pub fn pong() -> ril::Result<()> {
    let paddle_height = 5; // should always be an odd number
    let mut x_vector: bool = true;
    let mut y_vector: bool = true;
    let x_min: u32 = 3;
    let y_min: u32 = 0;
    let x_max: u32 = 60 - 4;
    let y_max: u32 = 30 - 1;
    let y_midpoint = 15;

    let mut rng = thread_rng();
    let mut x: u32 = rng.gen_range(x_min..=x_max);
    let mut y: u32 = rng.gen_range(y_min..=y_max);

    // Paddles
    let mut left_paddle_y = 15;
    let mut right_paddle_y = 15;

    // Don't start the ball inside the wall
    if y == 0 || y == y_max {
        y = 1;
    }

    init();
    loop {
        let mut image: Image<Rgba> = Image::new(60, 30, Rgba::new(0, 0, 0, 255));

        if x_vector {
            if x + 1 <= x_min || x + 1 >= x_max {
                x_vector = false;
            }
        } else {
            if x - 1 <= x_min || x - 1 >= x_max {
                x_vector = true;
            }
        }

        if y_vector {
            if y + 1 <= y_min || y + 1 >= y_max {
                y_vector = false;
            }
        } else {
            if y - 1 <= y_min || y - 1 >= y_max {
                y_vector = true;
            }
        }

        if x_vector {
            x += 1;
        } else {
            x -= 1;
        }
        if y_vector {
            y += 1;
        } else {
            y -= 1;
        }

        // Move paddles to meet the ball
        if x <= x_min + y_midpoint + paddle_height {
            left_paddle_y = move_paddle(left_paddle_y, y);
        }

        if x >= x_max - y_midpoint - paddle_height {
            right_paddle_y = move_paddle(right_paddle_y, y);
        }

        // Draw top and bottom walls
        let line = Line::new((x_min + 1, 0), (x_max, 0), Rgba::white());
        image.draw(&line);
        let line = Line::new((x_min + 1, y_max), (x_max + 1, y_max), Rgba::white());
        image.draw(&line);

        // Draw dashed line in middle of field
        for i in 1..7 {
            let rectangle: Rectangle<Rgba> = Rectangle::at(29, i * 4)
                .with_size(1, 2)
                .with_fill(Rgba::white());
            image.draw(&rectangle);
        }

        // Draw ball
        let rectangle: Rectangle<Rgba> =
            Rectangle::at(x, y).with_size(1, 1).with_fill(Rgba::white());
        image.draw(&rectangle);

        // Draw paddles
        let rectangle: Rectangle<Rgba> = Rectangle::at(2, left_paddle_y)
            .with_size(2, 5)
            .with_fill(Rgba::white());
        image.draw(&rectangle);

        let rectangle: Rectangle<Rgba> = Rectangle::at(x_max, right_paddle_y)
            .with_size(2, 5)
            .with_fill(Rgba::white());
        image.draw(&rectangle);

        render(image);
    }
    let _ = finish();
    Ok(())
}
