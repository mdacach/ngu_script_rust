use std::thread;

use ngu_script::adventure::AdventureZone;
use ngu_script::menu::Menu;
use ngu_script::{adventure, constants, input, inventory, menu};

fn main() {
    thread::spawn(|| loop {
        menu::navigate(Menu::Adventure);
        // adventure::push_itopod();
        adventure::fast_kill_monsters_at_zone(50, AdventureZone::Sky);
        // adventure::go_to_zone(AdventureZone::Safe);

        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..constants::inventory::SLOTS_AVAILABLE {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
        inventory::boost_cube();
    });

    input::handle_user_termination();
}
