use std::thread;
use std::time::Duration;

use image::Rgb;
use rdev::Key;

use crate::coords::Position;
use crate::input::send_key;
use crate::pixel;
use crate::pixel::get_pixel_rgb;

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

fn refresh_zone() {
    send_key(Key::LeftArrow);
    send_key(Key::RightArrow);
}

fn attack_highest_available() {
    // Attempt to cast all skills, stronger first
    // This (generally) amounts to using the strongest skill available
    // TODO: refactor this to
    //       1 - Know which ones are in cooldown (get button pixel color)
    //       2 - Use skills in order for maximum effectiveness

    UltimateBuff::cast();
    OffensiveBuff::cast();
    Charge::cast();
    UltimateAttack::cast();
    PiercingAttack::cast();
    Parry::cast();
    StrongAttack::cast();
    RegularAttack::cast();

    // Defensive skills are not needed right now
    // Block::cast();
    // DefensiveBuff::cast();
    // Heal::cast();
}

fn attack() {
    send_key(Key::KeyW); // Regular attack
}

fn is_enemy_alive() -> bool {
    let color = pixel::get_pixel_rgb(pixel::ENEMY_BAR_LEFT_PIXEL.into());
    color == pixel::ENEMY_ALIVE_RGB
}

fn is_enemy_boss() -> bool {
    let color = pixel::get_pixel_rgb(Pixels::BOSS_CROWN.into());
    color == Colors::BOSS_CROWN_RGB
}

fn is_idle_mode() -> bool {
    let color = pixel::get_pixel_rgb(pixel::IDLE_MODE_PIXEL.into());
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

struct Pixels;

impl Pixels {
    const REGULAR_ATTACK: Position = Position::from_coords(620, 128);
    const STRONG_ATTACK: Position = Position::from_coords(768, 128);
    const PARRY: Position = Position::from_coords(906, 128);
    const PIERCING_ATTACK: Position = Position::from_coords(1051, 128);
    const ULTIMATE_ATTACK: Position = Position::from_coords(1189, 128);

    const BLOCK: Position = Position::from_coords(485, 175);
    const DEFENSIVE_BUFF: Position = Position::from_coords(631, 128);
    const HEAL: Position = Position::from_coords(766, 128);
    const OFFENSIVE_BUFF: Position = Position::from_coords(910, 128);
    const CHARGE: Position = Position::from_coords(1050, 128);
    const ULTIMATE_BUFF: Position = Position::from_coords(1190, 128);

    const BOSS_CROWN: Position = Position::from_coords(986, 377);
}

struct Colors;

impl Colors {
    const FIRST_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([124, 78, 78]);
    const SECOND_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([51, 68, 82]);
    const BOSS_CROWN_RGB: Rgb<u8> = Rgb([247, 239, 41]);
}

struct RegularAttack;

struct StrongAttack;

struct Parry;

struct PiercingAttack;

struct UltimateAttack;

struct Block;

struct DefensiveBuff;

struct Heal;

struct OffensiveBuff;

struct Charge;

struct UltimateBuff;

trait Skill {
    /// Returns true if skill is currently available to be used, false otherwise.
    fn is_available() -> bool;

    /// Attempts to cast the skill. Returns true if cast was successful.
    ///
    /// A cast is successful if the skill was ready (i.e is_available() is true).
    /// Otherwise, the cast fails and nothing happens.
    fn cast() -> bool;
}

impl Skill for RegularAttack {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::REGULAR_ATTACK.into());
        current_color != Colors::FIRST_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        // There's no need to try casting it if it's off.
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::REGULAR_ATTACK);
        true
        // TODO: We could check here if cast was successful by checking the pixel color again
        //       but I don't think the overhead (an extra call to get_pixel_rgb is justified right now).
    }
}

impl Skill for StrongAttack {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::STRONG_ATTACK.into());
        current_color != Colors::FIRST_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::STRONG_ATTACK);
        true
    }
}

impl Skill for Parry {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::PARRY.into());
        current_color != Colors::FIRST_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::PARRY);
        true
    }
}

impl Skill for PiercingAttack {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::PIERCING_ATTACK.into());
        current_color != Colors::FIRST_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::PIERCING_ATTACK);
        true
    }
}

impl Skill for UltimateAttack {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::ULTIMATE_ATTACK.into());
        current_color != Colors::FIRST_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::ULTIMATE_ATTACK);
        true
    }
}

impl Skill for Block {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::BLOCK.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::BLOCK);
        true
    }
}

impl Skill for DefensiveBuff {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::DEFENSIVE_BUFF.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::DEFENSIVE_BUFF);
        true
    }
}

impl Skill for Heal {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::HEAL.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::HEAL);
        true
    }
}

impl Skill for OffensiveBuff {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::OFFENSIVE_BUFF.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::OFFENSIVE_BUFF);
        true
    }
}

impl Skill for Charge {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::CHARGE.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::CHARGE);
        true
    }
}

impl Skill for UltimateBuff {
    fn is_available() -> bool {
        let current_color = get_pixel_rgb(Pixels::ULTIMATE_BUFF.into());
        current_color != Colors::SECOND_ROW_COOLDOWN_RGB
    }

    fn cast() -> bool {
        if !Self::is_available() {
            return false;
        }

        send_key(Keys::ULTIMATE_BUFF);
        true
    }
}
