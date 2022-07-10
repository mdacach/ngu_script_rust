use std::thread;

use rdev::{listen, Event, EventType, Key};

use crate::adventure::AdventureZone;
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
        adventure::kill_monsters_at_zone(15, AdventureZone::Sky);
        adventure::go_to_zone(AdventureZone::Safe);

        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..24 {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
        inventory::boost_cube();
    });

    handle_user_termination();
}

/// Handles script termination by listening for a "z" key press.
fn handle_user_termination() {
    // It's possible to terminate the script when the worker thread is in the middle of an action
    // e.g. Pressing a key, but not yet releasing it
    // This will cause the key to be pressed forever, and will be an issue for the OS
    // To work around that, all currently pressed keys are tracked, and this thread will
    // release those when terminating.

    // Keep track of currently pressed keys (or buttons).
    let mut presses = Vec::new();
    let event_handler = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(Key::KeyZ) => {
                // This hangs the working thread, but OK for now.
                // Note that the script will not press Z by itself, so we don't need
                // to track it in presses for releasing.
                presses.iter().for_each(release);
                println!("Terminating due to user input.");
                std::process::exit(0);
            }
            // Press events populate `presses`.
            EventType::KeyPress(other_key) => {
                presses.push(InputPress::Key(other_key));
            }
            EventType::ButtonPress(button) => {
                presses.push(InputPress::Button(button));
            }
            // Release events remove the corresponding key/button from `presses`.
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

    // Constantly listen for events.
    if let Err(e) = listen(event_handler) {
        println!("Error listening to events: {:?}", e);
    }
}
