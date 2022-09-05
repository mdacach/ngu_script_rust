use std::path::Path;

use crate::coords::GameAwareRectangle;
use crate::pixel;

pub fn get_ocr_text_from_file(path: &Path) -> Option<String> {
    let mut ocr = leptess::tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptess::leptonica::pix_read(path).unwrap();
    ocr.set_image(&pix);

    if let Ok(text) = ocr.get_utf8_text() {
        Some(text)
    } else {
        None
    }
}

pub fn get_ocr_text_from_area(area: GameAwareRectangle) -> Option<String> {
    let path = Path::new("images/temporary_ocr_screenshot.png");
    pixel::save_screenshot_area_to(area, path);

    get_ocr_text_from_file(path)
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
    dbg!(get_ocr_text_from_file(path));
}

#[test]
fn test_ocr_next_itopod_rewards() {
    crate::input::click_at(*crate::constants::itopod::coords::GET_TOOLTIP_PIXEL);
    std::thread::sleep(std::time::Duration::from_secs(2));
    dbg!(get_ocr_text_from_area(
        *crate::constants::itopod::areas::ITOPOD_TOOLTIP_OCR_AREA
    ));
}
