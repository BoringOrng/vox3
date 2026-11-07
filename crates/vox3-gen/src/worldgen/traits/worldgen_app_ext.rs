use std::sync::Arc;

use bevy::prelude::*;

use crate::worldgen::{resources::WorldgenPasses, traits::gen_pass::GenPass};

pub trait WorldgenAppExt {
    fn worldgen_pass<P: GenPass>(&mut self, pass: P) -> &mut Self;
}

impl WorldgenAppExt for App {
    fn worldgen_pass<P: GenPass>(&mut self, pass: P) -> &mut Self {
        let mut world_generator = self.world_mut().resource_mut::<WorldgenPasses>();
        world_generator.passes.push(Arc::new(pass));

        self
    }
}
