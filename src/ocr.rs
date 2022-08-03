use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::coords::GameAwareRectangle;
use crate::pixel;

pub fn get_ocr_text(path: &Path) -> Option<String> {
    let mut ocr = leptess::tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptess::leptonica::pix_read(path).unwrap();
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
    let path = Path::new("images/temporary_screenshot.png");
    pixel::save_screenshot_area_to(adventure_zone_name, path);
    dbg!(get_ocr_text(path));
}

#[test]
fn test_ocr_unspent_exp() {
    let x = 581;
    let y = 90;
    let width = 486;
    let height = 36;
    let unspent_exp = GameAwareRectangle::from_coords(x, y, width, height);
    let path = Path::new("images/temporary_screenshot.png");
    pixel::save_screenshot_area_to(unspent_exp, path);
    dbg!(get_ocr_text(path));
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
    let path = Path::new("images/temporary_screenshot.png");
    pixel::save_screenshot_area_to(next_itopod_rewards, path);
    dbg!(get_ocr_text(path));
}
