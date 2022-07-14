use std::iter::from_fn;

use rdev::Key;

use crate::constants::inventory::*;
use crate::coords::GameAwarePosition;
use crate::input::{click_at, right_click_at};
use crate::{input, pixel};

/// Moves the mouse to corresponding slot in inventory tab.
/// Slots are numbered from left to right and top to down, starting at 0.
/// Requires the game to be in "Inventory" menu.
pub fn move_to_slot(id: u16) {
    let pos = get_coords_of_slot(id);
    input::mouse_move(pos);
}

pub fn get_coords_of_slot(id: u16) -> GameAwarePosition {
    let mut pos = *coords::SLOT_FIRST;
    // Rows wrap around after some slots
    let move_right = id % SLOTS_PER_ROW;
    let move_down = id / SLOTS_PER_ROW;
    pos.x += move_right * SLOT_SIZE.width;
    pos.y += move_down * SLOT_SIZE.height;

    pos
}

/// Moves the mouse to corresponding slot in inventory tab and left-clicks it.
/// Slots are numbered from left to right and top to down, starting at 0.
/// Requires the game to be in "Inventory" menu.
pub fn click_slot(id: u16) {
    move_to_slot(id);
    input::click();
}

/// Merges the item in corresponding slot.
/// If the slot is empty, nothing happens.
/// Slots are numbered from left to right and top to down, starting at 0.
/// Requires the game to be in "Inventory" menu.
pub fn merge_slot(id: u16) {
    // Clicking is better than just moving because it puts the game in focus
    click_slot(id);
    merge();
}

/// Boosts the item in corresponding slot.
/// If the slot is empty, nothing happens.
/// Slots are numbered from left to right and top to down, starting at 0.
/// Requires the game to be in "Inventory" menu.
pub fn boost_slot(id: u16) {
    // Clicking is better than just moving because it puts the game in focus
    click_slot(id);
    boost();
}

/// Merges all equipments slots.
/// Requires the game to be in "Inventory" menu.
pub fn merge_equips() {
    merge_at(*coords::WEAPON);
    merge_at(*coords::HELMET);
    merge_at(*coords::CHEST);
    merge_at(*coords::LEGS);
    merge_at(*coords::BOOTS);
    merge_at(*coords::ACC1);
    merge_at(*coords::ACC2);
}

/// Boosts all equipments slots.
/// Order of boosting must be hard-coded.
/// Requires the game to be in "Inventory" menu.
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

fn merge_at(pos: GameAwarePosition) {
    click_at(pos);
    merge();
}

fn boost_at(pos: GameAwarePosition) {
    click_at(pos);
    boost();
}

/// Inventory shortcut to merge is "d".
/// Note that you must have enabled "Simple Inventory Shortcut" in Settings.
fn merge() {
    input::send_key(Key::KeyD);
}

/// Inventory shortcut to boost is "a".
/// Note that you must have enabled "Simple Inventory Shortcut" in Settings.
fn boost() {
    input::send_key(Key::KeyA);
}

/// Boosts infinity cube.
/// Infinity Cube is a special accessory meant to consume boosts.
/// Requires the game to be in "Inventory" menu.
pub fn boost_cube() {
    // Instead of boosting as usually (pressing "a"), cube uses a right click instead.
    right_click_at(*coords::CUBE);
}

/// Returns true if inventory slot is empty.
pub fn is_slot_empty(id: u16) -> bool {
    let coords = get_coords_of_slot(id);
    let color = pixel::get_pixel_rgb(coords);
    color == colors::EMPTY_SLOT_RGB
    // This checks a specific pixel in the inventory slot.
    // If the pixel is gray (as empty slots are), the slot is considered empty.
    // This can mistakenly identify a slot as empty if the item in there happens to be
    // of the same color. TODO: Add redundancy here (e.g., check for a couple pixels more)
}

/// Returns the number of empty slots in inventory.
pub fn count_empty_slots() -> u16 {
    let empty_count = (0..SLOTS_AVAILABLE).filter(|&id| is_slot_empty(id)).count();
    empty_count as u16
}

/// Iterates over all available inventory slots positions.
// TODO: Make a struct to represent a (an?) inventory slot
pub fn inventory_slots() -> impl Iterator<Item = GameAwarePosition> {
    let mut current_id = 0;
    let get_slot = move || {
        if current_id < SLOTS_AVAILABLE {
            current_id += 1;
            Some(get_coords_of_slot(current_id))
        } else {
            None
        }
    };
    from_fn(get_slot)
}

#[test]
fn test_empty_slots() {
    let empty_count = count_empty_slots();
    println!("Empty slots: {}", empty_count);
}

#[test]
fn test_iterator() {
    inventory_slots().for_each(|s| {
        merge_at(s);
        boost_at(s);
    });
}
