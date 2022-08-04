use std::thread;
use std::time::Duration;

use ngu_script::adventure::AdventureZone;
use ngu_script::menu::Menu;
use ngu_script::{adventure, menu};

fn main() {
    let script_routine = || {
        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);
            // This kills only one boss
            adventure::snipe_boss_at_zone(AdventureZone::Mega);
            adventure::go_to_zone(AdventureZone::Safe);
            thread::sleep(Duration::from_secs(10));

            kill_counter += 1;
            println!("[LOG] Total kill counter: {}", kill_counter);
        }
    };

    ngu_script::run(script_routine);
}
