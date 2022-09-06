pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref ADD_ENERGY: GameAwarePosition = GameAwarePosition::from_coords(739, 334);
        pub static ref CAP_ENERGY: GameAwarePosition = GameAwarePosition::from_coords(833, 334);
        pub static ref ADD_MAGIC: GameAwarePosition = GameAwarePosition::from_coords(739, 465);
        pub static ref CAP_MAGIC: GameAwarePosition = GameAwarePosition::from_coords(833, 465);
    }
}