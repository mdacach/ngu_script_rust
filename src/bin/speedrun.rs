use std::arch::x86_64::_mm_extract_si64;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use enigo::Key;

use ngu_script::coords::GameAwarePosition;
use ngu_script::input::{click_at, send_key};
use ngu_script::menu::Menu;
use ngu_script::*;
use ngu_script::{itopod, menu};

fn main() {
    let script_routine = || {
        let big_number = 1_000_000_000;
        let basic_training = GameAwarePosition::from_coords(1122, 186);
        let idle_half = GameAwarePosition::from_coords(1156, 26);
        let rebirth_button = GameAwarePosition::from_coords(734, 700);
        let confirm = GameAwarePosition::from_coords(578, 430);

        let augment = 3;

        loop {
            let start = Instant::now();
            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::BasicTraining);
            thread::sleep(Duration::from_secs(2));
            click_at(basic_training);

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            thread::sleep(Duration::from_secs(6));
            for _ in 0..5 {
                fight_boss::fight();
                thread::sleep(Duration::from_secs(2));
            }

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::Adventure);
            adventure::go_to_latest();
            adventure::turn_on_idle_mode();

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::TimeMachine);
            time_machine::add_energy(big_number);
            thread::sleep(Duration::from_secs(2));
            time_machine::add_magic(big_number);

            while start.elapsed().as_secs() < 60 {
                menu::navigate(Menu::TimeMachine);
                time_machine::add_energy(big_number);
                thread::sleep(Duration::from_secs(2));
                time_machine::add_magic(big_number);
                thread::sleep(Duration::from_secs(2));
            }

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            thread::sleep(Duration::from_secs(6));
            for _ in 0..5 {
                fight_boss::fight();
                thread::sleep(Duration::from_secs(2));
            }

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::Adventure);
            thread::sleep(Duration::from_secs(2));
            adventure::go_to_latest();
            thread::sleep(Duration::from_secs(2));
            adventure::turn_on_idle_mode();
            thread::sleep(Duration::from_secs(2));

            // reclaim
            menu::navigate(Menu::Augmentation);
            send_key(Key::Layout('r'));
            thread::sleep(Duration::from_secs(2));
            while start.elapsed().as_secs() < 150 {
                menu::navigate(Menu::Augmentation);
                thread::sleep(Duration::from_secs(2));
                click_at(idle_half);
                thread::sleep(Duration::from_secs(2));

                augments::add_augment(augment);
                thread::sleep(Duration::from_secs(2));
                augments::add_enhancement(augment);

                thread::sleep(Duration::from_secs(2));
                menu::navigate(Menu::BloodMagic);
                blood_magic::add_ritual(3);

                thread::sleep(Duration::from_secs(2));
                menu::navigate(Menu::FightBoss);
                fight_boss::nuke();
                thread::sleep(Duration::from_secs(2));
            }

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::Adventure);
            thread::sleep(Duration::from_secs(2));
            adventure::go_to_latest();
            thread::sleep(Duration::from_secs(2));
            adventure::turn_on_idle_mode();

            thread::sleep(Duration::from_secs(2));
            menu::navigate(Menu::BloodMagic);
            blood_magic::add_ritual(3);
            thread::sleep(Duration::from_secs(2));

            while start.elapsed().as_secs() < 182 {
                thread::sleep(Duration::from_secs(1));
            }
            menu::navigate(Menu::Rebirth);
            thread::sleep(Duration::from_secs(2));
            click_at(rebirth_button);
            thread::sleep(Duration::from_secs(2));
            click_at(confirm);
            thread::sleep(Duration::from_secs(2));
        }
    };

    ngu_script::run(script_routine);
}
