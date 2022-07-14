use crate::coords::Size;

pub const SLOT_SIZE: Size = Size {
    width: 66,
    height: 67,
};

pub const SLOTS_PER_ROW: u16 = 12;

/// Inventory slots are earned throughout the game.
pub const SLOTS_AVAILABLE: u16 = 53;

pub mod colors {
    use image::Rgb;

    pub const EMPTY_SLOT_RGB: Rgb<u8> = Rgb([130, 130, 130]);
}

pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref SLOT_FIRST: GameAwarePosition = GameAwarePosition::from_coords(470, 440);
        pub static ref HELMET: GameAwarePosition = GameAwarePosition::from_coords(705, 88);
        pub static ref CHEST: GameAwarePosition = GameAwarePosition::from_coords(705, 157);
        pub static ref LEGS: GameAwarePosition = GameAwarePosition::from_coords(705, 224);
        pub static ref BOOTS: GameAwarePosition = GameAwarePosition::from_coords(705, 289);
        pub static ref WEAPON: GameAwarePosition = GameAwarePosition::from_coords(775, 157);
        pub static ref ACC1: GameAwarePosition = GameAwarePosition::from_coords(639, 88);
        pub static ref ACC2: GameAwarePosition = GameAwarePosition::from_coords(639, 157);
        pub static ref CUBE: GameAwarePosition = GameAwarePosition::from_coords(840, 157);
    }
}
