use crate::*;

pub fn add_ritual(number: u8) {
    match number {
        0 => input::click_at(*constants::blood_magic::coords::RIT_0_ADD),
        1 => input::click_at(*constants::blood_magic::coords::RIT_1_ADD),
        2 => input::click_at(*constants::blood_magic::coords::RIT_2_ADD),
        3 => input::click_at(*constants::blood_magic::coords::RIT_3_ADD),
        4 => input::click_at(*constants::blood_magic::coords::RIT_4_ADD),
        5 => input::click_at(*constants::blood_magic::coords::RIT_5_ADD),
        6 => input::click_at(*constants::blood_magic::coords::RIT_6_ADD),
        _ => {}
    }
}

#[test]
fn test_menu() {
    menu::navigate(menu::Menu::BloodMagic);
    add_ritual(4);
}
