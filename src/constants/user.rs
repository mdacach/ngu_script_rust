use std::time::Duration;

// For less powerful PCs more sleep may be needed.
pub const FAST_SLEEP: Duration = Duration::from_millis(20);
pub const LONG_SLEEP: Duration = Duration::from_millis(100);

/// Inventory slots are earned throughout the game.
pub const SLOTS_AVAILABLE: u16 = 60;

pub enum Monitor {
    Primary,
    Secondary,
}

pub const MONITOR_USED: Monitor = Monitor::Secondary;

pub const ATTACK_COOLDOWN: Duration = Duration::from_millis(800);
