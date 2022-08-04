pub mod adventure;
pub mod constants;
pub mod coords;
pub mod input;
pub mod inventory;
pub mod itopod;
pub mod menu;
pub mod ocr;
pub mod pixel;
pub mod yggdrasil;

pub fn run<F>(script_routine: F)
where
    F: Fn() + Send + Sync + 'static,
{
    std::thread::spawn(move || {
        script_routine();
    });

    input::handle_user_termination();
}
