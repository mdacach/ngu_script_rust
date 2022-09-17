use image::Rgb;

use crate::*;

pub fn add_augment(number: u8) {
    match number {
        0 => input::click_at(*constants::augments::coords::AUG_0_ADD),
        1 => input::click_at(*constants::augments::coords::AUG_1_ADD),
        2 => input::click_at(*constants::augments::coords::AUG_2_ADD),
        3 => input::click_at(*constants::augments::coords::AUG_3_ADD),
        4 => input::click_at(*constants::augments::coords::AUG_4_ADD),
        _ => {}
    }
}

pub fn is_capped_augment(number: u8) -> bool {
    let is_white = |pos| {
        let color = pixel::get_pixel_rgb(pos);
        color == Rgb([255, 255, 255])
    };
    match number {
        0 => !is_white(*constants::augments::coords::AUG_0_CAP),
        1 => !is_white(*constants::augments::coords::AUG_1_CAP),
        2 => !is_white(*constants::augments::coords::AUG_2_CAP),
        3 => !is_white(*constants::augments::coords::AUG_3_CAP),
        4 => !is_white(*constants::augments::coords::AUG_4_CAP),
        _ => true,
    }
}

pub fn is_capped_enhancement(number: u8) -> bool {
    let is_white = |pos| {
        let color = pixel::get_pixel_rgb(pos);
        color == Rgb([255, 255, 255])
    };
    match number {
        0 => !is_white(*constants::augments::coords::ENH_0_CAP),
        1 => !is_white(*constants::augments::coords::ENH_1_CAP),
        2 => !is_white(*constants::augments::coords::ENH_2_CAP),
        3 => !is_white(*constants::augments::coords::ENH_3_CAP),
        4 => !is_white(*constants::augments::coords::ENH_4_CAP),
        _ => true,
    }
}

pub fn add_enhancement(number: u8) {
    match number {
        0 => input::click_at(*constants::augments::coords::ENH_0_ADD),
        1 => input::click_at(*constants::augments::coords::ENH_1_ADD),
        2 => input::click_at(*constants::augments::coords::ENH_2_ADD),
        3 => input::click_at(*constants::augments::coords::ENH_3_ADD),
        4 => input::click_at(*constants::augments::coords::ENH_4_ADD),
        _ => {}
    }
}

#[test]
fn test_menu() {
    menu::navigate(menu::Menu::Augmentation);
    add_augment(3);
    add_enhancement(4);
}
