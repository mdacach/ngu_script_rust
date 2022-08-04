use std::thread;

use crate::adventure;
use crate::constants::itopod::coords;
use crate::constants::user::*;
use crate::input;

/// Kills `quantity` enemies on ITOPOD's optimal floor.
/// Requires the game to be in "Adventure" menu.
pub fn fast_itopod(quantity: u16) {
    // Enter itopod and choose optimal floor
    input::click_at(*coords::ENTER_PIXEL);

    thread::sleep(LONG_SLEEP);
    input::click_at(*coords::OPTIMAL_FLOOR_PIXEL);
    thread::sleep(LONG_SLEEP);
    input::click_at(*coords::ENTER_CONFIRMATION_PIXEL);
    thread::sleep(LONG_SLEEP);

    if adventure::is_idle_mode() {
        input::send_key(rdev::Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !adventure::is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }

        while adventure::is_enemy_alive() {
            adventure::attack();
            thread::sleep(FAST_SLEEP);
        }

        println!("[LOG] Kill Counter: {}", kills);
    }

    if !adventure::is_idle_mode() {
        input::send_key(rdev::Key::KeyQ); // Disable Idle Mode
    }
}

/// Pushes the max floor of ITOPOD. Repeats if player dies.
/// Requires the game to be in "Adventure" menu.
pub fn push_itopod() {
    // Enter itopod
    input::click_at(*coords::ENTER_PIXEL);
    thread::sleep(LONG_SLEEP);

    // Set initial floor to MAX
    input::click_at(*coords::MAX_FLOOR_PIXEL);
    thread::sleep(LONG_SLEEP);

    // Set end floor to some big enough number
    input::click_at(*coords::END_FLOOR_INPUT_PIXEL);
    thread::sleep(LONG_SLEEP);
    input::send_key(rdev::Key::Num9);
    input::send_key(rdev::Key::Num9);
    input::send_key(rdev::Key::Num9);

    // Confirm Enter
    input::click_at(*coords::ENTER_CONFIRMATION_PIXEL);
    thread::sleep(LONG_SLEEP);

    if adventure::is_idle_mode() {
        input::send_key(rdev::Key::KeyQ); // Disable Idle Mode
    }

    let mut kill_counter = 0;
    loop {
        while !adventure::is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }

        while adventure::is_enemy_alive() {
            adventure::attack_highest_available();
            thread::sleep(FAST_SLEEP);
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = std::time::Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        adventure::attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill counter: {}", kill_counter);
    }
}
