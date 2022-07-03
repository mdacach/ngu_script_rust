use std::{thread, time};

use rdev::{listen, Event, EventType, Key};

mod coords;
mod input;
mod inventory;

fn main() {
    // input::mouse_move(*coords::CORNER);

    thread::spawn(|| {
        loop {
            // check_user_termination();
            inventory::merge_equips();
            inventory::boost_equips();
            for id in 0..24 {
                inventory::merge_slot(id as u32);
                inventory::boost_slot(id as u32);
            }
            thread::sleep(time::Duration::from_secs(10));
        }
    });

    if let Err(e) = listen(check_for_user_termination) {
        println!("Error listening to events: {:?}", e);
    }
}

fn check_for_user_termination(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::KeyZ) => {
            println!("Terminating due to user input.");
            // This hangs the working thread, but OK for now.
            std::process::exit(0);
        }
        EventType::KeyPress(other_key) => {
            println!("{:?} pressed.", other_key);
        }
        EventType::ButtonPress(button) => {
            println!("{:?} pressed.", button);
        }
        EventType::KeyRelease(other_key) => {
            println!("{:?} released.", other_key);
        }
        EventType::ButtonRelease(button) => {
            println!("{:?} released.", button);
        }
        _ => (),
    }
}
