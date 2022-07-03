use rdev::Key;

use crate::coords::{Position, Size};
use crate::input;
use crate::input::{click_at, right_click_at};

const SLOT_SIZE: Size = Size {
    width: 66,
    height: 67,
};
const SLOT_FIRST: Position = Position::from_coords(470, 440);
const SLOTS_PER_ROW: u16 = 12;

const HELMET: Position = Position::from_coords(705, 88);
const CHEST: Position = Position::from_coords(705, 157);
const LEGS: Position = Position::from_coords(705, 224);
const BOOTS: Position = Position::from_coords(705, 289);
const WEAPON: Position = Position::from_coords(775, 157);
const ACC1: Position = Position::from_coords(639, 88);
const ACC2: Position = Position::from_coords(639, 157);
const CUBE: Position = Position::from_coords(840, 157);

pub fn move_to_slot(id: u16) {
    let Position { mut x, mut y } = SLOT_FIRST;
    // Rows wrap around after some slots
    let move_right = id % SLOTS_PER_ROW;
    let move_down = id / SLOTS_PER_ROW;
    x += move_right * SLOT_SIZE.width;
    y += move_down * SLOT_SIZE.height;
    input::mouse_move(Position::from_coords(x, y).into());
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
    merge_at(WEAPON);
    merge_at(HELMET);
    merge_at(CHEST);
    merge_at(LEGS);
    merge_at(BOOTS);
    merge_at(ACC1);
    merge_at(ACC2);
}

pub fn boost_equips() {
    // Order here will change depending on game's progression.
    // Put the most important items first, so that boost is used
    // more efficiently.
    boost_at(WEAPON);
    boost_at(HELMET);
    boost_at(CHEST);
    boost_at(LEGS);
    boost_at(BOOTS);
    boost_at(ACC1);
    boost_at(ACC2);
    boost_cube();
}

fn merge_at(pos: Position) {
    click_at(pos.into());
    merge();
}

fn boost_at(pos: Position) {
    click_at(pos.into());
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
fn boost_cube() {
    right_click_at(CUBE.into());
}
