use rdev::Key;

use crate::input;

const SLOT_FIRST: (u32, u32) = (470, 440);
const SLOT_SIZE: (u32, u32) = (66, 67);
const SLOTS_PER_ROW: u32 = 12;

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
    input::send_key(Key::KeyD);
}
