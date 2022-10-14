use ngu_script::menu::Menu;
use ngu_script::{itopod, inventory, menu};
use ngu_script::inventory::{Boost, EquipmentSlot, Merge};

fn main() {
    let script_routine = || {
        let mut kill_counter = 0;
        loop {
            menu::navigate(Menu::Adventure);
            itopod::push_itopod();

            kill_counter += 1;
            println!("[LOG] Total kill counter: {}", kill_counter);
            menu::navigate(Menu::Inventory);
            let inventory_routine = || {
                inventory::merge_equips();
                inventory::boost_equips();
                inventory::inventory_slots().take(5).for_each(|slot| {
                    slot.merge();
                    slot.boost();
                });
                inventory::boost_equip(EquipmentSlot::Cube);
            };
                inventory_routine();
        }
    };

    ngu_script::run(script_routine);
}
