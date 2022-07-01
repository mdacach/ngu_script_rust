use std::{thread, time};

mod coords;
mod input;
mod inventory;

fn main() {
    input::mouse_move(*coords::CORNER);

    for id in 0..24 {
        inventory::move_to_slot(id as u32);
        thread::sleep(time::Duration::from_millis(300));
    }
}
