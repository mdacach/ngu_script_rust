use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use rdev::Key;

use crate::constants;
use crate::constants::adventure::*;
use crate::coords::GameAwarePosition;
use crate::input::{right_click_at, send_key};
use crate::pixel;
use crate::pixel::get_pixel_rgb;

/// Kills `quantity` monsters in the Adventure Zone chosen.
/// Will disable Idle Mode if needed.
/// Requires the game to be in "Adventure" menu.
pub fn kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        attack(); // So we attack an extra time
        println!("[LOG] Kill Counter: {kills}");
    }
}

pub fn fast_kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }

        while is_enemy_alive() {
            attack();
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }
        println!("[LOG] Kill Counter: {kills}");
    }
}

/// Kills `quantity` bosses in the Adventure Zone chosen.
/// Will disable Idle Mode if needed.
/// Requires the game to be in "Adventure" menu.
pub fn kill_bosses_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    let mut kill_counter = 0;
    while kill_counter < quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }

        if is_enemy_boss() {
            println!("[LOG] Found a boss, fighting.");
        } else {
            println!("[LOG] Not a boss, skipping.");
            refresh_zone();
            continue;
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(Duration::from_millis(constants::FAST_SLEEP));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill Counter: {kill_counter}");
    }
}

/// Zones in Adventure Menu, ordered by appearance.
pub enum AdventureZone {
    // This is meant to be updated as you progress through the game.
    /// Note that order here is important, as it's used for navigating between zones.
    Safe,
    Tutorial,
    Sewers,
    Forest,
    Cave,
    Sky,
    HSB,
    GRB,
    Clock,
}

/// Navigates to corresponding zone.
/// Requires the game to be in "Adventure" menu.
pub fn go_to_zone(zone: AdventureZone) {
    // First we must start from 0: The Safe Zone
    // Right clicking adventure's left arrow makes us go to Safe
    right_click_at(*coords::RETREAT_ZONE_PIXEL);

    // Then, we get how much to go forward by Zone Numbering
    let forward_steps = zone as u8;
    for _ in 0..forward_steps {
        advance_zone();
    }
}

fn go_to_latest() {
    // Right clicking adventure's right arrow makes us to go to latest available zone
    right_click_at(*coords::ADVANCE_ZONE_PIXEL);
}

fn advance_zone() {
    send_key(Key::RightArrow); // We could also use the mouse here
}

fn retreat_zone() {
    send_key(Key::LeftArrow); // We could also use the mouse here
}

fn refresh_zone() {
    retreat_zone();
    advance_zone();
}

fn attack_highest_available() {
    // Attempt to cast all skills, stronger first
    // This (generally) amounts to using the strongest skill available
    // TODO: refactor this to
    //       1 - Know which ones are in cooldown (get button pixel color)
    //       2 - Use skills in order for maximum effectiveness

    ULTIMATE_BUFF.cast();
    OFFENSIVE_BUFF.cast();
    CHARGE.cast();
    ULTIMATE_ATTACK.cast();
    PIERCING_ATTACK.cast();
    PARRY.cast();
    STRONG_ATTACK.cast();
    REGULAR_ATTACK.cast();

    // Defensive skills are not needed right now
    // BLOCK.cast();
    // DEFENSIVE_BUFF.cast();
    // HEAL.cast();
}

fn attack() {
    send_key(Key::KeyW); // Regular attack
}

fn is_enemy_alive() -> bool {
    let color = get_pixel_rgb(*pixel::ENEMY_BAR_LEFT_PIXEL);
    color == pixel::ENEMY_ALIVE_RGB
}

fn is_enemy_boss() -> bool {
    let color = get_pixel_rgb(*coords::BOSS_CROWN_PIXEL);
    color == colors::BOSS_CROWN_RGB
}

fn is_idle_mode() -> bool {
    let color = get_pixel_rgb(*pixel::IDLE_MODE_PIXEL);
    color == pixel::IDLE_MODE_ON_RGB
}

/// Represents a castable adventure skill.
struct AdventureSkill {
    key: Key,
    // Key used to cast the skill.
    pixel_coords: GameAwarePosition,
    // Pixel to check if the skill is available.
    row_number: u8, // Row the skill is at, from 1 to 3.
}

impl AdventureSkill {
    const fn new(key: Key, pixel_coords: GameAwarePosition, row_number: u8) -> Self {
        Self {
            key,
            pixel_coords,
            row_number,
        }
    }
}

trait Skill {
    /// Returns true if skill is currently available to be used, false otherwise.
    fn is_available(&self) -> bool;

    /// Attempts to cast the skill. Returns true if cast was successful.
    ///
    /// A cast is successful if the skill was ready (i.e is_available() is true).
    /// Otherwise, the cast fails and nothing happens.
    fn cast(&self) -> bool;
}

impl Skill for AdventureSkill {
    fn is_available(&self) -> bool {
        let current_color = get_pixel_rgb(self.pixel_coords);
        match self.row_number {
            1 => current_color != colors::FIRST_ROW_COOLDOWN_RGB,
            2 => current_color != colors::SECOND_ROW_COOLDOWN_RGB,
            _ => panic!("Unexpected row number"),
        }
    }

    fn cast(&self) -> bool {
        if !self.is_available() {
            return false;
        }

        send_key(self.key);
        true
    }
}

// Create the skills with constants from `constants` module.
lazy_static! {
    static ref REGULAR_ATTACK: AdventureSkill =
        AdventureSkill::new(keys::REGULAR_ATTACK, *coords::REGULAR_ATTACK_PIXEL, 1);
    static ref STRONG_ATTACK: AdventureSkill =
        AdventureSkill::new(keys::STRONG_ATTACK, *coords::STRONG_ATTACK_PIXEL, 1);
    static ref PARRY: AdventureSkill = AdventureSkill::new(keys::PARRY, *coords::PARRY_PIXEL, 1);
    static ref PIERCING_ATTACK: AdventureSkill =
        AdventureSkill::new(keys::PIERCING_ATTACK, *coords::PIERCING_ATTACK_PIXEL, 1);
    static ref ULTIMATE_ATTACK: AdventureSkill =
        AdventureSkill::new(keys::ULTIMATE_ATTACK, *coords::ULTIMATE_ATTACK_PIXEL, 1);
    static ref BLOCK: AdventureSkill = AdventureSkill::new(keys::BLOCK, *coords::BLOCK_PIXEL, 2);
    static ref DEFENSIVE_BUFF: AdventureSkill =
        AdventureSkill::new(keys::DEFENSIVE_BUFF, *coords::DEFENSIVE_BUFF_PIXEL, 2);
    static ref HEAL: AdventureSkill = AdventureSkill::new(keys::HEAL, *coords::HEAL_PIXEL, 2);
    static ref OFFENSIVE_BUFF: AdventureSkill =
        AdventureSkill::new(keys::OFFENSIVE_BUFF, *coords::OFFENSIVE_BUFF_PIXEL, 2);
    static ref CHARGE: AdventureSkill = AdventureSkill::new(keys::CHARGE, *coords::CHARGE_PIXEL, 2);
    static ref ULTIMATE_BUFF: AdventureSkill =
        AdventureSkill::new(keys::ULTIMATE_BUFF, *coords::ULTIMATE_BUFF_PIXEL, 2);
}
