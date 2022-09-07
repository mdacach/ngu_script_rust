use std::thread;
use std::time::Duration;

use crate::constants::time_machine;
use crate::constants::user::LONG_SLEEP;
use crate::input::input_number;
use crate::menu;
use crate::menu::Menu;
use crate::*;

pub fn add_energy() {
    input::click_at(*time_machine::coords::ADD_ENERGY);
}

pub fn add_magic() {
    input::click_at(*time_machine::coords::ADD_MAGIC);
}

pub fn remove_energy() {
    input::click_at(*time_machine::coords::REMOVE_ENERGY);
}

pub fn remove_magic() {
    input::click_at(*time_machine::coords::REMOVE_MAGIC);
}

#[test]
fn test_menu() {
    menu::navigate(Menu::TimeMachine);
    input_number(10_000_000);
    std::thread::sleep(std::time::Duration::from_secs(2));
    input::click_at(*time_machine::coords::ADD_ENERGY);
}
