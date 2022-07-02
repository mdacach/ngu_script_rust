use rdev::Key;

use crate::input;
use crate::input::{click_at, mouse_move};

const SLOT_FIRST: (u32, u32) = (470, 440);
const SLOT_SIZE: (u32, u32) = (66, 67);
const SLOTS_PER_ROW: u32 = 12;

const HELMET: (u32, u32) = (705, 88);
const CHEST: (u32, u32) = (705, 157);
const LEGS: (u32, u32) = (705, 224);
const BOOTS: (u32, u32) = (705, 289);
const WEAPON: (u32, u32) = (775, 157);
const ACC1: (u32, u32) = (639, 88);
const ACC2: (u32, u32) = (639, 157);
const CUBE: (u32, u32) = (840, 157);

pub fn move_to_slot(id: u32) {
    let (mut x, mut y) = SLOT_FIRST;
    // Rows wrap around after some slots
    let move_right = id % SLOTS_PER_ROW;
    let move_down = id / SLOTS_PER_ROW;
    x += move_right * SLOT_SIZE.0;
    y += move_down * SLOT_SIZE.1;
    input::mouse_move((x, y));
}

pub fn click_slot(id: u32) {
    move_to_slot(id);
    input::click();
}

pub fn merge_slot(id: u32) {
    // Clicking is better than just moving because it puts the game in focus
    click_slot(id);
    merge();
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
}

fn merge_at(coords: (u32, u32)) {
    click_at(coords);
    merge();
}

fn boost_at(coords: (u32, u32)) {
    click_at(coords);
    boost();
}

fn merge() {
    input::send_key(Key::KeyD);
}

fn boost() {
    input::send_key(Key::KeyA);
}
