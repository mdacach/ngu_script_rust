use std::thread;
use std::time::Duration;

use enigo::Key;
use lazy_static::lazy_static;

use crate::constants::adventure::*;
use crate::constants::user;
use crate::constants::user::{FAST_SLEEP, LONG_SLEEP};
use crate::coords::GameAwarePosition;
use crate::input::{right_click_at, send_key};
use crate::menu;
use crate::pixel;
use crate::pixel::get_pixel_rgb;

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
    AVSP,
    Mega,
    UUG,
    Beard,
    Walderp,
    BDW,
}

pub fn disable_idle_mode_if_needed() {
    if is_idle_mode() {
        send_key(Key::Layout('q')); // Disable Idle Mode key.
    }
}

/// Performs regular attacks until enemy is dead.
/// Enemy must be already in screen when function is called. (Wait for it to spawn *before* calling it).
/// Best used if you are considerably stronger than the enemy, and can kill it
/// in few hits.
pub fn fast_kill_enemy() {
    while is_enemy_alive() {
        attack();
        thread::sleep(FAST_SLEEP);
    }
}

/// Kills `quantity` monsters in the Adventure Zone chosen.
/// Will disable Idle Mode if needed.
/// Requires the game to be in "Adventure" menu.
pub fn kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    disable_idle_mode_if_needed();

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }

        kill_enemy();
        println!("[LOG] Kill Counter: {}", kills);
    }
}

/// Performs the strongest available skills until enemy is dead.
/// Enemy must be already in screen when function is called. (Wait for it to spawn *before* calling it).
fn kill_enemy() {
    'outer: while is_enemy_alive() {
        println!("Checking available skills");
        let mut available_skills = get_available_attack_skills();
        dbg!(&available_skills);

        for skill in available_skills {
            println!("Casting skill {:?}", skill);
            skill.cast();
            if !is_enemy_alive() {
                break 'outer;
            }
            thread::sleep(user::ATTACK_COOLDOWN + FAST_SLEEP);
        }
        thread::sleep(FAST_SLEEP);
    }

    // It's possible that the monster is still alive, but we can not see it
    // because the bar is almost completely white
    thread::sleep(user::ATTACK_COOLDOWN);
    attack(); // So we attack an extra time
}

fn kill_hard_enemy() {
    'outer: while is_enemy_alive() {
        let mut available_skills = get_available_defense_skills();
        available_skills.extend(get_available_attack_skills());

        for skill in available_skills {
            skill.cast();
            if !is_enemy_alive() {
                break 'outer;
            }
            thread::sleep(user::ATTACK_COOLDOWN);
        }
        thread::sleep(FAST_SLEEP);
    }

    // It's possible that the monster is still alive, but we can not see it
    // because the bar is almost completely white
    for _ in 0..3 {
        thread::sleep(user::ATTACK_COOLDOWN);
        attack(); // So we attack an extra time
    }
}

pub fn fast_kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    disable_idle_mode_if_needed();

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }

        while is_enemy_alive() {
            attack();
            thread::sleep(FAST_SLEEP);
        }
        println!("[LOG] Kill Counter: {}", kills);
    }
}

