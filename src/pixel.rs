use std::path::Path;

use image::{open, ImageBuffer, Rgb, RgbImage};
use lazy_static::lazy_static;
use screenshots::Screen;

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
    let GameAwarePosition { x, y } = pos;
    let image = get_screenshot();

    *image.get_pixel(x.into(), y.into())
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
    save_screenshot_to(Path::new("images/temporary_screenshot.png"));
    open("images/temporary_screenshot.png")
        .expect("Could not open previous screenshot")
        .to_rgb8()
}

pub fn save_screenshot_to(path: &Path) {
    let left_monitor = Screen::from_point(100, 100).expect("Could not find display screen");
    let screenshot = left_monitor.capture().expect("Could not screenshot");
    std::fs::write(path, screenshot.buffer()).expect("Could not save screenshot");
}

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
