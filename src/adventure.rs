use std::thread;
use std::time::Duration;

use imageproc::math::cast;
use lazy_static::lazy_static;
use rdev::Key;

use crate::constants::adventure::*;
use crate::constants::user::{FAST_SLEEP, LONG_SLEEP};
use crate::coords::GameAwarePosition;
use crate::input::{click_at, right_click_at, send_key};
use crate::pixel;
use crate::pixel::get_pixel_rgb;
use crate::{constants, menu};

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
    GCT,
    TwoD,
    AB,
    Jake,
}

fn disable_idle_mode_if_needed() {
    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode key.
    }
}

/// Kills `quantity` enemies on ITOPOD's optimal floor.
/// Requires the game to be in "Adventure" menu.
pub fn fast_itopod(quantity: u16) {
    // Enter itopod and choose optimal floor
    click_at(*coords::ITOPOD_ENTER_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);
    click_at(*coords::ITOPOD_OPTIMAL_FLOOR_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);
    click_at(*coords::ITOPOD_ENTER_CONFIRMATION_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(constants::user::FAST_SLEEP);
        }

        while is_enemy_alive() {
            attack();
            thread::sleep(constants::user::FAST_SLEEP);
        }

        println!("[LOG] Kill Counter: {}", kills);
    }

    if !is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }
}

/// Performs regular attacks until enemy is dead.
/// Enemy must be already in screen when function is called. (Wait for it to spawn *before* calling it).
/// Best used if you are considerably stronger than the enemy, and can kill it
/// in few hits.
fn fast_kill_enemy() {
    while is_enemy_alive() {
        attack();
        thread::sleep(constants::user::FAST_SLEEP);
    }
}

/// Performs the strongs available skills until enemy is dead.
/// Enemy must be already in screen when function is called. (Wait for it to spawn *before* calling it).
/// TODO: Change this to "cache" available skills (remove the need of regular screenshots).
fn kill_enemy() {
    while is_enemy_alive() {
        attack_highest_available();
        thread::sleep(constants::user::FAST_SLEEP);
    }
    // It's possible that the monster is still alive, but we can not see it
    // because the bar is almost completely white
    let a_bit_more_than_a_sec = Duration::from_millis(1050);
    thread::sleep(a_bit_more_than_a_sec);
    attack(); // So we attack an extra time
}

/// Pushes the max floor of ITOPOD. Repeats if player dies.
/// Requires the game to be in "Adventure" menu.
pub fn push_itopod() {
    // Enter itopod
    click_at(*coords::ITOPOD_ENTER_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);

    // Set initial floor to MAX
    click_at(*coords::ITOPOD_MAX_FLOOR_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);

    // Set end floor to some big enough number
    click_at(*coords::ITOPOD_END_FLOOR_INPUT_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);
    send_key(Key::Num9);
    send_key(Key::Num9);
    send_key(Key::Num9);

    // Confirm Enter
    click_at(*coords::ITOPOD_ENTER_CONFIRMATION_PIXEL);
    thread::sleep(constants::user::LONG_SLEEP);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    let mut kill_counter = 0;
    loop {
        while !is_enemy_alive() {
            thread::sleep(constants::user::FAST_SLEEP);
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(constants::user::FAST_SLEEP);
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill counter: {}", kill_counter);
    }
}

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
            thread::sleep(constants::user::FAST_SLEEP);
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(constants::user::FAST_SLEEP);
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        attack(); // So we attack an extra time
        println!("[LOG] Kill Counter: {}", kills);
    }
}

pub fn fast_kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(constants::user::FAST_SLEEP);
        }

        while is_enemy_alive() {
            attack();
            thread::sleep(constants::user::FAST_SLEEP);
        }
        println!("[LOG] Kill Counter: {}", kills);
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
            thread::sleep(constants::user::FAST_SLEEP);
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
            thread::sleep(constants::user::FAST_SLEEP);
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        let a_bit_more_than_a_sec = Duration::from_millis(1050);
        thread::sleep(a_bit_more_than_a_sec);
        attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill Counter: {}", kill_counter);
    }
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

pub fn go_to_latest() {
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

/// Performs a specific routine to kill one boss at desired zone.
/// 1. Wait for 100% HP in Safe Zone
/// 2. Use Charge and Parry
/// 3. Recharge Charge and Parry cooldowns
/// 4. Wait for Offensive and Ultimate Buffs to be available
/// 5. Go to zone and wait for boss
pub fn snipe_boss_at_zone(zone: AdventureZone) {
    let wait_for = |skill: &AdventureSkill| {
        thread::sleep(5 * LONG_SLEEP); // Let the game catch-up.
        while !skill.is_available() {
            thread::sleep(FAST_SLEEP);
        }
    };

    let cast_when_available = |skill: &AdventureSkill| {
        // This is needed because of between-attacks cooldown.
        wait_for(skill);
        skill.cast();
        thread::sleep(LONG_SLEEP); // Let the game catch-up.
    };

    let initial_attack_routine = || {
        cast_when_available(&OFFENSIVE_BUFF);
        cast_when_available(&ULTIMATE_BUFF);
        cast_when_available(&ULTIMATE_ATTACK);
        cast_when_available(&PIERCING_ATTACK);
    };

    menu::navigate(menu::Menu::Adventure);
    println!("Navigated");

    // We need to pre-charge our skills in SafeZone.
    go_to_zone(AdventureZone::Safe);
    disable_idle_mode_if_needed();
    println!("Disabled IDLE");
    println!("Waiting for CHARGE");
    cast_when_available(&CHARGE);
    println!("Waiting for PARRY");
    cast_when_available(&PARRY);

    // Now we need to wait for the skills to recharge.
    println!("Waiting for CHARGE to end cooldown");
    wait_for(&CHARGE);
    println!("Waiting for PARRY ot end cooldown");
    wait_for(&PARRY);

    // We must have our offensive buffs prepared too.
    println!("Waiting for OFFENSIVE_BUFF cooldown");
    wait_for(&OFFENSIVE_BUFF);
    println!("Waiting for ULTIMATE_BUFF cooldown");
    wait_for(&ULTIMATE_BUFF);
    // TODO: Wait for 100% HP too.
    // TODO: Remove comments.

    go_to_zone(zone);
    loop {
        while !is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }
        // TODO: this DOES NOT work for Titans! (Infinite loop)
        //       Create specific function for killing Titans
        if is_enemy_boss() {
            initial_attack_routine();
            kill_enemy();
            break; // Eventually we will encounter some boss.
        } else {
            refresh_zone();
        }
    }
    // We are done with one sniping.
}

#[test]
fn snipe_boss() {
    menu::navigate(menu::Menu::Adventure);
    loop {
        snipe_boss_at_zone(AdventureZone::AB);
    }
}
