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

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
