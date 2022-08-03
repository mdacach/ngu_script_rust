use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::coords::GameAwareRectangle;
use crate::pixel::get_screenshot_area;

pub fn get_ocr_text(path_as_str: &str) -> Option<String> {
    let mut ocr = leptess::tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptess::leptonica::pix_read(Path::new(path_as_str)).unwrap();
    ocr.set_image(&pix);

    if let Ok(text) = ocr.get_utf8_text() {
        Some(text)
    } else {
        None
    }
}

#[test]
fn test_ocr_adventure_zone() {
    let x = 1006;
    let y = 268;
    let width = 200;
    let height = 30;
    let adventure_zone_name = GameAwareRectangle::from_coords(x, y, width, height);
    get_screenshot_area(adventure_zone_name);
    dbg!(get_ocr_text("images/screenshot.png"));
}

#[test]
fn test_ocr_unspent_exp() {
    let x = 581;
    let y = 90;
    let width = 486;
    let height = 36;
    let unspent_exp = GameAwareRectangle::from_coords(x, y, width, height);
    get_screenshot_area(unspent_exp);
    dbg!(get_ocr_text("images/screenshot.png"));
}

#[test]
fn test_ocr_next_itopod_rewards() {
    let x = 699;
    let y = 56;
    let width = 352;
    let height = 155;
    let move_mouse_x = 663;
    let move_mouse_y = 29;
    let mouse_position = crate::coords::GameAwarePosition::from_coords(move_mouse_x, move_mouse_y);
    crate::input::mouse_move(mouse_position);
    thread::sleep(Duration::from_secs(2));
    let next_itopod_rewards = GameAwareRectangle::from_coords(x, y, width, height);
    get_screenshot_area(next_itopod_rewards);
    dbg!(get_ocr_text("images/screenshot.png"));
}
