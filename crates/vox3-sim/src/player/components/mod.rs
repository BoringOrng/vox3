use bevy::prelude::*;

use crate::entity::components::SimEntity;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[require(SimEntity)]
pub struct Player;
