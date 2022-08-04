use crate::constants::yggdrasil::coords;
use crate::input::click_at;

pub fn harvest_all_max_tier() {
    click_at(*coords::HARVEST_ALL_MAX_TIER)
}

pub fn harvest_all_any_tier() {
    click_at(*coords::HARVEST_ALL_ANY_TIER);
}

#[cfg(test)]
mod tests {
    use crate::menu;

    use super::*;

    #[test]
    fn test_harvest() {
        menu::navigate(menu::Menu::Yggdrasil);
        harvest_all_max_tier();
    }
}
