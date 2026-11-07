use std::sync::Arc;

use bevy::prelude::*;

use crate::worldgen::{resources::WorldgenPasses, traits::GenPass};

pub trait WorldgenAppExt {
    fn worldgen_pass<P: GenPass>(&mut self, pass: P);
}

impl WorldgenAppExt for App {
    fn worldgen_pass<P: GenPass>(&mut self, pass: P) {
        let mut world_generator = self.world_mut().resource_mut::<WorldgenPasses>();
        world_generator.passes.push(Arc::new(pass));
    }
}
