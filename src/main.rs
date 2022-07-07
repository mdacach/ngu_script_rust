use std::{thread, time};

use rdev::{listen, Button, Event, EventType, Key};

use crate::menu::Menu;

mod adventure;
mod coords;
mod input;
mod inventory;
mod menu;
mod pixel;

fn main() {
    thread::spawn(|| loop {
        menu::navigate(Menu::Adventure);
        adventure::kill_monsters(25);

        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..24 {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
    });

    if let Err(e) = listen(input::handle_user_termination) {
        println!("Error listening to events: {:?}", e);
    }
}
