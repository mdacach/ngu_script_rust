pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref ADD_ENERGY: GameAwarePosition = GameAwarePosition::from_coords(714, 315);
        pub static ref ADD_MAGIC: GameAwarePosition = GameAwarePosition::from_coords(714, 445);
        pub static ref REMOVE_ENERGY: GameAwarePosition = GameAwarePosition::from_coords(760, 315);
        pub static ref REMOVE_MAGIC: GameAwarePosition = GameAwarePosition::from_coords(760, 445);
    }
}
