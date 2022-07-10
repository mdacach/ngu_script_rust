use image::{open, ImageBuffer, Rgb, RgbImage};
use lazy_static::lazy_static;
use screenshots::Screen;

use crate::coords::GameAwarePosition;

lazy_static! {
    pub static ref ENEMY_BAR_RIGHT_PIXEL: GameAwarePosition =
        GameAwarePosition::from_coords(1240, 555);
    pub static ref ENEMY_BAR_LEFT_PIXEL: GameAwarePosition =
        GameAwarePosition::from_coords(984, 555);
    pub static ref IDLE_MODE_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(554, 119);
}

pub const ENEMY_ALIVE_RGB: Rgb<u8> = Rgb([236, 52, 52]);
pub const NO_ENEMY_RGB: Rgb<u8> = Rgb([255, 255, 255]);
pub const IDLE_MODE_ON_RGB: Rgb<u8> = Rgb([255, 235, 4]);

pub fn get_pixel_rgb(pos: GameAwarePosition) -> Rgb<u8> {
    let GameAwarePosition { x, y } = pos;
    let image = get_screenshot_from_scrap();

    *image.get_pixel(x.into(), y.into())
}

/// Returns a screenshot of leftmost display.
/// TODO: This is probably having a memory leak somewhere, investigate.
fn get_screenshot() -> RgbImage {
    let left_monitor = Screen::from_point(100, 100).expect("Could not find display screen");
    let screenshot = left_monitor.capture().expect("Could not screenshot");
    // TODO: avoid the extra write to file, we already have the image here
    std::fs::write("images/screenshot.png", screenshot.buffer())
        .expect("Could not save screenshot");

    open("images/screenshot.png")
        .expect("Could not open previous screenshot")
        .to_rgb8()
}

/// Returns a screenshot of secondary display.
/// Requires that you have two monitors.
pub fn get_screenshot_from_scrap() -> RgbImage {
    use scrap::{Capturer, Display};
    use std::io::ErrorKind::WouldBlock;
    use std::thread;
    use std::time::Duration;

    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let mut all_displays = Display::all().expect("Could not find all displays.");
    let secondary = all_displays.remove(1);
    let mut capturer = Capturer::new(secondary).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        // Flip the ARGB image into a RGB image.
        // It seems  ^  this is wrong actually, more like the original image was ABGR
        let mut bitflipped = Vec::with_capacity(w * h * 3);
        let stride = buffer.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.push(buffer[i + 2]);
                bitflipped.push(buffer[i + 1]);
                bitflipped.push(buffer[i]);
            }
        }

        let raw_data = bitflipped;
        return ImageBuffer::from_raw(w as u32, h as u32, raw_data).unwrap();
    }
}
