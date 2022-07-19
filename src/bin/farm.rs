use std::thread;

use ngu_script::adventure::AdventureZone;
use ngu_script::inventory::{Boost, EquipmentSlot, Merge};
use ngu_script::menu::Menu;
use ngu_script::{adventure, constants, input, inventory, menu};

fn main() {
    let script_routine = || {
        // Get initial count of empty slots for tracking.
        menu::navigate(Menu::Inventory);
        let mut previous_empty_slots = inventory::count_empty_slots();

        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);

            let quantity = 50;
            adventure::fast_kill_monsters_at_zone(quantity, AdventureZone::AVSP);
            kill_counter += quantity;
            adventure::go_to_zone(AdventureZone::Safe);

            menu::navigate(Menu::Inventory);
            let inventory_routine = || {
                inventory::merge_equips();
                inventory::boost_equips();
                inventory::inventory_slots().take(36).for_each(|slot| {
                    slot.merge();
                    slot.boost();
                });
                inventory::merge_equip(EquipmentSlot::Cube);
            };
            println!("[LOG] Finished adventure routine");
            println!("[LOG] Total kill counter: {}", kill_counter);

            let empty_slots = inventory::count_empty_slots();
            let items_dropped = previous_empty_slots - empty_slots;
            println!("[LOG] Empty slots: {}", empty_slots);
            println!("[LOG] Items dropped: {}", items_dropped);
            if items_dropped > empty_slots {
                // There's no room for more items, so we must tidy up the inventory.
                println!("Performing Inventory routine");
                inventory_routine();
            }
            previous_empty_slots = inventory::count_empty_slots();
        }
    };

    thread::spawn(move || {
        script_routine();
    });

    input::handle_user_termination();
}
