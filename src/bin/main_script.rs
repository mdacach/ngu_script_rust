use std::thread;

use ngu_script::adventure::AdventureZone;
use ngu_script::menu::Menu;
use ngu_script::{adventure, input, inventory, menu};

fn main() {
    thread::spawn(|| loop {
        menu::navigate(Menu::Adventure);
        adventure::fast_itopod(25);
        // adventure::kill_monsters_at_zone(25, AdventureZone::Forest);
        // adventure::go_to_zone(AdventureZone::Safe);

        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..24 {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
        inventory::boost_cube();
    });

    input::handle_user_termination();
}
