use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cardinal {
    Up,
    Down,
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub const ALL: [Self; 6] = [
        Self::Up,
        Self::Down,
        Self::North,
        Self::South,
        Self::East,
        Self::West,
    ];

    #[must_use]
    pub const fn to_ivec3(self) -> IVec3 {
        match self {
            Self::Up => IVec3::Y,
            Self::Down => IVec3::NEG_Y,
            Self::North => IVec3::NEG_Z,
            Self::South => IVec3::Z,
            Self::East => IVec3::X,
            Self::West => IVec3::NEG_X,
        }
    }

    #[must_use]
    pub fn to_vec3(self) -> Vec3 {
        self.to_ivec3().as_vec3()
    }
}
