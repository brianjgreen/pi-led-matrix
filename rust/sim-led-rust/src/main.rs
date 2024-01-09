use ril::draw::Rectangle;
use ril::prelude::*;
use ril::ResizeAlgorithm::Nearest;

fn main() -> ril::Result<()> {
    let mut output = ImageSequence::<Rgba>::new();
    for i in 0..10 {
        let mut image: Image<Rgba> = Image::new(60, 30, Rgba::new(0, 0, i * 20 as u8, 128));

        let rectangle: Rectangle<Rgba> = Rectangle::at((i * 2).into(), (i * 2).into())
            .with_size(4, 4)
            .with_fill(Rgba::white());
        image.draw(&rectangle);

        image.resize(600, 300, Nearest);
        output.push_frame(image.into());
    }
    output.save_inferred("output.gif")?;
    Ok(())
}
