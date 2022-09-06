pub mod adventure;
pub mod augments;
pub mod blood_magic;
pub mod constants;
pub mod coords;
pub mod exp;
pub mod input;
pub mod inventory;
pub mod itopod;
pub mod menu;
pub mod ocr;
pub mod pixel;
pub mod time_machine;
pub mod yggdrasil;
pub mod fight_boss;

pub fn run<F>(script_routine: F)
where
    F: Fn() + Send + Sync + 'static,
{
    std::thread::spawn(move || {
        script_routine();
    });

    input::handle_user_termination();
}
