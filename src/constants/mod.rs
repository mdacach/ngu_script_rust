use std::time::Duration;

pub mod adventure;
pub mod inventory;
pub mod menu;

pub const FAST_SLEEP: Duration = Duration::from_millis(20);
pub const LONG_SLEEP: Duration = Duration::from_millis(100);
