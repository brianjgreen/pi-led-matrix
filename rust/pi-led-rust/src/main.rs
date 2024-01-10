#[cfg(target_arch = "arm")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType} ;
use ril::draw::Rectangle;
use ril::prelude::*;
use ril::ResizeAlgorithm::Nearest;

// use std::time::Duration;
use std::{thread, time};

mod effects;
use crate::effects::pong::pong;

#[cfg(target_arch = "arm")]
fn draw_color_field(controller: &mut Controller, color: [u8; 4], wait_time: Duration) {
    let leds = controller.leds_mut(0);
    for led in leds {
        *led = color;
    }
    controller.render().unwrap();
    thread::sleep(wait_time);
}

#[cfg(target_arch = "arm")]
fn main() {
    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(12) // GPIO 18 = PWM0
                .count(256) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(20) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let two_secs = time::Duration::from_millis(2000);

    loop {
        draw_color_field(&mut controller, [0, 0, 255, 0], two_secs);
        draw_color_field(&mut controller, [0, 255, 0, 0], two_secs);
        draw_color_field(&mut controller, [255, 0, 0, 0], two_secs);
        draw_color_field(&mut controller, [0, 0, 0, 0], two_secs);
    }
}

#[cfg(not(target_arch = "arm"))]
fn main() -> ril::Result<()> {
    pong();
    /*
    let mut output = ImageSequence::<Rgba>::new();
    for i in 0..10 {
        let mut image: Image<Rgba> = Image::new(60, 30, Rgba::new(0, 0, i * 20 as u8, 128));

        let rectangle: Rectangle<Rgba> = Rectangle::at((i * 2).into(), (i * 2).into())
            .with_size(4, 4)
            .with_fill(Rgba::white());
        image.draw(&rectangle);

        image.resize(600, 300, Nearest);
        output.push_frame(image.into());
    }
    output.save_inferred("output.gif")?;
    */
    Ok(())
}
