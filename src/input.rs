use std::thread;
use std::time::Duration;

use rdev::{listen, simulate, Button, Event, EventType, Key, SimulateError};

use crate::coords::GameAwarePosition;

pub fn mouse_move(pos: GameAwarePosition) {
    send(&EventType::MouseMove {
        x: pos.x.into(),
        y: pos.y.into(),
    });
}

pub fn click() {
    send(&EventType::ButtonPress(Button::Left));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::ButtonRelease(Button::Left));
}

pub fn right_click() {
    send(&EventType::ButtonPress(Button::Right));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::ButtonRelease(Button::Right));
}

pub fn right_click_at(pos: GameAwarePosition) {
    mouse_move(pos);
    right_click();
}

pub fn click_at(pos: GameAwarePosition) {
    mouse_move(pos);
    click();
}

pub fn send_key(key: Key) {
    send(&EventType::KeyPress(key));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::KeyRelease(key));
}

#[derive(PartialEq)]
pub enum InputPress {
    Key(Key),
    Button(Button),
}

pub fn release(input: &InputPress) {
    match input {
        InputPress::Key(k) => send(&EventType::KeyRelease(*k)),
        InputPress::Button(b) => send(&EventType::ButtonRelease(*b)),
    }
}

fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    thread::sleep(delay);
}

/// Handles script termination by listening for a "z" key press.
pub fn handle_user_termination() {
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