/// Kills `quantity` bosses in the Adventure Zone chosen.
/// Will disable Idle Mode if needed.
/// Requires the game to be in "Adventure" menu.
pub fn kill_bosses_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);

    disable_idle_mode_if_needed();

    let mut kill_counter = 0;
    while kill_counter < quantity {
        while !is_enemy_alive() {
            thread::sleep(FAST_SLEEP);
        }

        if is_enemy_boss() {
            println!("[LOG] Found a boss, fighting.");
        } else {
            println!("[LOG] Not a boss, skipping.");
            refresh_zone();
            continue;
        }

        kill_enemy();
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

fn get_available_defense_skills() -> Vec<AdventureSkill> {
    let mut available_skills = Vec::new();
    let mut add_if_available = |skill: AdventureSkill| {
        if skill.is_available() {
            available_skills.push(skill);
        }
    };
    add_if_available(*ULTIMATE_BUFF);
    add_if_available(*DEFENSIVE_BUFF);
    add_if_available(*HYPER_REGEN);
    add_if_available(*HEAL);
    add_if_available(*BLOCK);
    add_if_available(*PARRY);
    available_skills
}

/// Returns the list of available-to-cast skills in order of priority
fn get_available_attack_skills() -> Vec<AdventureSkill> {
    let mut available_skills = Vec::new();
    let mut add_if_available = |skill: AdventureSkill| {
        if skill.is_available() {
            available_skills.push(skill);
        }
    };
    add_if_available(*ULTIMATE_BUFF);
    add_if_available(*OFFENSIVE_BUFF);
    add_if_available(*CHARGE);
    add_if_available(*ULTIMATE_ATTACK);
    add_if_available(*PIERCING_ATTACK);
    add_if_available(*STRONG_ATTACK);
    add_if_available(*REGULAR_ATTACK);
    available_skills
}

#[test]
fn test_available_skills() {
    dbg!(get_available_attack_skills());
    loop {
        kill_hard_enemy();
    }
}

fn attack_or_defense_highest_available() {
    // Attempt to cast all skills, stronger first
    // This (generally) amounts to using the strongest skill available
    // TODO: refactor this to
    //       1 - Know which ones are in cooldown (get button pixel color)
    //       2 - Use skills in order for maximum effectiveness

    ULTIMATE_BUFF.cast();
    DEFENSIVE_BUFF.cast();
    HYPER_REGEN.cast();
    HEAL.cast();
    BLOCK.cast();
    PARRY.cast();
    OFFENSIVE_BUFF.cast();
    ULTIMATE_ATTACK.cast();
    PIERCING_ATTACK.cast();
    CHARGE.cast();
    STRONG_ATTACK.cast();
    REGULAR_ATTACK.cast();
}

pub fn attack_highest_available() {
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

pub fn attack() {
    send_key(Key::Layout('w')); // Regular attack
}

pub fn is_enemy_alive() -> bool {
    let color = get_pixel_rgb(*pixel::ENEMY_BAR_LEFT_PIXEL);
    // Depending on screen properties, the pixel we are looking at may be the main red color of
    // the bar, or the closest-to-black brown, so we check both of them (approximately, as the colors vary
    // a bit from screen to screen)
    pixel::approximately_equal(color, colors::ENEMY_ALIVE_RGB_MAIN)
        || pixel::approximately_equal(color, colors::ENEMY_ALIVE_RGB_SECONDARY)
}

#[test]
fn test_enemy_alive_pixel() {
    loop {
        println!("{}", is_enemy_alive());
    }
}

pub fn is_enemy_boss() -> bool {
    let color = get_pixel_rgb(*coords::BOSS_CROWN_PIXEL);
    pixel::approximately_equal(color, colors::BOSS_CROWN_RGB)
}

pub fn is_idle_mode() -> bool {
    let color = get_pixel_rgb(*pixel::IDLE_MODE_PIXEL);
    pixel::approximately_equal(color, colors::IDLE_MODE_ON_RGB)
}

#[derive(Debug, Clone, Copy)]
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
            3 => current_color != colors::THIRD_ROW_COOLDOWN_RGB,
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
    static ref HYPER_REGEN: AdventureSkill =
        AdventureSkill::new(keys::HYPER_REGEN, *coords::HYPER_REGEN_PIXEL, 2);
}

/// Performs a specific routine to kill one boss at desired zone.
/// 1. Wait for 100% HP in Safe Zone
/// 2. Use Charge
/// 3. Recharge Charge cooldown
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
        cast_when_available(&CHARGE);
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
    // cast_when_available(&PARRY);

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
            kill_hard_enemy();
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
        snipe_boss_at_zone(AdventureZone::Mega);
    }
}
