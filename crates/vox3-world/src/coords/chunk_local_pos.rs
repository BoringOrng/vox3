use bevy::{math::U8Vec3, prelude::*};

use crate::chunk::components::Chunk;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkLocalPos(u16);

impl ChunkLocalPos {
    const SHIFT: u32 = (Chunk::SIZE - 1).bit_width();
    const MASK: u8 = (1u8 << Self::SHIFT) - 1;

    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(0x7FFF);

    pub fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!((0..Chunk::SIZE).contains(&x), "x-side length too long: {x}");
        debug_assert!((0..Chunk::SIZE).contains(&y), "y-side length too long: {y}");
        debug_assert!((0..Chunk::SIZE).contains(&z), "z-side length too long: {z}");

        let [x, y, z] = [x, y, z].map(Self::mask);
        Self(z << (2 * Self::SHIFT) | y << Self::SHIFT | x)
    }

    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn x(self) -> u8 {
        (self.0 & u16::from(Self::MASK)) as u8
    }

    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn y(self) -> u8 {
        ((self.0 >> Self::SHIFT) & u16::from(Self::MASK)) as u8
    }

    #[must_use]
    pub fn z(self) -> u8 {
        ((self.0 >> (2 * Self::SHIFT)) & u16::from(Self::MASK)) as u8
    }

    #[must_use]
    pub fn as_ivec3(self) -> IVec3 {
        U8Vec3::new(self.x(), self.y(), self.z()).as_ivec3()
    }

    pub(crate) const fn index(self) -> u16 {
        self.0
    }

    pub(crate) const fn from_index(idx: u16) -> Self {
        Self(idx)
    }

    fn mask(axis: u8) -> u16 {
        u16::from(axis & Self::MASK)
    }
}

impl From<&GlobalTransform> for ChunkLocalPos {
    fn from(value: &GlobalTransform) -> Self {
        let [x, y, z] = value
            .translation()
            .rem_euclid(Vec3::splat(Chunk::SIZE.into()))
            .as_u8vec3()
            .to_array();

        Self::new(x, y, z)
    }
}
