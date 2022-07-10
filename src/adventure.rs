use std::thread;
use std::time::Duration;

use image::Rgb;
use lazy_static::lazy_static;
use rdev::Key;

use crate::coords::InGamePosition;
use crate::input::{right_click_at, send_key};
use crate::pixel;
use crate::pixel::get_pixel_rgb;

/// Kills `quantity` monsters in the current Adventure Zone. Will disable Idle Mode if needed.
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

/// Kills `quantity` bosses in the current Adventure Zone. Will disable Idle Mode if needed.
pub fn kill_bosses(quantity: u16) {
    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    let mut kill_counter = 0;
    while kill_counter < quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
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
            thread::sleep(Duration::from_millis(50));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        thread::sleep(Duration::from_millis(1050));
        attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill Counter: {kill_counter}");
    }
}

/// Kills `quantity` monsters in the Adventure Zone chosen. Will disable Idle Mode if needed.
pub fn kill_monsters_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);
    kill_monsters(quantity);
}

/// Kills `quantity` bosses in the Adventure Zone chosen. Will disable Idle Mode if needed.
pub fn kill_bosses_at_zone(quantity: u16, zone: AdventureZone) {
    go_to_zone(zone);
    kill_bosses(quantity);
}

/// Zones in Adventure Menu.
///
/// Note that order here is important, as we use the zone
/// numbering as the amount of right presses to get there.
pub enum AdventureZone {
    Safe,
    Tutorial,
    Sewers,
    Forest,
    Cave,
    Sky,
    HSB,
}

pub fn go_to_zone(zone: AdventureZone) {
    // First we must start from 0: The Safe Zone
    // Right clicking adventure's left arrow makes us go to Safe
    right_click_at(*RETREAT_ZONE_PIXEL);

    // Then, we get how much to go forward by Zone Numbering
    let forward_steps = zone as u8;
    for _ in 0..forward_steps {
        advance_zone();
    }
}

fn go_to_latest() {
    // Right clicking adventure's right arrow makes us to go to latest available zone
    right_click_at(*ADVANCE_ZONE_PIXEL);
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
    let color = get_pixel_rgb(*BOSS_CROWN_PIXEL);
    color == Colors::BOSS_CROWN_RGB
}

fn is_idle_mode() -> bool {
    let color = get_pixel_rgb(*pixel::IDLE_MODE_PIXEL);
    color == pixel::IDLE_MODE_ON_RGB
}

/// Adventure mode skill keys.
#[non_exhaustive]
struct Keys;

impl Keys {
    const REGULAR_ATTACK: Key = Key::KeyW;
    const STRONG_ATTACK: Key = Key::KeyE;
    const PARRY: Key = Key::KeyR;
    const PIERCING_ATTACK: Key = Key::KeyT;
    const ULTIMATE_ATTACK: Key = Key::KeyY;

    const BLOCK: Key = Key::KeyA;
    const DEFENSIVE_BUFF: Key = Key::KeyS;
    const HEAL: Key = Key::KeyD;
    const OFFENSIVE_BUFF: Key = Key::KeyF;
    const CHARGE: Key = Key::KeyG;
    const ULTIMATE_BUFF: Key = Key::KeyH;
}

struct AdventureSkill {
    key: Key,
    pixel_coords: InGamePosition,
    row_number: u8,
}

impl AdventureSkill {
    const fn new(key: Key, pixel_coords: InGamePosition, row_number: u8) -> Self {
        Self {
            key,
            pixel_coords,
            row_number,
        }
    }
}

lazy_static! {
    static ref REGULAR_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(620, 128);
    static ref STRONG_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(768, 128);
    static ref PARRY_PIXEL: InGamePosition = InGamePosition::from_coords(906, 128);
    static ref PIERCING_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(1051, 128);
    static ref ULTIMATE_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(1189, 128);
    static ref BLOCK_PIXEL: InGamePosition = InGamePosition::from_coords(485, 175);
    static ref DEFENSIVE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(631, 128);
    static ref HEAL_PIXEL: InGamePosition = InGamePosition::from_coords(766, 128);
    static ref OFFENSIVE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(910, 128);
    static ref CHARGE_PIXEL: InGamePosition = InGamePosition::from_coords(1050, 128);
    static ref ULTIMATE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(1190, 128);
    static ref BOSS_CROWN_PIXEL: InGamePosition = InGamePosition::from_coords(986, 377);
    static ref RETREAT_ZONE_PIXEL: InGamePosition = InGamePosition::from_coords(976, 283);
    static ref ADVANCE_ZONE_PIXEL: InGamePosition = InGamePosition::from_coords(1257, 283);
}

struct Colors;

impl Colors {
    const FIRST_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([124, 78, 78]);
    const SECOND_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([51, 68, 82]);
    const BOSS_CROWN_RGB: Rgb<u8> = Rgb([247, 239, 41]);
}

impl Skill for AdventureSkill {
    fn is_available(&self) -> bool {
        let current_color = get_pixel_rgb(self.pixel_coords.into());
        match self.row_number {
            1 => current_color != Colors::FIRST_ROW_COOLDOWN_RGB,
            2 => current_color != Colors::SECOND_ROW_COOLDOWN_RGB,
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

lazy_static! {
    static ref REGULAR_ATTACK: AdventureSkill =
        AdventureSkill::new(Keys::REGULAR_ATTACK, *REGULAR_ATTACK_PIXEL, 1);
    static ref STRONG_ATTACK: AdventureSkill =
        AdventureSkill::new(Keys::STRONG_ATTACK, *STRONG_ATTACK_PIXEL, 1);
    static ref PARRY: AdventureSkill = AdventureSkill::new(Keys::PARRY, *PARRY_PIXEL, 1);
    static ref PIERCING_ATTACK: AdventureSkill =
        AdventureSkill::new(Keys::PIERCING_ATTACK, *PIERCING_ATTACK_PIXEL, 1);
    static ref ULTIMATE_ATTACK: AdventureSkill =
        AdventureSkill::new(Keys::ULTIMATE_ATTACK, *ULTIMATE_ATTACK_PIXEL, 1);
    static ref BLOCK: AdventureSkill = AdventureSkill::new(Keys::BLOCK, *BLOCK_PIXEL, 2);
    static ref DEFENSIVE_BUFF: AdventureSkill =
        AdventureSkill::new(Keys::DEFENSIVE_BUFF, *DEFENSIVE_BUFF_PIXEL, 2);
    static ref HEAL: AdventureSkill = AdventureSkill::new(Keys::HEAL, *HEAL_PIXEL, 2);
    static ref OFFENSIVE_BUFF: AdventureSkill =
        AdventureSkill::new(Keys::OFFENSIVE_BUFF, *OFFENSIVE_BUFF_PIXEL, 2);
    static ref CHARGE: AdventureSkill = AdventureSkill::new(Keys::CHARGE, *CHARGE_PIXEL, 2);
    static ref ULTIMATE_BUFF: AdventureSkill =
        AdventureSkill::new(Keys::ULTIMATE_BUFF, *ULTIMATE_BUFF_PIXEL, 2);
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
