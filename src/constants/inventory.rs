use crate::coords::Size;

pub const SLOT_SIZE: Size = Size {
    width: 66,
    height: 67,
};

pub const SLOTS_PER_ROW: u16 = 12;

pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::InGamePosition;

    lazy_static! {
        pub static ref SLOT_FIRST: InGamePosition = InGamePosition::from_coords(470, 440);
        pub static ref HELMET: InGamePosition = InGamePosition::from_coords(705, 88);
        pub static ref CHEST: InGamePosition = InGamePosition::from_coords(705, 157);
        pub static ref LEGS: InGamePosition = InGamePosition::from_coords(705, 224);
        pub static ref BOOTS: InGamePosition = InGamePosition::from_coords(705, 289);
        pub static ref WEAPON: InGamePosition = InGamePosition::from_coords(775, 157);
        pub static ref ACC1: InGamePosition = InGamePosition::from_coords(639, 88);
        pub static ref ACC2: InGamePosition = InGamePosition::from_coords(639, 157);
        pub static ref CUBE: InGamePosition = InGamePosition::from_coords(840, 157);
    }
}
