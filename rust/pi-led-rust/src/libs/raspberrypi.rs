#[cfg(target_arch = "arm")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
#[cfg(target_arch = "arm")]
use ril::prelude::*;
#[cfg(target_arch = "arm")]
use std::cell::RefCell;
#[cfg(target_arch = "arm")]
use super::config::get_hardware;

#[cfg(target_arch = "arm")]
thread_local!(static CONTROLLER: RefCell<Controller> = RefCell::new(ControllerBuilder::new().dma(10)
.channel(
    0, // Channel Index
    ChannelBuilder::new()
        .pin(get_hardware("pin")) // GPIO 18 = PWM0
        .count(get_hardware("columns") * get_hardware("rows")) // Number of LEDs
        .strip_type(StripType::Ws2812)
        .brightness(get_hardware("brightness")) // default: 255
        .build(),
)
.build()
.unwrap()));

#[allow(unused_mut)]
#[cfg(target_arch = "arm")]
pub fn render(mut image: Image<Rgba>) {
    CONTROLLER.with(|controller| {
        let mut binding = controller.borrow_mut();
        let mut leds = binding.leds_mut(0);
        let mut rev_col: bool = false;
        let mut i = 0;
        for x in 0..60 {
            let mut y_range = [0, 1, 2, 3,4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
            if rev_col {
                y_range = [29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];
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
    });
    CONTROLLER.with(|controller| controller.borrow_mut().render().unwrap());
}

#[cfg(target_arch = "arm")]
pub fn finish() -> ril::Result<()> {
    Ok(())
}
