use std::thread;
use std::time::Duration;

use rdev::{listen, simulate, Button, Event, EventType, Key, SimulateError};

use crate::constants;
use crate::coords::GameAwarePosition;

/// Moves the mouse to `pos`.
pub fn mouse_move(pos: GameAwarePosition) {
    send(&EventType::MouseMove {
        x: pos.x.into(),
        y: pos.y.into(),
    });
}

/// Left-clicks on current position.
pub fn click() {
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}

/// Right-clicks on current position.
pub fn right_click() {
    send(&EventType::ButtonPress(Button::Right));
    send(&EventType::ButtonRelease(Button::Right));
}

/// Moves the mouse to `pos` and right-clicks.
pub fn right_click_at(pos: GameAwarePosition) {
    mouse_move(pos);
    right_click();
}

/// Moves the mouse to `pos` and left-clicks.
pub fn click_at(pos: GameAwarePosition) {
    mouse_move(pos);
    click();
}

/// Sends `key` as if it were pressed by the keyboard.
pub fn send_key(key: Key) {
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
}

/// Represents a object that can perform a "press" event.
///
/// This is used to handle "pressed" keys that were not "released".
#[derive(PartialEq)]
pub enum InputPress {
    Key(Key),
    Button(Button),
}

/// "Releases" a pressed input.
/// If you have pressed "a" before, it will stay pressed until it is released.
/// Note that when typing on a keyboard, you release the key by manually letting go of the physical key.
pub fn release(input: &InputPress) {
    match input {
        InputPress::Key(k) => send(&EventType::KeyRelease(*k)),
        InputPress::Button(b) => send(&EventType::ButtonRelease(*b)),
    }
}

/// Wrapper around `rdev` functionality.
fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    thread::sleep(constants::FAST_SLEEP);
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
