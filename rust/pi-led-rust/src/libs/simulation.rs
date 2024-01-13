use ril::prelude::*;
#[cfg(not(target_arch = "arm"))]
use ril::ResizeAlgorithm::Nearest;
use std::cell::RefCell;
#[cfg(not(target_arch = "arm"))]
use super::config::get_config;

thread_local!(static SIM_GIF: RefCell<ImageSequence<Rgba>> = RefCell::new(<ril::ImageSequence<_>>::new()));

#[cfg(not(target_arch = "arm"))]
pub fn render(mut image: Image<Rgba>) {
    image.resize(get_config("columns") as u32 * 10, get_config("rows") as u32 * 10, Nearest);
    SIM_GIF.with(|output| output.borrow_mut().push_frame(image.into()));
}

#[cfg(not(target_arch = "arm"))]
pub fn finish() -> ril::Result<()> {
    return SIM_GIF.with(|output| output.borrow_mut().save_inferred("output.gif"));
}
