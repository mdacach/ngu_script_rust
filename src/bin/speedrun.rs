use std::thread;
use std::time::{Duration, Instant};

use ngu_script::constants::gold_diggers::coords::{CAP_STAT, CAP_WANDOOS};
use ngu_script::coords::GameAwarePosition;
use ngu_script::input::click_at;
use ngu_script::menu::Menu;
use ngu_script::*;
use ngu_script::{itopod, menu};

fn main() {
    let script_routine = || {
        let basic_training = GameAwarePosition::from_coords(1122, 186);
        let energy_idle_half = GameAwarePosition::from_coords(1156, 26);
        let energy_idle_quarter = GameAwarePosition::from_coords(1200, 26);
        let magic_idle_half = GameAwarePosition::from_coords(1156, 60);
        let magic_idle_quarter = GameAwarePosition::from_coords(1200, 60);
        let rebirth_button = GameAwarePosition::from_coords(734, 700);
        let confirm = GameAwarePosition::from_coords(578, 430);
        let mid_menu_sleep = Duration::from_millis(450);

        let pit_feed = GameAwarePosition::from_coords(784, 324);
        let pit_confirm = GameAwarePosition::from_coords(580, 422);
        let feed_money_pit = || {
            menu::navigate(Menu::MoneyPit);
            thread::sleep(mid_menu_sleep);
            click_at(pit_feed);
            thread::sleep(mid_menu_sleep);
            click_at(pit_confirm);
            thread::sleep(mid_menu_sleep);
        };

        let blood_magic = 4;
        let augment = 3;

        let script_start = Instant::now();
        let mut run_id = 1;

        let get_exp = || {
            menu::navigate(Menu::SpendEXP);
            thread::sleep(mid_menu_sleep);
            exp::get_unspent_exp().ok()
        };

        let script_start_exp = get_exp();
        println!("Starting script exp: {:#?}", script_start_exp);

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
            // wait for a kill
            thread::sleep(Duration::from_secs(5));
            itopod::enter_itoped_at_optimal_floor();
            thread::sleep(mid_menu_sleep);

            let mut loop_counter = 0;
            while start.elapsed().as_secs() < 155 {
                menu::navigate(Menu::TimeMachine);
                thread::sleep(mid_menu_sleep);
                click_at(energy_idle_quarter);
                thread::sleep(mid_menu_sleep);
                time_machine::add_energy();
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::Wandoos);
                thread::sleep(mid_menu_sleep);
                wandoos::add_energy();
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::Augmentation);
                thread::sleep(mid_menu_sleep);
                augments::add_augment(augment);
                thread::sleep(mid_menu_sleep);
                augments::add_enhancement(augment);
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::TimeMachine);
                thread::sleep(mid_menu_sleep);
                click_at(magic_idle_half);
                thread::sleep(mid_menu_sleep);
                time_machine::add_magic();
                thread::sleep(mid_menu_sleep);

                click_at(magic_idle_half);
                thread::sleep(mid_menu_sleep);
                menu::navigate(Menu::Wandoos);
                thread::sleep(mid_menu_sleep);
                wandoos::add_magic();
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::BloodMagic);
                thread::sleep(mid_menu_sleep);
                blood_magic::add_ritual(blood_magic);
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::GoldDiggers);
                thread::sleep(mid_menu_sleep);
                click_at(*CAP_WANDOOS);
                thread::sleep(mid_menu_sleep);
                click_at(*CAP_STAT);
                thread::sleep(mid_menu_sleep);

                menu::navigate(Menu::FightBoss);
                thread::sleep(mid_menu_sleep);
                fight_boss::nuke();
                thread::sleep(mid_menu_sleep);
                fight_boss::fight();
                thread::sleep(mid_menu_sleep);
                fight_boss::fight();
                thread::sleep(mid_menu_sleep);

                if loop_counter == 0 {
                    thread::sleep(mid_menu_sleep);
                    menu::navigate(Menu::Adventure);
                    thread::sleep(mid_menu_sleep);
                    adventure::go_to_latest();
                    thread::sleep(mid_menu_sleep);
                    adventure::turn_on_idle_mode();
                    // wait for a kill
                    thread::sleep(Duration::from_secs(5));
                    itopod::enter_itoped_at_optimal_floor();
                    thread::sleep(mid_menu_sleep);
                }
                loop_counter += 1;
            }

            feed_money_pit();

            menu::navigate(Menu::FightBoss);
            while start.elapsed().as_secs() < 179 {
                fight_boss::fight();
                thread::sleep(mid_menu_sleep);
            }
            fight_boss::stop();
            thread::sleep(mid_menu_sleep);

            menu::navigate(Menu::Rebirth);
            thread::sleep(mid_menu_sleep);
            click_at(rebirth_button);
            thread::sleep(mid_menu_sleep);
            click_at(confirm);
            thread::sleep(mid_menu_sleep);

            if run_id % 5 == 0 {
                menu::navigate(Menu::SpendEXP);
                thread::sleep(mid_menu_sleep);
                let current_exp = exp::get_unspent_exp().ok();
                let total_elapsed = script_start.elapsed().as_secs();
                println!("Total elapsed time: {}", total_elapsed);
                let time_per_run = total_elapsed as f64 / run_id as f64;
                println!("Time per run: {}", time_per_run);
                if let Some(v) = current_exp {
                    let total_exp = v - script_start_exp.unwrap();
                    println!("Total exp earned: {}", total_exp);
                    let exp_per_run = total_exp as f64 / run_id as f64;
                    println!("Exp per run: {}", exp_per_run);
                }
            }

            run_id += 1;
        }
    };

    ngu_script::run(script_routine);
}
