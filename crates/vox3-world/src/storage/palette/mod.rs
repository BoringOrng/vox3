mod index;
mod slot;

use bevy::reflect::Reflect;
pub use index::Index;
use serde::{Deserialize, Serialize};
pub use slot::Slot;

#[expect(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Palette<T: Copy + Eq> {
    slots: Vec<Option<Slot<T>>>,
    free: Vec<u16>,
}

impl<T: Copy + Eq> Palette<T> {
    /// # Panics
    ///
    /// Panics if the number of slots is greater-than-or-equal-to what can be
    /// represented by an `u16`.
    pub fn intern(&mut self, item: T) -> Index {
        if let Some(i) = self
            .slots
            .iter()
            .position(|slot| slot.is_some_and(|s| s.item() == item))
        {
            // SAFETY: slot is occupied in this branch
            let slot = unsafe { self.slots[i].as_mut().unwrap_unchecked() };
            *slot.refs_mut() += 1;

            // SAFETY: `i` must be a valid `u16` because we panic otherwise.
            return Index::new(unsafe { u16::try_from(i).unwrap_unchecked() });
        }

        let slot = Slot::new(item);

        if let Some(index) = self.free.pop() {
            self.slots[index as usize] = Some(slot);
            return Index::new(index);
        }

        let index = u16::try_from(self.slots.len())
            .expect("There shouldn't more more slots than what can be represented by an `u16`");

        self.slots.push(Some(slot));
        Index::new(index)
    }

    /// # Panics
    ///
    /// Panics if `index` isn't occupied.
    pub fn add_ref(&mut self, index: Index) {
        if index.is_empty() {
            return;
        }

        *self.slots[usize::from(*index)]
            .as_mut()
            .expect("palette slot should be occupied")
            .refs_mut() += 1;
    }

    /// # Panics
    ///
    /// Panics if `index` isn't occupied.
    pub fn unref(&mut self, index: Index) {
        if index.is_empty() {
            return;
        }

        let slot = self.slots[usize::from(*index)]
            .as_mut()
            .expect("palette slot should be occupied");

        *slot.refs_mut() -= 1;

        if slot.refs() == 0 {
            self.slots[usize::from(*index)] = None;
            self.free.push(*index);
        }
    }

    /// # Panics
    ///
    /// Panics if `index` isn't occupied.
    #[must_use]
    pub fn resolve(&self, index: Index) -> Option<T> {
        if index.is_empty() {
            return None;
        }

        Some(
            self.slots[usize::from(*index)]
                .expect("palette slot should be occupied")
                .item(),
        )
    }
}

impl<T: Copy + Eq> Default for Palette<T> {
    fn default() -> Self {
        Self {
            slots: Vec::default(),
            free: Vec::default(),
        }
    }
}
