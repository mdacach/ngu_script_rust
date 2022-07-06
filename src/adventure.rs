use std::thread;
use std::time::Duration;

use rdev::Key;

use crate::input::send_key;
use crate::pixel;

pub fn kill_monsters() {
    let mut kill_counter = 0;
    while kill_counter < 10 {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
            // println!("Waiting for respawn.");
        }
        while is_enemy_alive() {
            attack();
            thread::sleep(Duration::from_millis(100));
        }
        kill_counter += 1;
        println!("[LOG] Kill Counter: {kill_counter}");
    }
    println!("Successfully killed 10 monsters");
}

fn attack() {
    send_key(Key::KeyW);
}

fn is_enemy_alive() -> bool {
    let color = pixel::get_pixel_rgb(pixel::ENEMY_BAR_LEFT.into());
    color == pixel::ENEMY_ALIVE_RGB
}
