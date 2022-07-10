use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;

use crate::coords::InGamePosition;
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

lazy_static! {
    static ref BASIC_TRAINING: InGamePosition = InGamePosition::from_coords(315, 30);
    static ref FIGHT_BOSS: InGamePosition = InGamePosition::from_coords(315, 70);
    static ref MONEY_PIT: InGamePosition = InGamePosition::from_coords(315, 110);
    static ref ADVENTURE: InGamePosition = InGamePosition::from_coords(315, 140);
    static ref INVENTORY: InGamePosition = InGamePosition::from_coords(315, 180);
    static ref AUGMENTATION: InGamePosition = InGamePosition::from_coords(315, 210);
}

/// Navigates between main menus.
pub fn navigate(menu: Menu) {
    match menu {
        Menu::BasicTraining => navigate_to(*BASIC_TRAINING),
        Menu::FightBoss => navigate_to(*FIGHT_BOSS),
        Menu::MoneyPit => navigate_to(*MONEY_PIT),
        Menu::Adventure => navigate_to(*ADVENTURE),
        Menu::Inventory => navigate_to(*INVENTORY),
        Menu::Augmentation => navigate_to(*AUGMENTATION),
    }
    // Give it time for the game to load
    thread::sleep(Duration::from_millis(100));
}

fn navigate_to(pos: InGamePosition) {
    click_at(pos);
}
