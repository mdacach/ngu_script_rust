pub mod inventory {
    use lazy_static::lazy_static;

    use crate::coords::InGamePosition;

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
