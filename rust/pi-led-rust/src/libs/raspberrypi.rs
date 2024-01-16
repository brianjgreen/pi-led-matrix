#[cfg(target_arch = "arm")]
use super::config::get_config;
#[cfg(target_arch = "arm")]
use ril::prelude::*;
#[cfg(target_arch = "arm")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
#[cfg(target_arch = "arm")]
use std::cell::RefCell;

#[cfg(target_arch = "arm")]
thread_local!(static CONTROLLER: RefCell<Controller> = RefCell::new(ControllerBuilder::new().dma(10)
.channel(
    0, // Channel Index
    ChannelBuilder::new()
        .pin(get_config("pin") as i32) // GPIO 18 = PWM0
        .count(get_config("columns") as i32 * get_config("rows") as i32) // Number of LEDs
        .strip_type(StripType::Ws2812)
        .brightness(get_config("brightness") as u8) // default: 255
        .build(),
)
.build()
.unwrap()));

#[allow(unused_mut)]
#[cfg(target_arch = "arm")]
pub fn render(mut image: Image<Rgba>) {
    let columns = get_config("columns");
    let rows: usize = get_config("rows") as usize;
    let mut y_range_forward: Vec<usize> = Vec::new();
    for y in 0..rows {
        y_range_forward.push(y);
    }
    let mut y_range_reverse = y_range_forward.clone();
    y_range_reverse.reverse();
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
    CONTROLLER.with(|controller| controller.borrow_mut().render().unwrap());
}
