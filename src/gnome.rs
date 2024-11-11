use std::sync::atomic::{AtomicBool, AtomicU8};

/// Dummy data

#[derive(Default)]
pub struct Gnome {
    pub age: u8,
    pub grow: bool,
}

pub struct AtomicGnome {
    pub age: AtomicU8,
    pub grow: AtomicBool,
}

impl Default for AtomicGnome {
    fn default() -> Self {
        AtomicGnome {
            age: AtomicU8::new(0),
            grow: AtomicBool::new(false),
        }
    }
}
