# Raspberry Pi LED Matrix in Rust

This is a LED matrix library written in Rust that simplifies driving a LED matrix of any size created with WS2812B LED strips.  It also is a virtual LED matrix simulator you can use to help create effects and animations without the need for a physical matrix. 

This is a fork of Nate Lewis' Raspberry Pi LED Matrix for Python
https://github.com/natelewis/pi-led-matrix
<BR>See Nate's project for the hardware details.

## Quick Start
### Simulator on MacOS, Linux, and Windows
1. Install Rust:<BR>
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. Clone this git repo:<BR>
```git clone https://github.com/brianjgreen/pi-led-matrix.git```
3. Change to the rust directory:<BR>
```cd pi-led-matrix/rust```
4. Compile a simple "Hello World!" example Rust project:<BR>
```cargo run```
5. Change to the pi-led-rust project:<BR>
```cd pi-led-rust```
6. Compile the pi-led-rust project (pi-led-matrix/rust/pi-led-rust):<BR>
```cargo build```
7. Run the pi-led-rust project simulator:<BR>
```cargo run```
8. Share and Enjoy!

### Raspberry Pi with LEDs
*Tested on Raspberry Pi Zero and 3B*
1. Install Rust:<BR>
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
1. Clone this git repo:<BR>
```git clone https://github.com/brianjgreen/pi-led-matrix.git```
1. Change to the rust directory:<BR>
```cd pi-led-matrix/rust```
1. Compile a simple "Hello World!" example Rust project:<BR>
```cargo run```
1. Change to the pi-led-rust project:<BR>
```cd pi-led-rust```
1. Install the Clang and LLVM development tools:<BR>
```sudo apt install libclang-dev```
1. Enable the PWM signal, add or change these in `/boot/config.txt`:<BR>
```
dtoverlay=pwm
dtparam=audio=off # Edit from dtparam=audio=on default
```
8. Compile the pi-led-rust project (pi-led-matrix/rust/pi-led-rust):<BR>
```cargo build```
9. Run the pi-led-rust project on the LEDs:<BR>
```cargo run```
10. Share and Enjoy!


## Configuration file (config.toml)

### Example configuration file
```
# config.toml

[hardware]
columns = 60
rows = 30
# pin is GPIO number, not physical pin number
pin = 18
brightness = 20

[effects]
# playtime is number of frames rendered
playtime = 100
fontpath = "../../fonts/dosis.ttf"
message = "Rust RPi LEDs"
```

## Common API Calls

### get_config()
```
get_config().hardware.columns
```
Use the configuration variables from the config.toml file.

### color
```
color()
```
Creates a pixel of the names color.<BR>["white", "blue", "red", "green", "yellow", "orange", "purple", "brown", "gold", "gray", "pink", "silver", "black"]

### render
```
render()
```
`render()` writes the buffer to the LEDs or simulator.

## Rust Imaging Library Common API Calls

### LED Field (or simulator field)
```
let mut image: Image<Rgba> = Image::new(columns, rows, color("black));
image.draw(&layout);
```
`Image::new` creates a buffer for the color of each LED.<BR>
`image.draw()` adds shapes, text, or other images to the buffer.<BR>

### Text
```
let font = Font::open(get_config_string("fontpath"), 14.0).unwrap();
let mut layout = TextLayout::<Rgba>::new()
    .with_wrap(WrapStyle::Word)
    .with_width(columns);
let segment = TextSegment::new(
    &font,
    "Your string here",
    color("black"),
);
image.draw(&layout);
```
`TextLayout::<Rgba>::new()` sets up the layout of the text.<BR>
`TextSegment::new()` places the text into the layout.

### Circle
*TODO using ellipse() call.*

### Line
```
let line = Line::new((x_min + 1, 0), (x_max, 0), color("white"));
image.draw(&line);
```
Draw a line from (x1, y1) to (x2, y2) with color.

### Rectangle
```
let rectangle: Rectangle<Rgba> = Rectangle::at(2, left_paddle_y)
    .with_size(2, 5)
    .with_fill(color("white"));
image.draw(&rectangle);
```
Draw a rectangle with top left (x, y) of size(x, y) with color.

### 
## References
- https://github.com/natelewis/pi-led-matrix
- https://www.rust-lang.org/tools/install
- https://github.com/rpi-ws281x/rpi-ws281x-rust
- https://github.com/jgarff/rpi_ws281x
- https://docs.rs/ril/latest/ril/draw/index.html
