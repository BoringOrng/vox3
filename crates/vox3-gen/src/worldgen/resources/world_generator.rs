use std::sync::Arc;

use bevy::prelude::*;

use crate::worldgen::traits::gen_pass::GenPass;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct WorldgenPasses {
    pub(in crate::worldgen) passes: Vec<Arc<dyn GenPass>>,
}
