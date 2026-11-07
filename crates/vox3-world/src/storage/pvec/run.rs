use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::storage::palette;

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub struct Run {
    start: u16,
    palette: palette::Index,
}

impl Run {
    pub const fn new(start: u16, palette: palette::Index) -> Self {
        Self { start, palette }
    }

    #[must_use]
    pub const fn start(self) -> u16 {
        self.start
    }

    #[must_use]
    pub const fn palette(self) -> palette::Index {
        self.palette
    }
}

impl Default for Run {
    fn default() -> Self {
        Self {
            start: 0,
            palette: palette::Index::EMPTY,
        }
    }
}
