use std::thread;

use enigo::Key;

use crate::adventure;
use crate::constants::itopod::coords;
use crate::constants::user::*;
use crate::{input, ocr};

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

    adventure::disable_idle_mode_if_needed();

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
        input::send_key(Key::Layout('q')); // Disable Idle Mode
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
    input::input_number(999);

    // Confirm Enter
    input::click_at(*coords::ENTER_CONFIRMATION_PIXEL);
    thread::sleep(LONG_SLEEP);

    adventure::disable_idle_mode_if_needed();

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

pub fn enter_itopod_at_floor(floor: u16) {
    // Enter itopod
    input::click_at(*coords::ENTER_PIXEL);
    thread::sleep(LONG_SLEEP);

    // Set starting floor to desired
    input::click_at(*coords::START_FLOOR_INPUT_PIXEL);
    thread::sleep(LONG_SLEEP);
    input::input_number(floor as u64);

    // Confirm Enter
    input::click_at(*coords::ENTER_CONFIRMATION_PIXEL);
    thread::sleep(LONG_SLEEP);
}

pub fn get_current_tier_next_rewards() -> u8 {
    input::click_at(*crate::constants::itopod::coords::GET_TOOLTIP_PIXEL);
    thread::sleep(std::time::Duration::from_secs(2));
    let text =
        ocr::get_ocr_text_from_area(*crate::constants::itopod::areas::ITOPOD_TOOLTIP_OCR_AREA)
            .expect("Could not OCR ITOPOD tier rewards");

    let text: Vec<String> = text.split(' ').map(str::to_string).collect();
    let after_count = text
        .iter()
        .position(|x| x.contains("kills."))
        .expect("Something wrong with ITOPOD OCR");
    let count = text
        .get(after_count - 1)
        .expect("Something wrong with ITOPOD OCR");
    count
        .parse::<u8>()
        .expect("Could not parse ITOPOD kills as integer")
}

pub fn get_tier_next_rewards_of_floor(floor: u16) -> u8 {
    enter_itopod_at_floor(floor);
    get_current_tier_next_rewards()
}

pub fn itopod_exploit() {
    let floor_max_counter: Vec<_> = (20..40).rev().collect();
    let mut floor_to_count = Vec::new();
    (0..=200).step_by(50).for_each(|floor| {
        let count = get_tier_next_rewards_of_floor(floor);
        floor_to_count.push(count);
    });

    let process_kill = |counters: &mut Vec<_>| {
        for (index, count) in counters.iter_mut().enumerate() {
            if *count == 1 {
                *count = floor_max_counter[index];
            } else {
                *count -= 1;
            }
        }
    };

    println!("Initiating the itopod routine");
    thread::sleep(LONG_SLEEP * 3);
    println!("Disable idle");
    adventure::disable_idle_mode_if_needed();
    let mut current_floor = 9999;
    loop {
        let min_count = floor_to_count.iter().min().unwrap();
        let next_floor = floor_to_count.iter().position(|x| x == min_count).unwrap() * 50;
        println!("Min count {} at floor {}", min_count, next_floor);
        println!("Killing one enemy at floor {}", next_floor);
        if next_floor != current_floor {
            enter_itopod_at_floor(next_floor as _);
            current_floor = next_floor;
        }
        while !adventure::is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }
        adventure::fast_kill_enemy();
        process_kill(&mut floor_to_count);
        println!("Killed one enemy. Updated counters {:?}", floor_to_count);
    }
}

#[cfg(test)]
mod tests {
    use crate::menu;

    use super::*;

    #[test]
    fn test_ocr_itopod_tiers() {
        menu::navigate(menu::Menu::Adventure);

        println!("{}", get_tier_next_rewards_of_floor(0));
        println!("{}", get_tier_next_rewards_of_floor(50));
        println!("{}", get_tier_next_rewards_of_floor(100));
        println!("{}", get_tier_next_rewards_of_floor(150));
        println!("{}", get_tier_next_rewards_of_floor(200));
    }
}
