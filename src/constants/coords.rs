pub mod inventory {
    use lazy_static::lazy_static;

    use crate::coords::{InGamePosition, Size};

    pub const SLOT_SIZE: Size = Size {
        width: 66,
        height: 67,
    };

    pub const SLOTS_PER_ROW: u16 = 12;

    lazy_static! {
        pub static ref SLOT_FIRST: InGamePosition = InGamePosition::from_coords(470, 440);
        pub static ref HELMET: InGamePosition = InGamePosition::from_coords(705, 88);
        pub static ref CHEST: InGamePosition = InGamePosition::from_coords(705, 157);
        pub static ref LEGS: InGamePosition = InGamePosition::from_coords(705, 224);
        pub static ref BOOTS: InGamePosition = InGamePosition::from_coords(705, 289);
        pub static ref WEAPON: InGamePosition = InGamePosition::from_coords(775, 157);
        pub static ref ACC1: InGamePosition = InGamePosition::from_coords(639, 88);
        pub static ref ACC2: InGamePosition = InGamePosition::from_coords(639, 157);
        pub static ref CUBE: InGamePosition = InGamePosition::from_coords(840, 157);
    }
}

pub mod adventure {
    use lazy_static::lazy_static;

    use crate::coords::InGamePosition;

    pub mod colors {
        use image::Rgb;

        pub const FIRST_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([124, 78, 78]);
        pub const SECOND_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([51, 68, 82]);
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
    }

    lazy_static! {
        pub static ref REGULAR_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(620, 128);
        pub static ref STRONG_ATTACK_PIXEL: InGamePosition = InGamePosition::from_coords(768, 128);
        pub static ref PARRY_PIXEL: InGamePosition = InGamePosition::from_coords(906, 128);
        pub static ref PIERCING_ATTACK_PIXEL: InGamePosition =
            InGamePosition::from_coords(1051, 128);
        pub static ref ULTIMATE_ATTACK_PIXEL: InGamePosition =
            InGamePosition::from_coords(1189, 128);
        pub static ref BLOCK_PIXEL: InGamePosition = InGamePosition::from_coords(485, 175);
        pub static ref DEFENSIVE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(631, 128);
        pub static ref HEAL_PIXEL: InGamePosition = InGamePosition::from_coords(766, 128);
        pub static ref OFFENSIVE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(910, 128);
        pub static ref CHARGE_PIXEL: InGamePosition = InGamePosition::from_coords(1050, 128);
        pub static ref ULTIMATE_BUFF_PIXEL: InGamePosition = InGamePosition::from_coords(1190, 128);
        pub static ref BOSS_CROWN_PIXEL: InGamePosition = InGamePosition::from_coords(986, 377);
        pub static ref RETREAT_ZONE_PIXEL: InGamePosition = InGamePosition::from_coords(976, 283);
        pub static ref ADVANCE_ZONE_PIXEL: InGamePosition = InGamePosition::from_coords(1257, 283);
    }
}

pub mod menu {
    use lazy_static::lazy_static;

    use crate::coords::InGamePosition;

    lazy_static! {
        pub static ref BASIC_TRAINING: InGamePosition = InGamePosition::from_coords(315, 30);
        pub static ref FIGHT_BOSS: InGamePosition = InGamePosition::from_coords(315, 70);
        pub static ref MONEY_PIT: InGamePosition = InGamePosition::from_coords(315, 110);
        pub static ref ADVENTURE: InGamePosition = InGamePosition::from_coords(315, 140);
        pub static ref INVENTORY: InGamePosition = InGamePosition::from_coords(315, 180);
        pub static ref AUGMENTATION: InGamePosition = InGamePosition::from_coords(315, 210);
    }
}
