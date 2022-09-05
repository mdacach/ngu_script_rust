use std::thread;

use crate::constants;
use crate::constants::menu::*;
use crate::coords::GameAwarePosition;
use crate::input::click_at;

// Represents a main menu that you can navigate to.
pub enum Menu {
    BasicTraining,
    FightBoss,
    MoneyPit,
    Adventure,
    Inventory,
    Augmentation,
    AdvTraining,
    TimeMachine,
    BloodMagic,
    Wandoos,
    NGU,
    Yggdrasil,
    Rebirth,
    SpendEXP,
}

/// Navigates between menus.
pub fn navigate(menu: Menu) {
    match menu {
        Menu::BasicTraining => navigate_to(*coords::BASIC_TRAINING),
        Menu::FightBoss => navigate_to(*coords::FIGHT_BOSS),
        Menu::MoneyPit => navigate_to(*coords::MONEY_PIT),
        Menu::Adventure => navigate_to(*coords::ADVENTURE),
        Menu::Inventory => navigate_to(*coords::INVENTORY),
        Menu::Augmentation => navigate_to(*coords::AUGMENTATION),
        Menu::AdvTraining => navigate_to(*coords::ADV_TRAINING),
        Menu::TimeMachine => navigate_to(*coords::TIME_MACHINE),
        Menu::BloodMagic => navigate_to(*coords::BLOOD_MAGIC),
        Menu::Wandoos => navigate_to(*coords::WANDOOS),
        Menu::NGU => navigate_to(*coords::NGU),
        Menu::Yggdrasil => navigate_to(*coords::YGGDRASIL),
        Menu::Rebirth => navigate_to(*coords::REBIRTH),
        Menu::SpendEXP => navigate_to(*coords::SPEND_EXP),
    }
    // Give it time for the game to load
    thread::sleep(constants::user::LONG_SLEEP);
}

fn navigate_to(pos: GameAwarePosition) {
    click_at(pos);
}
