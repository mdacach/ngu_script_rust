use std::thread;

use ngu_script::menu::Menu;
use ngu_script::{input, itopod, menu};

fn main() {
    let script_routine = || {
        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);
            itopod::fast_itopod(200);

            kill_counter += 1;
            println!("[LOG] Total kill counter: {}", kill_counter);
        }
    };

    ngu_script::run(script_routine);
}
