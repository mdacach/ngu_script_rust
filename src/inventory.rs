use lazy_static::lazy_static;
use rdev::Key;

use crate::coords::{InGamePosition, Size};
use crate::input;
use crate::input::{click_at, right_click_at};

const SLOT_SIZE: Size = Size {
    width: 66,
    height: 67,
};

const SLOTS_PER_ROW: u16 = 12;

lazy_static! {
    static ref SLOT_FIRST: InGamePosition = InGamePosition::from_coords(470, 440);
    static ref HELMET: InGamePosition = InGamePosition::from_coords(705, 88);
    static ref CHEST: InGamePosition = InGamePosition::from_coords(705, 157);
    static ref LEGS: InGamePosition = InGamePosition::from_coords(705, 224);
    static ref BOOTS: InGamePosition = InGamePosition::from_coords(705, 289);
    static ref WEAPON: InGamePosition = InGamePosition::from_coords(775, 157);
    static ref ACC1: InGamePosition = InGamePosition::from_coords(639, 88);
    static ref ACC2: InGamePosition = InGamePosition::from_coords(639, 157);
    static ref CUBE: InGamePosition = InGamePosition::from_coords(840, 157);
}

pub fn move_to_slot(id: u16) {
    let mut pos = *SLOT_FIRST;
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
    merge_at(*WEAPON);
    merge_at(*HELMET);
    merge_at(*CHEST);
    merge_at(*LEGS);
    merge_at(*BOOTS);
    merge_at(*ACC1);
    merge_at(*ACC2);
}

pub fn boost_equips() {
    // Order here will change depending on game's progression.
    // Put the most important items first, so that boost is used
    // more efficiently.
    boost_at(*WEAPON);
    boost_at(*HELMET);
    boost_at(*CHEST);
    boost_at(*LEGS);
    boost_at(*BOOTS);
    boost_at(*ACC1);
    boost_at(*ACC2);
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
    right_click_at(*CUBE);
}
