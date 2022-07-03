use std::time::Duration;
use std::{thread, time};

use rdev::{simulate, Button, EventType, Key, SimulateError};

use crate::coords;

pub fn mouse_move((x, y): (u32, u32)) {
    let x = x + coords::CORNER.0;
    let y = y + coords::CORNER.1;
    send(&EventType::MouseMove {
        x: x.into(),
        y: y.into(),
    });
}

pub fn click() {
    send(&EventType::ButtonPress(Button::Left));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::ButtonRelease(Button::Left));
}

pub fn click_at(coords: (u32, u32)) {
    mouse_move(coords);
    click();
}

pub fn send_key(key: Key) {
    send(&EventType::KeyPress(key));
    thread::sleep(Duration::from_millis(20));
    send(&EventType::KeyRelease(key));
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
