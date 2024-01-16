#[cfg(not(target_arch = "arm"))]
use super::config::get_config_i64;
#[cfg(not(target_arch = "arm"))]
use minifb::{Window, WindowOptions};
#[cfg(not(target_arch = "arm"))]
use ril::prelude::*;
#[cfg(not(target_arch = "arm"))]
use ril::ResizeAlgorithm::Nearest;
#[cfg(not(target_arch = "arm"))]
use std::cell::RefCell;

// thread_local!(static SIM_GIF: RefCell<ImageSequence<Rgba>> = RefCell::new(<ril::ImageSequence<_>>::new()));
#[cfg(not(target_arch = "arm"))]
thread_local!(static WINDOW: RefCell<Window> = RefCell::new(Window::new(
    "LED Simulation - ESC to exit",
    get_config_i64("columns") as usize * 10,
    get_config_i64("rows") as usize * 10,
    WindowOptions::default(),
)
.unwrap_or_else(|e| {
    panic!("{}", e);
})));

#[cfg(not(target_arch = "arm"))]
pub fn render(mut image: Image<Rgba>) {
    let columns = get_config_i64("columns") as usize * 10;
    let rows: usize = get_config_i64("rows") as usize * 10;
    image.resize(columns as u32, rows as u32, Nearest);
    // SIM_GIF.with(|output| output.borrow_mut().push_frame(image.into()));
    let mut buffer: Vec<u32> = vec![0; 600 * 300];
    let mut i = 0;
    for y in 0..rows {
        for x in 0..columns {
            buffer[i] = u32::from_ne_bytes(image.pixel(x as u32, y as u32).as_bytes());
            i += 1;
        }
    }
    WINDOW.with(|output| {
        output
            .borrow_mut()
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)))
    });
    WINDOW.with(|output| {
        output
            .borrow_mut()
            .update_with_buffer(&buffer, columns, rows)
            .unwrap()
    });
}
