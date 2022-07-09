use std::time::Duration;
use std::{thread, time};

use rdev::{simulate, Button, EventType, Key, SimulateError};

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
