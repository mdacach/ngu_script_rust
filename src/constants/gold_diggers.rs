pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref PAGE1: GameAwarePosition = GameAwarePosition::from_coords(460, 146);
        pub static ref PAGE2: GameAwarePosition = GameAwarePosition::from_coords(550, 146);
        pub static ref PAGE3: GameAwarePosition = GameAwarePosition::from_coords(640, 146);
        pub static ref CLEAR_ACTIVE: GameAwarePosition = GameAwarePosition::from_coords(1214, 134);
        pub static ref CAP_DROP_CHANCE: GameAwarePosition =
            GameAwarePosition::from_coords(739, 252);
        pub static ref CAP_WANDOOS: GameAwarePosition = GameAwarePosition::from_coords(1160, 250);
        pub static ref CAP_STAT: GameAwarePosition = GameAwarePosition::from_coords(739, 506);
        pub static ref CAP_ADVENTURE: GameAwarePosition = GameAwarePosition::from_coords(1160, 506);
    }
}
