use std::thread;
use std::time::Duration;

use rdev::Key;

use crate::input::send_key;
use crate::pixel;

pub fn kill_monsters(quantity: u16) {
    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
        }

        while is_enemy_alive() {
            attack();
            thread::sleep(Duration::from_millis(100));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        attack(); // So we attack an extra time
        println!("[LOG] Kill Counter: {kills}");
    }
}

fn attack() {
    send_key(Key::KeyW); // Regular attack
}

fn is_enemy_alive() -> bool {
    let color = pixel::get_pixel_rgb(pixel::ENEMY_BAR_LEFT.into());
    color == pixel::ENEMY_ALIVE_RGB
}
