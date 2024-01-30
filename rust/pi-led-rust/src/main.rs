use ril::prelude::*;
mod effects;
use effects::color_test::color_test;
use effects::displayimage::displayimage;
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
        res = match e.as_str().unwrap() {
            "color_test" => color_test(),
            "displayimage" => displayimage(),
            "displaytext" => displaytext(),
            "leds_off" => leds_off(),
            "pacman" => pacman(),
            "pong" => pong(),
            "space_invaders" => space_invaders(),
            _ => color_test(),
        };
    }
    res
}
