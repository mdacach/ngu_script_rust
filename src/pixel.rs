use image::{open, Pixel, Rgb, RgbImage};
use screenshots::Screen;

use crate::coords::{InGamePosition, Position};

pub const ENEMY_BAR_RIGHT: Position = Position::from_coords(1240, 555);
pub const ENEMY_ALIVE_RGB: Rgb<u8> = Rgb([236, 52, 52]);
pub const NO_ENEMY_RGB: Rgb<u8> = Rgb([255, 255, 255]);

pub fn get_pixel_rgb(pos: InGamePosition) -> Rgb<u8> {
    let InGamePosition { x, y } = pos;
    let image = get_screenshot();

    *image.get_pixel(x.into(), y.into())
}

fn get_screenshot() -> RgbImage {
    let left_monitor = Screen::from_point(100, 100).expect("Could not find display screen");
    println!("calling capture");
    let screenshot = left_monitor.capture().expect("Could not screenshot");
    // TODO: avoid the extra write to file, we already have the image here
    std::fs::write("images/screenshot.png", screenshot.buffer())
        .expect("Could not save screenshot");

    let screenshot = open("images/screenshot.png")
        .expect("Could not open previous screenshot")
        .to_rgb8();

    screenshot
}
