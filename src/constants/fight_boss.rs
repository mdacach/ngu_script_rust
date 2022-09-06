pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref NUKE: GameAwarePosition = GameAwarePosition::from_coords(833, 206);
        pub static ref FIGHT: GameAwarePosition = GameAwarePosition::from_coords(833, 354);
    }
}
