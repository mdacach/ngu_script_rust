use std::thread;
use std::time::Duration;

use crate::coords::{InGamePosition, Position};
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

const BASIC_TRAINING: Position = Position::from_coords(315, 30);
const FIGHT_BOSS: Position = Position::from_coords(315, 70);
const MONEY_PIT: Position = Position::from_coords(315, 110);
const ADVENTURE: Position = Position::from_coords(315, 140);
const INVENTORY: Position = Position::from_coords(315, 180);
const AUGMENTATION: Position = Position::from_coords(315, 210);

/// Navigates between main menus.
pub fn navigate(menu: Menu) {
    match menu {
        Menu::BasicTraining => navigate_to(BASIC_TRAINING.into()),
        Menu::FightBoss => navigate_to(FIGHT_BOSS.into()),
        Menu::MoneyPit => navigate_to(MONEY_PIT.into()),
        Menu::Adventure => navigate_to(ADVENTURE.into()),
        Menu::Inventory => navigate_to(INVENTORY.into()),
        Menu::Augmentation => navigate_to(AUGMENTATION.into()),
    }
    // Give it time for the game to load
    thread::sleep(Duration::from_millis(100));
}

fn navigate_to(pos: InGamePosition) {
    click_at(pos);
}
