pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref HARVEST_ALL_MAX_TIER: GameAwarePosition =
            GameAwarePosition::from_coords(1082, 597);
        pub static ref HARVEST_ALL_ANY_TIER: GameAwarePosition =
            GameAwarePosition::from_coords(1082, 653);
    }
}
