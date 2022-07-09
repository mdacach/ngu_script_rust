use std::{thread, time};

use rdev::{listen, Button, Event, EventType, Key};

use crate::input::{release, InputPress};
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
        // adventure::kill_monsters(25);
        adventure::kill_bosses(10);

        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..24 {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
        inventory::boost_cube();
    });

    let mut presses = Vec::new();
    let event_handler = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(Key::KeyZ) => {
                // This hangs the working thread, but OK for now.
                // Note the script will not press Z by itself, so we don't need
                // to track it in PRESSES for releasing
                presses.iter().for_each(release);
                println!("Terminating due to user input.");
                std::process::exit(0);
            }
            EventType::KeyPress(other_key) => {
                presses.push(InputPress::Key(other_key));
            }
            EventType::ButtonPress(button) => {
                presses.push(InputPress::Button(button));
            }
            EventType::KeyRelease(other_key) => {
                let index = presses
                    .iter()
                    .position(|x| x == &InputPress::Key(other_key))
                    .unwrap();
                presses.swap_remove(index);
            }
            EventType::ButtonRelease(button) => {
                let index = presses
                    .iter()
                    .position(|x| x == &InputPress::Button(button))
                    .unwrap();
                presses.swap_remove(index);
            }
            _ => (),
        }
    };

    if let Err(e) = listen(event_handler) {
        println!("Error listening to events: {:?}", e);
    }
}
