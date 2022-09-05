use std::path::Path;

use lazy_static::lazy_static;

use crate::coords::GameAwareRectangle;
use crate::menu::Menu;
use crate::ocr::get_ocr_text_from_file;
use crate::{menu, pixel};

lazy_static! {
    pub static ref UNSPENT_EXP_OCR_AREA: GameAwareRectangle =
        GameAwareRectangle::from_coords(581, 90, 486, 36);
}

pub fn get_unspent_exp() -> u32 {
    let path = Path::new("images/temporary_screenshot.png");
    pixel::save_screenshot_area_to(*UNSPENT_EXP_OCR_AREA, path);
    let text = get_ocr_text_from_file(path).expect("Could not read unspent EXP");
    let mut text = text.split_ascii_whitespace();
    let amount = text.nth(3).unwrap();
    let amount: String = amount
        .chars()
        .into_iter()
        .filter(|x| x.is_digit(10))
        .collect();
    amount.parse().unwrap()
}

#[test]
fn test_ocr_unspent_exp() {
    menu::navigate(Menu::SpendEXP);
    dbg!(get_unspent_exp());
}
