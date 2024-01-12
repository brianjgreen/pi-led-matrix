#[cfg(target_arch = "arm")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
#[cfg(target_arch = "arm")]
use ril::prelude::*;
#[cfg(target_arch = "arm")]
use std::cell::RefCell;

#[cfg(target_arch = "arm")]
thread_local!(static CONTROLLER: RefCell<ControllerBuilder> = RefCell::new(ControllerBuilder::new()));

#[cfg(target_arch = "arm")]
pub fn init() {
    CONTROLLER.with(|controller| controller.borrow_mut()
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
        .unwrap());
}

#[cfg(target_arch = "arm")]
pub fn render(mut image: Image<Rgba>) {
    let mut leds = CONTROLLER.borrow_mut().leds_mut(0);
    let mut i = 0;
    let mut rev_col: bool = false;
    for x in 0..32 {
        let mut y_range = [0, 1, 2, 3, 4, 5, 6, 7];
        if rev_col {
            y_range = [7, 6, 5, 4, 3, 2, 1, 0];
        }
        for y in y_range {
            leds[i] = image.pixel(x, y).as_bytes();
             i += 1;
        }
        if rev_col {
            rev_col = false;
        } else {
            rev_col = true;
        }
    }
    CONTROLLER.with(|controller| controller.borrow_mut().render().unwrap());
}

#[cfg(target_arch = "arm")]
pub fn finish() -> ril::Result<()> {
    Ok(())
}
