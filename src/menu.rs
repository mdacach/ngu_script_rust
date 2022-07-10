use std::thread;
use std::time::Duration;

use crate::constants::menu::*;
use crate::coords::GameAwarePosition;
use crate::input::click_at;

// Represents a main menu.
pub enum Menu {
    BasicTraining,
    FightBoss,
    MoneyPit,
    Adventure,
    Inventory,
    Augmentation,
}

/// Navigates between main menus.
pub fn navigate(menu: Menu) {
    match menu {
        Menu::BasicTraining => navigate_to(*coords::BASIC_TRAINING),
        Menu::FightBoss => navigate_to(*coords::FIGHT_BOSS),
        Menu::MoneyPit => navigate_to(*coords::MONEY_PIT),
        Menu::Adventure => navigate_to(*coords::ADVENTURE),
        Menu::Inventory => navigate_to(*coords::INVENTORY),
        Menu::Augmentation => navigate_to(*coords::AUGMENTATION),
    }
    // Give it time for the game to load
    thread::sleep(Duration::from_millis(100));
}

fn navigate_to(pos: GameAwarePosition) {
    click_at(pos);
}
