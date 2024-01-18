#[cfg(not(target_arch = "arm"))]
use super::config::get_config;
#[cfg(not(target_arch = "arm"))]
use minifb::{Window, WindowOptions};
#[cfg(not(target_arch = "arm"))]
use ril::prelude::*;
#[cfg(not(target_arch = "arm"))]
use ril::ResizeAlgorithm::Nearest;
#[cfg(not(target_arch = "arm"))]
use std::cell::RefCell;

#[cfg(not(target_arch = "arm"))]
thread_local!(static WINDOW: RefCell<Window> = RefCell::new(Window::new(
    "LED Simulation",
    get_config().hardware.columns as usize * get_config().hardware.simscale as usize,
    get_config().hardware.rows as usize * get_config().hardware.simscale as usize,
    WindowOptions::default(),
)
.unwrap_or_else(|e| {
    panic!("{}", e);
})));

#[cfg(not(target_arch = "arm"))]
pub fn render(mut image: Image<Rgba>) {
    let columns = get_config().hardware.columns as usize * get_config().hardware.simscale as usize;
    let rows: usize = get_config().hardware.rows as usize * get_config().hardware.simscale as usize;
    image.resize(columns as u32, rows as u32, Nearest);
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
