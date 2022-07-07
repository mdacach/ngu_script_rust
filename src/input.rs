use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use lazy_static::lazy_static;
use rdev::{simulate, Button, Event, EventType, Key, SimulateError};

use crate::coords::InGamePosition;

pub fn mouse_move(pos: InGamePosition) {
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

pub fn right_click_at(pos: InGamePosition) {
    mouse_move(pos);
    right_click();
}

pub fn click_at(pos: InGamePosition) {
    mouse_move(pos);
    click();
}

pub fn send_key(key: Key) {
    send(&EventType::KeyPress(key));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::KeyRelease(key));
}

fn release(input: &InputPress) {
    match input {
        InputPress::Key(k) => send(&EventType::KeyRelease(*k)),
        InputPress::Button(b) => send(&EventType::ButtonRelease(*b)),
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    thread::sleep(delay);
}

lazy_static! {
    static ref PRESSES: Mutex<Vec<InputPress>> = Mutex::new(Vec::new());
}

#[derive(PartialEq)]
enum InputPress {
    Key(Key),
    Button(Button),
}

pub fn handle_user_termination(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::KeyZ) => {
            // This hangs the working thread, but OK for now.
            // Note the script will not press Z by itself, so we don't need
            // to track it in PRESSES for releasing
            let presses = PRESSES.lock().unwrap();
            presses.iter().for_each(release);
            println!("Terminating due to user input.");
            std::process::exit(0);
        }
        EventType::KeyPress(other_key) => {
            PRESSES.lock().unwrap().push(InputPress::Key(other_key));
        }
        EventType::ButtonPress(button) => {
            PRESSES.lock().unwrap().push(InputPress::Button(button));
        }
        EventType::KeyRelease(other_key) => {
            let mut presses = PRESSES.lock().unwrap();
            let index = presses
                .iter()
                .position(|x| x == &InputPress::Key(other_key))
                .unwrap();
            presses.swap_remove(index);
        }
        EventType::ButtonRelease(button) => {
            let mut presses = PRESSES.lock().unwrap();
            let index = presses
                .iter()
                .position(|x| x == &InputPress::Button(button))
                .unwrap();
            presses.swap_remove(index);
        }
        _ => (),
    }
}
