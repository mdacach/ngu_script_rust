use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread;
use std::time::Duration;

use image::{open, ImageBuffer, Rgb, RgbImage};
use lazy_static::lazy_static;
use scrap::{Capturer, Display};
use screenshots::Screen;

use crate::constants::user::{Monitor, MONITOR_USED};
use crate::coords;
use crate::coords::GameAwarePosition;

lazy_static! {
    pub static ref ENEMY_BAR_RIGHT_PIXEL: GameAwarePosition =
        GameAwarePosition::from_coords(1240, 555);
    pub static ref ENEMY_BAR_LEFT_PIXEL: GameAwarePosition =
        GameAwarePosition::from_coords(981, 555);
    pub static ref IDLE_MODE_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(554, 119);
}

pub fn get_pixel_rgb(pos: GameAwarePosition) -> Rgb<u8> {
    get_pixel_from_scrap(pos)
}

pub fn approximately_equal(lhs: Rgb<u8>, rhs: Rgb<u8>) -> bool {
    let eps = 3;
    let lhs = lhs.0;
    let rhs = rhs.0;

    !lhs.iter().zip(rhs.iter()).any(|(&l, &r)| {
        let l_plus_eps = l.saturating_add(eps);
        let r_plus_eps = r.saturating_add(eps);
        r > l_plus_eps || l > r_plus_eps
    })
}

/// Returns a screenshot of leftmost display.
pub fn get_screenshot() -> RgbImage {
    get_screenshot_from_scrap()
}

pub fn save_screenshot_to(path: &Path) {
    get_screenshot()
        .save(path)
        .expect("Could not save screenshot");
}

// TODO: scrap is faster than screenshots
//       crop the image from scrap instead of capturing area from screenshots
pub fn save_screenshot_area_to(area: coords::GameAwareRectangle, path: &Path) {
    let left_monitor = Screen::from_point(100, 100).expect("Could not find display screen");
    let screenshot = left_monitor
        .capture_area(
            area.x as i32,
            area.y as i32,
            area.width as u32,
            area.height as u32,
        )
        .expect("Could not screenshot area");
    std::fs::write(path, screenshot.buffer()).expect("Could not save screenshot");
}

/// Returns a screenshot of leftmost display.
pub fn get_screenshot_area(area: coords::GameAwareRectangle) -> RgbImage {
    save_screenshot_area_to(area, Path::new("images/temporary_screenshot.png"));
    open("images/temporary_screenshot.png")
        .expect("Could not open previous screenshot")
        .to_rgb8()
}

pub fn get_pixel_from_scrap(pos: GameAwarePosition) -> Rgb<u8> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = match MONITOR_USED {
        Monitor::Primary => Display::primary().expect("Could not find display"),
        Monitor::Secondary => {
            let mut all_displays = Display::all().unwrap();
            all_displays.remove(1)
        }
    };
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

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
        let GameAwarePosition { x, y } = pos;
        let height = 1080;
        let stride = buffer.len() / height;
        let i = stride * y as usize + 4 * x as usize;

        let b = buffer[i];
        let g = buffer[i + 1];
        let r = buffer[i + 2];
        return Rgb([r, g, b]);
    }
}

/// Returns a screenshot of secondary display.
/// Requires that you have two monitors.
pub fn get_screenshot_from_scrap() -> RgbImage {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = match MONITOR_USED {
        Monitor::Primary => Display::primary().expect("Could not find display"),
        Monitor::Secondary => {
            let mut all_displays = Display::all().unwrap();
            all_displays.remove(1)
        }
    };
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

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
        let width = 1920;
        let height = 1080;
        let mut bitflipped = Vec::with_capacity(width * height * 3);
        let stride = buffer.len() / height;
        for y in 0..height {
            for x in 0..width {
                let i = stride * y + 4 * x;
                bitflipped.push(buffer[i + 2]);
                bitflipped.push(buffer[i + 1]);
                bitflipped.push(buffer[i]);
            }
        }

        let raw_data = bitflipped;
        return ImageBuffer::from_raw(width as u32, height as u32, raw_data).unwrap();
    }
}
