use std::thread;
use std::time::{Duration, Instant};

use image::Rgb;

use ngu_script::constants::adventure::colors::NO_ENEMY_RGB;
use ngu_script::constants::gold_diggers::coords::{
    CAP_ADVENTURE, CAP_DROP_CHANCE, CAP_STAT, CAP_WANDOOS, PAGE1, PAGE2, PAGE3,
};
use ngu_script::constants::user::LONG_SLEEP;
use ngu_script::adventure::AdventureZone;
use ngu_script::coords::GameAwarePosition;
use ngu_script::input::{click_at, right_click_at};
use ngu_script::menu::Menu;
use ngu_script::*;
use ngu_script::{itopod, menu};

fn main() {
    let script_routine = || {
        // TODO: move these coordinates to lib
        let basic_training = GameAwarePosition::from_coords(1122, 186);
        let energy_idle_half = GameAwarePosition::from_coords(1156, 26);
        let energy_idle_quarter = GameAwarePosition::from_coords(1200, 26);
        let magic_idle_half = GameAwarePosition::from_coords(1156, 60);
        let magic_idle_quarter = GameAwarePosition::from_coords(1200, 60);
        let rebirth_button = GameAwarePosition::from_coords(734, 700);
        let confirm = GameAwarePosition::from_coords(578, 430);
        let mid_menu_sleep = Duration::from_millis(450);
        let loadout1 = GameAwarePosition::from_coords(445, 345);
        let loadout2 = GameAwarePosition::from_coords(485, 345);
        let loadout3 = GameAwarePosition::from_coords(525, 345);

        // NGU coordinates
        let ngu_augments_cap = GameAwarePosition::from_coords(790, 320);
        let ngu_wandoos_cap = GameAwarePosition::from_coords(790, 370);
        let ngu_respawn_cap = GameAwarePosition::from_coords(790, 415);
        let ngu_gold_cap = GameAwarePosition::from_coords(790, 460);
        let ngu_adventure_alpha_cap = GameAwarePosition::from_coords(790, 510);
        let ngu_power_alpha_cap = GameAwarePosition::from_coords(790, 555);
        let ngu_drop_chance_cap = GameAwarePosition::from_coords(790, 600);
        let ngu_magic_ngu_cap = GameAwarePosition::from_coords(790, 650);
        let ngu_pp_cap = GameAwarePosition::from_coords(790, 695);

        let ngu_augments_add = GameAwarePosition::from_coords(690, 320);
        let ngu_wandoos_add = GameAwarePosition::from_coords(690, 370);
        let ngu_respawn_add = GameAwarePosition::from_coords(690, 415);
        let ngu_gold_add = GameAwarePosition::from_coords(690, 460);
        let ngu_adventure_alpha_add = GameAwarePosition::from_coords(690, 510);
        let ngu_power_alpha_add = GameAwarePosition::from_coords(690, 555);
        let ngu_drop_chance_add = GameAwarePosition::from_coords(690, 600);
        let ngu_magic_ngu_add = GameAwarePosition::from_coords(690, 650);
        let ngu_pp_add = GameAwarePosition::from_coords(690, 695);

        let leftover_to_ngu = || {
            menu::navigate(Menu::NGU);
            /*
            click_at(ngu_magic_ngu_cap);
            click_at(*PAGE1);
            click_at(ngu_power_alpha_cap);
            */

            /*
            click_at(energy_idle_half);
            click_at(ngu_adventure_alpha_add);
            click_at(ngu_drop_chance_add);
            click_at(*PAGE1);
            click_at(ngu_drop_chance_cap);
            */


            click_at(energy_idle_quarter);
            click_at(ngu_augments_add);
            click_at(ngu_wandoos_add);
            click_at(ngu_gold_add);
            click_at(ngu_power_alpha_add);
            click_at(*PAGE1);
            click_at(ngu_drop_chance_cap);
        };

        let white: Rgb<u8> = Rgb([255, 255, 255]);
        let time_machine_energy_cap_pixel = GameAwarePosition::from_coords(678, 315);
        let time_machine_magic_cap_pixel = GameAwarePosition::from_coords(678, 445);

        let tm_energy_capped = || {
            let color = pixel::get_pixel_rgb(time_machine_energy_cap_pixel);
            !pixel::approximately_equal(color, white)
        };
        let tm_magic_capped = || {
            let color = pixel::get_pixel_rgb(time_machine_magic_cap_pixel);
            !pixel::approximately_equal(color, white)
        };

        let cap_energy_tm = || {
            if tm_energy_capped() {
                return;
            }

            for _ in 0..10 {
                time_machine::add_energy();
                thread::sleep(ngu_script::constants::user::MEDIUM_SLEEP);
                if tm_energy_capped() {
                    break;
                }
            }
            time_machine::add_energy();
            time_machine::add_energy();
            time_machine::add_energy();
            time_machine::add_energy();
        };

        let cap_magic_tm = || {
            if tm_magic_capped() {
                return;
            }

            for _ in 0..3 {
                time_machine::add_magic();
                thread::sleep(ngu_script::constants::user::MEDIUM_SLEEP);
                if tm_magic_capped() {
                    break;
                }
            }
                time_machine::add_magic();
                time_machine::add_magic();
        };

        let cap_augment = |number| {
            if augments::is_capped_augment(number) {
                return;
            }

            for _ in 0..10 {
                augments::add_augment(number);
                augments::add_enhancement(number);
                thread::sleep(ngu_script::constants::user::MEDIUM_SLEEP);
                if augments::is_capped_augment(number) {
                    break;
                }
            }
            augments::add_augment(number);
            augments::add_augment(number);
            augments::add_augment(number);
                augments::add_enhancement(number);
        };

        let activate_diggers = || {
            menu::navigate(Menu::GoldDiggers);
            click_at(*PAGE3);
            click_at(*CAP_ADVENTURE);
            click_at(*CAP_DROP_CHANCE);


            click_at(*PAGE2);
            click_at(*CAP_DROP_CHANCE);
            click_at(*CAP_WANDOOS);

            click_at(*PAGE1);
            click_at(*CAP_STAT);
        };

        let pit_feed = GameAwarePosition::from_coords(784, 324);
        let pit_confirm = GameAwarePosition::from_coords(580, 422);
        let feed_money_pit = || {
            menu::navigate(Menu::MoneyPit);
            //thread::sleep(mid_menu_sleep);
            click_at(pit_feed);
            //thread::sleep(mid_menu_sleep);
            click_at(pit_confirm);
            //thread::sleep(mid_menu_sleep);
        };

        let blood_magic = 6;
        let augment = 4;

        let script_start = Instant::now();
        let mut run_id = 1;

        let get_exp = || {
            menu::navigate(Menu::SpendEXP);
            //thread::sleep(mid_menu_sleep);
            exp::get_unspent_exp().ok()
        };

        menu::navigate(Menu::SpendEXP); // Focus the game
        let script_start_exp = get_exp();
        println!("Starting script exp: {:#?}", script_start_exp);

        loop {
            let start = Instant::now();

            menu::navigate(Menu::Inventory);
            click_at(loadout2);

            menu::navigate(Menu::BasicTraining);
            right_click_at(basic_training);

            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            // Wait for nuke to kill bosses
            thread::sleep(Duration::from_secs(4));
            for _ in 0..5 {
                fight_boss::fight();
                thread::sleep(Duration::from_secs(1));
            }

            menu::navigate(Menu::Adventure);
            adventure::go_to_zone(AdventureZone::BAE);
            adventure::turn_on_idle_mode();
            // wait for a kill
            thread::sleep(Duration::from_secs(8));
            itopod::enter_itoped_at_optimal_floor();

            let setup = || {
                menu::navigate(Menu::TimeMachine);
                let increment = 500_000_000;
                input::input_number(increment);
                cap_energy_tm();
                cap_magic_tm();

                menu::navigate(Menu::Wandoos);
                wandoos::cap_energy();
                wandoos::cap_magic();

                menu::navigate(Menu::Augmentation);
                cap_augment(augment);

                menu::navigate(Menu::BloodMagic);
                blood_magic::cap_ritual(blood_magic);

                activate_diggers();

                leftover_to_ngu();
            };

            menu::navigate(Menu::Inventory);
            click_at(loadout1);

            menu::navigate(Menu::BasicTraining);
            right_click_at(basic_training);

            // Here we want to do something else
            while start.elapsed().as_secs() < 155 {
                setup();
            }

            feed_money_pit();

            menu::navigate(Menu::FightBoss);
            fight_boss::nuke();
            while start.elapsed().as_secs() < 179 {
                fight_boss::fight();
            }
            fight_boss::stop();

            menu::navigate(Menu::Rebirth);
            click_at(rebirth_button);
            click_at(confirm);

            if run_id % 5 == 0 {
                menu::navigate(Menu::SpendEXP);
                thread::sleep(LONG_SLEEP);
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
