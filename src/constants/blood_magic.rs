pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref RIT_0_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 305);
        pub static ref RIT_1_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 348);
        pub static ref RIT_2_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 400);
        pub static ref RIT_3_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 445);
        pub static ref RIT_4_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 488);
        pub static ref RIT_5_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 536);
        pub static ref RIT_6_ADD: GameAwarePosition = GameAwarePosition::from_coords(664, 586);
    }
}
