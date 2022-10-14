use ngu_script::adventure::AdventureZone;
use ngu_script::inventory::{Boost, EquipmentSlot, Merge};
use ngu_script::menu::Menu;
use ngu_script::{adventure, inventory, menu};

fn main() {
    let script_routine = || {
        // Get initial count of empty slots for tracking.
        menu::navigate(Menu::Inventory);
        let mut previous_empty_slots = inventory::count_empty_slots();
        println!("Initial empty slots: {}", previous_empty_slots);

        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);

            let quantity = 40;
            adventure::fast_kill_monsters_at_zone(quantity, AdventureZone::BDW);
            kill_counter += quantity;
            adventure::go_to_zone(AdventureZone::Safe);

            menu::navigate(Menu::Inventory);
            let inventory_routine = || {
                inventory::merge_equips();
                inventory::boost_equips();
                inventory::inventory_slots().take(30).for_each(|slot| {
                    slot.merge();
                    slot.boost();
                });
                inventory::boost_equip(EquipmentSlot::Cube);
            };
            println!("[LOG] Total kill counter: {}", kill_counter);
                inventory_routine();
        }
    };

    ngu_script::run(script_routine);
}
