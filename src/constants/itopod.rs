pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref ENTER_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(496, 302);
        pub static ref OPTIMAL_FLOOR_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(940, 275);
        pub static ref MAX_FLOOR_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(970, 325);
        pub static ref ENTER_CONFIRMATION_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(833, 400);
        pub static ref START_FLOOR_INPUT_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(833, 261);
        pub static ref END_FLOOR_INPUT_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(860, 314);
        pub static ref CLOSE_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(771, 596);
        pub static ref GET_TOOLTIP_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(663, 29);
    }
}

pub mod areas {
    use lazy_static::lazy_static;

    use crate::coords::GameAwareRectangle;

    lazy_static! {
        pub static ref ITOPOD_TOOLTIP_OCR_AREA: GameAwareRectangle =
            GameAwareRectangle::from_coords(699, 56, 352, 155);
    }
}
