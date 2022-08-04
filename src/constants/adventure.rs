pub mod colors {
    use image::Rgb;

    pub const FIRST_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([124, 78, 78]);
    pub const SECOND_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([51, 68, 82]);
    pub const THIRD_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([98, 74, 74]);
    pub const BOSS_CROWN_RGB: Rgb<u8> = Rgb([247, 239, 41]);
}

pub mod keys {
    use rdev::Key;

    pub const REGULAR_ATTACK: Key = Key::KeyW;
    pub const STRONG_ATTACK: Key = Key::KeyE;
    pub const PARRY: Key = Key::KeyR;
    pub const PIERCING_ATTACK: Key = Key::KeyT;
    pub const ULTIMATE_ATTACK: Key = Key::KeyY;

    pub const BLOCK: Key = Key::KeyA;
    pub const DEFENSIVE_BUFF: Key = Key::KeyS;
    pub const HEAL: Key = Key::KeyD;
    pub const OFFENSIVE_BUFF: Key = Key::KeyF;
    pub const CHARGE: Key = Key::KeyG;
    pub const ULTIMATE_BUFF: Key = Key::KeyH;

    pub const HYPER_REGEN: Key = Key::KeyX;
}

pub mod coords {
    use lazy_static::lazy_static;

    use crate::coords::GameAwarePosition;

    lazy_static! {
        pub static ref REGULAR_ATTACK_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(620, 128);
        pub static ref STRONG_ATTACK_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(768, 128);
        pub static ref PARRY_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(906, 128);
        pub static ref PIERCING_ATTACK_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(1051, 128);
        pub static ref ULTIMATE_ATTACK_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(1189, 128);
        pub static ref BLOCK_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(485, 175);
        pub static ref DEFENSIVE_BUFF_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(631, 175);
        pub static ref HEAL_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(766, 175);
        pub static ref OFFENSIVE_BUFF_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(910, 175);
        pub static ref CHARGE_PIXEL: GameAwarePosition = GameAwarePosition::from_coords(1050, 175);
        pub static ref ULTIMATE_BUFF_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(1190, 175);
        pub static ref HYPER_REGEN_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(627, 225);
        pub static ref BOSS_CROWN_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(986, 377);
        pub static ref RETREAT_ZONE_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(976, 283);
        pub static ref ADVANCE_ZONE_PIXEL: GameAwarePosition =
            GameAwarePosition::from_coords(1257, 283);
    }
}
