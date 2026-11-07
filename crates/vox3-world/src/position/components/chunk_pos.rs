use bevy::prelude::*;

use crate::chunk::components::Chunk;

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Component,
    Reflect,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Mul,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
)]
#[require(Transform)]
pub struct ChunkPos(IVec3);

impl ChunkPos {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    #[must_use]
    pub const fn from_ivec3(v: IVec3) -> Self {
        Self(v)
    }

    #[must_use]
    pub const fn x(self) -> i32 {
        self.0.x
    }

    #[must_use]
    pub const fn y(self) -> i32 {
        self.0.y
    }

    #[must_use]
    pub const fn z(self) -> i32 {
        self.0.z
    }

    #[must_use]
    pub fn distance_squared(self, other: Self) -> u32 {
        self.0.distance_squared(other.0).cast_unsigned()
    }

    #[must_use]
    pub fn as_vec3(self) -> Vec3 {
        self.0.as_vec3()
    }

    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 {
        self.0
    }
}

impl From<&GlobalTransform> for ChunkPos {
    fn from(value: &GlobalTransform) -> Self {
        Self::from_ivec3(
            value
                .translation()
                .div_euclid(Vec3::splat(Chunk::SIZE.into()))
                .as_ivec3(),
        )
    }
}
