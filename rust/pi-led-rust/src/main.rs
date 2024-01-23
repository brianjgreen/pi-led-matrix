use ril::prelude::*;
mod effects;
use effects::color_test::color_test;
use effects::displaytext::displaytext;
use effects::leds_off::leds_off;
use effects::pacman::pacman;
use effects::pong::pong;
use effects::space_invaders::space_invaders;
mod libs;
#[cfg(target_arch = "arm")]
use crate::libs::raspberrypi::render;
use libs::config::get_config;
#[cfg(not(target_arch = "arm"))]
use libs::simulation::render;

pub fn led_render(image: Image<Rgba>) {
    render(image);
}

fn main() -> ril::Result<()> {
    let mut res = Ok(());
    for e in get_config().effects.playlist {
        match e.as_str().unwrap() {
            "color_test" => res = color_test(),
            "displaytext" => res = displaytext(),
            "leds_off" => res = leds_off(),
            "pacman" => res = pacman(),
            "pong" => res = pong(),
            "space_invaders" => res = space_invaders(),
            _ => println!("Unknown effect {}", e),
        };
    }
    res
}
