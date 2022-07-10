pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::InGamePosition;

    lazy_static! {
        pub static ref BASIC_TRAINING: InGamePosition = InGamePosition::from_coords(315, 30);
        pub static ref FIGHT_BOSS: InGamePosition = InGamePosition::from_coords(315, 70);
        pub static ref MONEY_PIT: InGamePosition = InGamePosition::from_coords(315, 110);
        pub static ref ADVENTURE: InGamePosition = InGamePosition::from_coords(315, 140);
        pub static ref INVENTORY: InGamePosition = InGamePosition::from_coords(315, 180);
        pub static ref AUGMENTATION: InGamePosition = InGamePosition::from_coords(315, 210);
    }
}
