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

// This file only compiles on system that are NOT a Raspberry Pi including MacOS, Linux, Windows

// Thread safe static mutable buffer for the simulation window
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
    // Increase the size of the image buffer so that the simulation window is easier to see
    let columns = get_config().hardware.columns as usize * get_config().hardware.simscale as usize;
    let rows: usize = get_config().hardware.rows as usize * get_config().hardware.simscale as usize;
    image.resize(columns as u32, rows as u32, Nearest);
    let mut buffer: Vec<u32> = vec![0; 600 * 300];
    let mut i = 0;

    // Translate the image grid into a stream of Rgba u8 structures
    for y in 0..rows {
        for x in 0..columns {
            buffer[i] = u32::from_ne_bytes(image.pixel(x as u32, y as u32).as_bytes());
            i += 1;
        }
    }

    // Slow down the refresh rate to the simulation window to better mimic a Raspberry Pi driving LEDs
    WINDOW.with(|output| {
        output
            .borrow_mut()
            .limit_update_rate(Some(std::time::Duration::from_micros(
                get_config().hardware.simupdatelimit as u64,
            )))
    });

    // Copy the image buffer into the simulation window frame buiffer
    WINDOW.with(|output| {
        output
            .borrow_mut()
            .update_with_buffer(&buffer, columns, rows)
            .unwrap()
    });
}
