use rdev::Key;

use crate::constants::inventory::*;
use crate::coords::InGamePosition;
use crate::input;
use crate::input::{click_at, right_click_at};

pub fn move_to_slot(id: u16) {
    let mut pos = *coords::SLOT_FIRST;
    // Rows wrap around after some slots
    let move_right = id % SLOTS_PER_ROW;
    let move_down = id / SLOTS_PER_ROW;
    pos.x += move_right * SLOT_SIZE.width;
    pos.y += move_down * SLOT_SIZE.height;
    input::mouse_move(pos);
}

pub fn click_slot(id: u16) {
    move_to_slot(id);
    input::click();
}

pub fn merge_slot(id: u16) {
    // Clicking is better than just moving because it puts the game in focus
    click_slot(id);
    merge();
}

pub fn boost_slot(id: u16) {
    // Clicking is better than just moving because it puts the game in focus
    click_slot(id);
    boost();
}

pub fn merge_equips() {
    merge_at(*coords::WEAPON);
    merge_at(*coords::HELMET);
    merge_at(*coords::CHEST);
    merge_at(*coords::LEGS);
    merge_at(*coords::BOOTS);
    merge_at(*coords::ACC1);
    merge_at(*coords::ACC2);
}

pub fn boost_equips() {
    // Order here will change depending on game's progression.
    // Put the most important items first, so that boost is used
    // more efficiently.
    boost_at(*coords::WEAPON);
    // Accessories start to be more important now.
    boost_at(*coords::ACC1);
    boost_at(*coords::ACC2);
    boost_at(*coords::HELMET);
    boost_at(*coords::CHEST);
    boost_at(*coords::LEGS);
    boost_at(*coords::BOOTS);
}

fn merge_at(pos: InGamePosition) {
    click_at(pos);
    merge();
}

fn boost_at(pos: InGamePosition) {
    click_at(pos);
    boost();
}

fn merge() {
    input::send_key(Key::KeyD);
}

fn boost() {
    input::send_key(Key::KeyA);
}

/// Infinity Cube is a special accessory meant to consume boosts.
/// Instead of boosting as usually, it uses a right click instead.
pub fn boost_cube() {
    right_click_at(*coords::CUBE);
}
