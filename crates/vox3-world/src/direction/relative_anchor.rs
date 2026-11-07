use crate::direction::{Cardinal, Relative};

#[derive(Clone, Copy)]
pub struct Anchor {
    facing: u8,
}

impl Anchor {
    const ROTATING_CARDINALS: [Cardinal; 4] = [
        Cardinal::North,
        Cardinal::East,
        Cardinal::South,
        Cardinal::West,
    ];

    #[must_use]
    pub fn new(facing: Cardinal) -> Self {
        debug_assert!(!matches!(facing, Cardinal::Up | Cardinal::Down));

        Self {
            facing: match facing {
                Cardinal::North => 0,
                Cardinal::East => 1,
                Cardinal::South => 2,
                Cardinal::West => 3,
                _ => unreachable!(),
            },
        }
    }

    #[must_use]
    pub fn facing(self) -> Cardinal {
        Self::ROTATING_CARDINALS[usize::from(self.facing)]
    }

    #[must_use]
    pub fn cardinal_to(self, rel: Relative) -> Cardinal {
        debug_assert!(!matches!(rel, Relative::Up | Relative::Down));

        let rot = match rel {
            Relative::Front => 0,
            Relative::Right => 1,
            Relative::Back => 2,
            Relative::Left => 3,
            _ => unreachable!(),
        };

        let idx = (self.facing + rot) % 4;
        Self::ROTATING_CARDINALS[usize::from(idx)]
    }
}
