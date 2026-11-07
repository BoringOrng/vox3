use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct StreamingConfig {
    load_radius: u32,
}

impl StreamingConfig {
    #[must_use]
    pub const fn load_radius(self) -> u32 {
        self.load_radius
    }

    #[must_use]
    pub const fn unload_radius(self) -> u32 {
        self.load_radius + 1
    }
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self { load_radius: 8 }
    }
}
