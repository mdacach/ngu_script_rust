use crate::input::click_at;

pub fn nuke() {
    click_at(*crate::constants::fight_boss::coords::NUKE)
}

pub fn fight() {
    click_at(*crate::constants::fight_boss::coords::FIGHT)
}
