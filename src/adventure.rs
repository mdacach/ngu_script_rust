use std::thread;
use std::time::Duration;

use rdev::Key;

use crate::input::send_key;
use crate::pixel;

/// Kills `quantity` monsters in Adventure Mode, by using Regular Attacks. Will disable Idle Mode if needed.
pub fn kill_monsters(quantity: u16) {
    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(Duration::from_millis(50));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        thread::sleep(Duration::from_millis(1050));
        attack(); // So we attack an extra time
        println!("[LOG] Kill Counter: {kills}");
    }
}

/// Adventure mode skills.
///
/// Each skill has an associated letter, its trigger.
#[non_exhaustive]
struct Skills;

impl Skills {
    pub const REGULAR_ATTACK: Key = Key::KeyW;
    pub const STRONG_ATTACK: Key = Key::KeyE;
    pub const PARRY: Key = Key::KeyR;
    pub const PIERCING_ATTACK: Key = Key::KeyT;
    pub const ULTIMATE_ATTACK: Key = Key::KeyY;

    pub const BLOCK: Key = Key::KeyA;
    pub const DEFENSIVE_BUFF: Key = Key::KeyS;
    pub const HEAL: Key = Key::KeyD;
    pub const OFFENSIVE_BUFF: Key = Key::KeyF;
    pub const CHARGE: Key = Key::KeyG;
    pub const ULTIMATE_BUFF: Key = Key::KeyH;
}

fn attack_highest_available() {
    // Attempt to cast all skill, stronger first
    // This (generally) amounts to using the strongest skill available
    // TODO: refactor this to
    //       1 - Know which ones are in cooldown (get button pixel color)
    //       2 - Use skills in order for maximum effectiveness

    send_key(Skills::ULTIMATE_BUFF);
    send_key(Skills::OFFENSIVE_BUFF);
    send_key(Skills::CHARGE);
    send_key(Skills::ULTIMATE_ATTACK);
    send_key(Skills::PIERCING_ATTACK);
    send_key(Skills::PARRY);
    send_key(Skills::STRONG_ATTACK);
    send_key(Skills::REGULAR_ATTACK);

    // Defensive skills are not needed right now
    // send_key(Skills::BLOCK);
    // send_key(Skills::DEFENSIVE_BUFF);
    // send_key(Skills::HEAL);
}

fn attack() {
    send_key(Key::KeyW); // Regular attack
}

fn is_enemy_alive() -> bool {
    let color = pixel::get_pixel_rgb(pixel::ENEMY_BAR_LEFT.into());
    color == pixel::ENEMY_ALIVE_RGB
}

fn is_idle_mode() -> bool {
    let color = pixel::get_pixel_rgb(pixel::IDLE_MODE_PIXEL.into());
    color == pixel::IDLE_MODE_ON_RGB
}
