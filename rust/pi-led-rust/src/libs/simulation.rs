use ril::prelude::*;
use ril::ResizeAlgorithm::Nearest;
use std::cell::RefCell;

thread_local!(static SIM_GIF: RefCell<ImageSequence<Rgba>> = RefCell::new(<ril::ImageSequence<_>>::new()));

pub fn init() {
    println!("See output.gif")
}

pub fn render(mut image: Image<Rgba>) {
    image.resize(600, 300, Nearest);
    SIM_GIF.with(|output| output.borrow_mut().push_frame(image.into()));
}

pub fn finish() -> ril::Result<()> {
    return SIM_GIF.with(|output| output.borrow_mut().save_inferred("output.gif"));
}
