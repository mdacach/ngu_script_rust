pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref BASIC_TRAINING: GameAwarePosition = GameAwarePosition::from_coords(315, 30);
        pub static ref FIGHT_BOSS: GameAwarePosition = GameAwarePosition::from_coords(315, 70);
        pub static ref MONEY_PIT: GameAwarePosition = GameAwarePosition::from_coords(315, 110);
        pub static ref ADVENTURE: GameAwarePosition = GameAwarePosition::from_coords(315, 140);
        pub static ref INVENTORY: GameAwarePosition = GameAwarePosition::from_coords(315, 180);
        pub static ref AUGMENTATION: GameAwarePosition = GameAwarePosition::from_coords(315, 210);
        pub static ref ADV_TRAINING: GameAwarePosition = GameAwarePosition::from_coords(315, 250);
        pub static ref TIME_MACHINE: GameAwarePosition = GameAwarePosition::from_coords(315, 280);
        pub static ref BLOOD_MAGIC: GameAwarePosition = GameAwarePosition::from_coords(315, 320);
        pub static ref WANDOOS: GameAwarePosition = GameAwarePosition::from_coords(315, 360);
        pub static ref NGU: GameAwarePosition = GameAwarePosition::from_coords(315, 390);
        pub static ref YGGDRASIL: GameAwarePosition = GameAwarePosition::from_coords(315, 430);
        
        pub static ref GOLD_DIGGERS: GameAwarePosition = GameAwarePosition::from_coords(315, 465);
        pub static ref REBIRTH: GameAwarePosition = GameAwarePosition::from_coords(120, 564);
        pub static ref SPEND_EXP: GameAwarePosition = GameAwarePosition::from_coords(120, 605);
        pub static ref INPUT: GameAwarePosition = GameAwarePosition::from_coords(537, 27);
    }
}
