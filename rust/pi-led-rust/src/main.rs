use ril::prelude::*;
mod effects;
use crate::effects::displaytext::displaytext;
use crate::effects::pong::pong;
mod libs;
#[cfg(target_arch = "arm")]
use crate::libs::raspberrypi::render;
#[cfg(not(target_arch = "arm"))]
use crate::libs::simulation::render;

pub fn led_render(image: Image<Rgba>) {
    render(image);
}

fn main() -> ril::Result<()> {
    let _ = pong();
    let _ = displaytext();
    Ok(())
}
