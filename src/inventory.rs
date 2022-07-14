use std::iter::from_fn;

use rdev::Key;

use crate::constants::inventory::*;
use crate::coords::GameAwarePosition;
use crate::input::{click_at, right_click_at};
use crate::{input, pixel};

pub fn get_coords_of_slot(id: u16) -> GameAwarePosition {
    let mut pos = *coords::SLOT_FIRST;
    // Rows wrap around after some slots
    let move_right = id % SLOTS_PER_ROW;
    let move_down = id / SLOTS_PER_ROW;
    pos.x += move_right * SLOT_SIZE.width;
    pos.y += move_down * SLOT_SIZE.height;

    pos
}

/// Merges all equipments slots.
/// Requires the game to be in "Inventory" menu.
pub fn merge_equips() {
    use EquipmentSlot::*;
    // Order here is not as relevant as in boosting
    merge_equip(Weapon);
    merge_equip(Helmet);
    merge_equip(Chest);
    merge_equip(Legs);
    merge_equip(Boots);
    merge_equip(Acc1);
    merge_equip(Acc2);
}

/// Boosts all equipments slots.
/// Order of boosting must be hard-coded.
/// Requires the game to be in "Inventory" menu.
pub fn boost_equips() {
    // Order here will change depending on game's progression.
    // Put the most important items first, so that boost is used
    // more efficiently.
    use EquipmentSlot::*;
    boost_equip(Weapon);
    // Accessories start to be more important now.
    boost_equip(Acc1);
    boost_equip(Acc2);
    boost_equip(Helmet);
    boost_equip(Chest);
    boost_equip(Legs);
    boost_equip(Boots);
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
fn boost_cube() {
    // Instead of boosting as usually (pressing "a"), cube uses a right click instead.
    right_click_at(*coords::CUBE);
}

/// Returns true if inventory slot is empty.
pub fn is_slot_empty(slot: InventorySlot) -> bool {
    let color = pixel::get_pixel_rgb(slot.coords);
    color == colors::EMPTY_SLOT_RGB
    // This checks a specific pixel in the inventory slot.
    // If the pixel is gray (as empty slots are), the slot is considered empty.
    // This can mistakenly identify a slot as empty if the item in there happens to be
    // of the same color. TODO: Add redundancy here (e.g., check for a couple pixels more)
}

/// Returns the number of empty slots in inventory.
pub fn count_empty_slots() -> u16 {
    inventory_slots()
        .filter(|&slot| is_slot_empty(slot))
        .count() as u16
}

#[derive(Copy, Clone)]
pub struct InventorySlot {
    coords: GameAwarePosition,
}

impl InventorySlot {
    pub fn from_id(id: u16) -> Self {
        Self {
            coords: get_coords_of_slot(id),
        }
    }
}

impl Boost for InventorySlot {
    fn boost(&self) {
        boost_at(self.coords);
    }
}

impl Merge for InventorySlot {
    fn merge(&self) {
        merge_at(self.coords);
    }
}

pub trait Boost {
    fn boost(&self);
}

pub trait Merge {
    fn merge(&self);
}

/// Iterates over all available inventory slots positions.
pub fn inventory_slots() -> impl Iterator<Item = InventorySlot> {
    let mut current_id = 0;
    let get_slot = move || {
        let slot = if current_id < SLOTS_AVAILABLE {
            Some(InventorySlot::from_id(current_id))
        } else {
            None
        };
        current_id += 1;
        slot
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
        s.merge();
        s.boost();
    });
}

pub enum EquipmentSlot {
    Weapon,
    Acc1,
    Acc2,
    Helmet,
    Chest,
    Legs,
    Boots,
    Cube,
}

pub fn merge_equip(slot: EquipmentSlot) {
    use EquipmentSlot::*;
    match slot {
        Weapon => merge_at(*coords::WEAPON),
        Acc1 => merge_at(*coords::ACC1),
        Acc2 => merge_at(*coords::ACC2),
        Helmet => merge_at(*coords::HELMET),
        Chest => merge_at(*coords::CHEST),
        Legs => merge_at(*coords::LEGS),
        Boots => merge_at(*coords::BOOTS),
        Cube => (), // Cube does not merge
    }
}

pub fn boost_equip(slot: EquipmentSlot) {
    use EquipmentSlot::*;
    match slot {
        Weapon => boost_at(*coords::WEAPON),
        Acc1 => boost_at(*coords::ACC1),
        Acc2 => boost_at(*coords::ACC2),
        Helmet => boost_at(*coords::HELMET),
        Chest => boost_at(*coords::CHEST),
        Legs => boost_at(*coords::LEGS),
        Boots => boost_at(*coords::BOOTS),
        Cube => boost_cube(), // Cube does not boost
    }
}

#[test]
fn test_equip_slots() {
    merge_equips();
    boost_equips();
    boost_equip(EquipmentSlot::Cube);
}
