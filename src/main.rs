use std::{thread, time};

mod coords;
mod input;
mod inventory;

fn main() {
    input::mouse_move(*coords::CORNER);

    loop {
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..3 {
            inventory::merge_slot(id as u32);
            thread::sleep(time::Duration::from_millis(300));
        }
        thread::sleep(time::Duration::from_secs(5));
    }
}
