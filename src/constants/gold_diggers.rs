pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref CAP_STAT: GameAwarePosition = GameAwarePosition::from_coords(739, 506);
        pub static ref CAP_WANDOOS: GameAwarePosition = GameAwarePosition::from_coords(1160, 250);
    }
}
