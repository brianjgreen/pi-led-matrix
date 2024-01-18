use libs::config::get_config;
use ril::prelude::*;
mod effects;
use crate::effects::color_test::color_test;
use crate::effects::displaytext::displaytext;
use crate::effects::pacman::pacman;
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
    let mut res = Ok(());
    for e in get_config().effects.playlist {
        match e.as_str() {
            Some("color_test") => res = color_test(),
            Some("displaytext") => res = displaytext(),
            Some("pacman") => res = pacman(),
            Some("pong") => res = pong(),
            _ => println!("Unknown effect {}", e),
        };
    }
    res
}
