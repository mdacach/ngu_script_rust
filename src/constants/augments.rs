pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref AUG_0_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 352);
        pub static ref ENH_0_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 390);
        
        pub static ref AUG_1_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 440);
        pub static ref ENH_1_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 475);
        
        pub static ref AUG_2_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 525);
        pub static ref ENH_2_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 560);
        
        pub static ref AUG_3_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 610);
        pub static ref ENH_3_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 650);
        
        pub static ref AUG_4_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 700);
        pub static ref ENH_4_ADD: GameAwarePosition = GameAwarePosition::from_coords(717, 735);
    }
}
