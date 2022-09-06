use crate::input::click_at;

pub fn add_energy() {
    click_at(*crate::constants::wandoos::coords::ADD_ENERGY)
}

pub fn add_magic() {
    click_at(*crate::constants::wandoos::coords::ADD_MAGIC)
}

pub fn cap_energy() {
    click_at(*crate::constants::wandoos::coords::CAP_ENERGY)
}

pub fn cap_magic() {
    click_at(*crate::constants::wandoos::coords::CAP_MAGIC)
}
