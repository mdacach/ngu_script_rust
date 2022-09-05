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
