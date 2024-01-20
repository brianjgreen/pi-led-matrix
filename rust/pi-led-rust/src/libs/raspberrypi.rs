#[cfg(target_arch = "arm")]
use super::config::get_config;
#[cfg(target_arch = "arm")]
use ril::prelude::*;
#[cfg(target_arch = "arm")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
#[cfg(target_arch = "arm")]
use std::cell::RefCell;

// This file only compiles on a Raspberry Pi (or if you configured cross compile on another system)

// Thread safe static mutable memory location that writes directly to the LED driver
#[cfg(target_arch = "arm")]
thread_local!(static CONTROLLER: RefCell<Controller> = RefCell::new(ControllerBuilder::new().dma(10)
.channel(
    0, // Channel Index
    ChannelBuilder::new()
        .pin(get_config_i64("pin") as i32) // GPIO 18 = PWM0
        .count(get_config().hardware.columns as i32 * get_config().hardware.rows as i32) // Number of LEDs
        .strip_type(StripType::Ws2812)
        .brightness(get_config().hardware.brightness as u8) // default: 255
        .build(),
)
.build()
.unwrap()));

#[allow(unused_mut)]
#[cfg(target_arch = "arm")]
pub fn render(mut image: Image<Rgba>) {
    // Write the LED pixel buffer to the LED driver memory
    let columns = get_config().hardware.columns;
    let rows: usize = get_config().hardware.rows as usize;

    // The LEDs are expected to be layed out in a verticle zig zag pattern
    // so we need to reverse every other column of pixels to compensate
    let mut y_range_forward: Vec<usize> = Vec::new();
    for y in 0..rows {
        y_range_forward.push(y);
    }
    let mut y_range_reverse = y_range_forward.clone();
    y_range_reverse.reverse();
    
    // Copy the image buffer into the LED driver memory buffer
    CONTROLLER.with(|controller| {
        let mut binding = controller.borrow_mut();
        let mut leds = binding.leds_mut(0);
        let mut rev_col: bool = false;
        let mut i = 0;
        for x in 0..columns {
            let mut y_range = y_range_forward.clone();
            if rev_col {
                y_range = y_range_reverse.clone();
            }
            for y in y_range {
                leds[i] = image.pixel(x as u32, y as u32).as_bytes();
                i += 1;
            }
            if rev_col {
                rev_col = false;
            } else {
                rev_col = true;
            }
        }
    });

    // Order the LED driver to send the signal to the LEDs
    CONTROLLER.with(|controller| controller.borrow_mut().render().unwrap());
}
