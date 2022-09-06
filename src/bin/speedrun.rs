use std::arch::x86_64::_mm_extract_si64;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use enigo::Key;

use ngu_script::constants::gold_diggers::coords::{CAP_STAT, CAP_WANDOOS};
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
        let mid_menu_sleep = Duration::from_secs(1);

        let blood_magic = 4;
        let augment = 3;

        loop {
            let start = Instant::now();
            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::BasicTraining);
            thread::sleep(mid_menu_sleep);
            click_at(basic_training);

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            thread::sleep(Duration::from_secs(4));
            for _ in 0..5 {
                fight_boss::fight();
                thread::sleep(Duration::from_secs(1));
            }

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::Adventure);
            thread::sleep(mid_menu_sleep);
            adventure::go_to_latest();
            thread::sleep(mid_menu_sleep);
            adventure::turn_on_idle_mode();

            while start.elapsed().as_secs() < 60 {
                menu::navigate(Menu::TimeMachine);
                thread::sleep(mid_menu_sleep);
                time_machine::add_energy(big_number);
                thread::sleep(mid_menu_sleep);
                time_machine::add_magic(big_number);
                thread::sleep(mid_menu_sleep);
            }

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            thread::sleep(Duration::from_secs(6));
            for _ in 0..5 {
                fight_boss::fight();
                thread::sleep(Duration::from_secs(2));
            }

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::Adventure);
            thread::sleep(mid_menu_sleep);
            adventure::go_to_latest();
            thread::sleep(mid_menu_sleep);

            // reclaim
            menu::navigate(Menu::Augmentation);
            thread::sleep(mid_menu_sleep);
            send_key(Key::Layout('r'));
            thread::sleep(mid_menu_sleep);
            send_key(Key::Layout('t'));
            thread::sleep(mid_menu_sleep);

            menu::navigate(Menu::Wandoos);
            input::input_number(100_000_000);
            thread::sleep(mid_menu_sleep);
            wandoos::add_energy();
            thread::sleep(mid_menu_sleep);
            input::input_number(10_000_000);
            thread::sleep(mid_menu_sleep);
            wandoos::add_magic();
            thread::sleep(mid_menu_sleep);

            while start.elapsed().as_secs() < 150 {
                menu::navigate(Menu::Augmentation);
                thread::sleep(mid_menu_sleep);
                click_at(idle_half);
                thread::sleep(mid_menu_sleep);

                augments::add_augment(augment);
                thread::sleep(mid_menu_sleep);
                augments::add_enhancement(augment);

                thread::sleep(mid_menu_sleep);
                menu::navigate(Menu::BloodMagic);
                blood_magic::add_ritual(blood_magic);

                thread::sleep(mid_menu_sleep);
                menu::navigate(Menu::FightBoss);
                fight_boss::nuke();
                thread::sleep(mid_menu_sleep);
            }

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::Adventure);
            thread::sleep(mid_menu_sleep);
            adventure::go_to_latest();
            thread::sleep(mid_menu_sleep);

            menu::navigate(Menu::GoldDiggers);
            thread::sleep(mid_menu_sleep);
            click_at(*CAP_STAT);
            thread::sleep(mid_menu_sleep);
            click_at(*CAP_WANDOOS);

            thread::sleep(mid_menu_sleep);
            menu::navigate(Menu::FightBoss);
            while start.elapsed().as_secs() < 178 {
                fight_boss::fight();
                thread::sleep(mid_menu_sleep);
            }
            fight_boss::stop();

            menu::navigate(Menu::Rebirth);
            thread::sleep(mid_menu_sleep);
            click_at(rebirth_button);
            thread::sleep(mid_menu_sleep);
            click_at(confirm);
            thread::sleep(mid_menu_sleep);
        }
    };

    ngu_script::run(script_routine);
}
