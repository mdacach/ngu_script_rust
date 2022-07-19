use std::thread;

use ngu_script::adventure::AdventureZone;
use ngu_script::menu::Menu;
use ngu_script::{adventure, input, inventory, menu};

fn main() {
    let script_routine = || {
        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);
            // This kills only one boss
            adventure::snipe_boss_at_zone(AdventureZone::Mega);
            adventure::go_to_zone(AdventureZone::Safe);

            kill_counter += 1;
            println!("[LOG] Total kill counter: {}", kill_counter);
        }
    };

    thread::spawn(move || {
        script_routine();
    });

    input::handle_user_termination();
}
