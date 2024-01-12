use ril::prelude::*;

mod effects;
use crate::effects::pong::pong;
mod libs;
#[cfg(target_arch = "arm")]
use crate::libs::raspberrypi::{finish, init, render};
#[cfg(not(target_arch = "arm"))]
use crate::libs::simulation::{finish, init, render};

pub fn led_init() {
    init();
}

pub fn led_render(image: Image<Rgba>) {
    render(image);
}

pub fn led_finish() -> ril::Result<()> {
    return finish();
}

fn main() -> ril::Result<()> {
    let _ = pong();
    Ok(())
}
