mod run;

use std::ops::Range;

use bevy::reflect::Reflect;
use run::Run;
use serde::{Deserialize, Serialize};

use crate::{
    chunk::components::Chunk,
    storage::palette::{self, Palette},
};

#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct PVec<T: Copy + Eq + Reflect> {
    runs: Vec<Run>,
    palette: Palette<T>,
    len: usize,
}

impl<T: Copy + Eq + Reflect> PVec<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn get(&self, idx: u16) -> Option<T> {
        let run = self.runs[self.run_index_for(idx)];
        self.palette.resolve(run.palette())
    }

    pub fn set(&mut self, idx: u16, item: T) -> Option<T> {
        self.set_raw(idx, Some(item))
    }

    pub fn remove(&mut self, idx: u16) -> Option<T> {
        self.set_raw(idx, None)
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, T)> {
        (0..self.runs.len()).flat_map(|idx| {
            let run = self.runs[idx];
            let end = self.run_end(idx);

            self.palette
                .resolve(run.palette())
                .into_iter()
                .flat_map(move |item| (run.start()..end).map(move |idx| (idx, item)))
        })
    }

    /// Sets every index in `range` to `item` in one pass.
    pub fn fill(&mut self, range: Range<u16>, item: T) {
        let Range { start, end } = range;
        if start >= end {
            return;
        }

        let added = usize::from(end - start) - self.count_present(start, end);
        let first = self.run_index_for(start);
        let last = self.run_index_for(end - 1);
        let end_bound = self.run_end(last);

        let left = (start > self.runs[first].start())
            .then(|| Run::new(self.runs[first].start(), self.runs[first].palette()));
        let right = (end < end_bound).then(|| Run::new(end, self.runs[last].palette()));

        // Intern before unreferencing so leftover runs cannot share a recycled
        // slot with the new run.
        let target = self.palette.intern(item);
        for run in [left, right].into_iter().flatten() {
            self.palette.add_ref(run.palette());
        }
        for run in &self.runs[first..=last] {
            self.palette.unref(run.palette());
        }

        let mut replacement = Vec::with_capacity(3);
        replacement.extend(left);
        replacement.push(Run::new(start, target));
        replacement.extend(right);

        self.runs.splice(first..=last, replacement);
        self.merge_around(start);
        self.len += added;
    }

    fn set_raw(&mut self, idx: u16, item: Option<T>) -> Option<T> {
        let run_idx = self.run_index_for(idx);
        let run = self.runs[run_idx];
        let old = self.palette.resolve(run.palette());

        if old == item {
            return old;
        }

        match (old, item) {
            (None, Some(_)) => self.len += 1,
            (Some(_), None) => self.len -= 1,
            _ => {}
        }

        let target = item.map_or(palette::Index::EMPTY, |item| self.palette.intern(item));

        let run_end = self.run_end(run_idx);
        let left = idx > run.start();
        let right = idx + 1 < run_end;

        match (left, right) {
            (false, false) => self.palette.unref(run.palette()),
            (true, true) => self.palette.add_ref(run.palette()),
            _ => {}
        }

        let mut replacement = Vec::with_capacity(3);

        if left {
            replacement.push(Run::new(run.start(), run.palette()));
        }

        replacement.push(Run::new(idx, target));

        if right {
            replacement.push(Run::new(idx + 1, run.palette()));
        }

        self.runs.splice(run_idx..=run_idx, replacement);
        self.merge_around(idx);

        old
    }

    fn count_present(&self, start: u16, end: u16) -> usize {
        let first = self.run_index_for(start);
        let last = self.run_index_for(end - 1);

        (first..=last)
            .filter_map(|idx| {
                let run = self.runs[idx];
                let run_start = run.start().max(start);
                let run_end = self.run_end(idx).min(end);
                self.palette
                    .resolve(run.palette())
                    .map(|_| usize::from(run_end - run_start))
            })
            .sum()
    }

    fn run_index_for(&self, idx: u16) -> usize {
        self.runs.partition_point(|r| r.start() <= idx) - 1
    }

    fn run_end(&self, idx: usize) -> u16 {
        self.runs.get(idx + 1).map_or(Chunk::VOLUME, |r| r.start())
    }

    fn merge_around(&mut self, idx: u16) {
        let idx = self.run_index_for(idx);
        self.try_merge_with_next(idx);

        if idx > 0 {
            self.try_merge_with_next(idx - 1);
        }
    }

    fn try_merge_with_next(&mut self, idx: usize) {
        let Some(&next) = self.runs.get(idx + 1) else {
            return;
        };

        if next.palette() == self.runs[idx].palette() {
            self.palette.unref(next.palette());
            self.runs.remove(idx + 1);
        }
    }
}

impl<T: Copy + Eq + Reflect> Default for PVec<T> {
    fn default() -> Self {
        Self {
            runs: vec![Run::default()],
            palette: Palette::default(),
            len: 0,
        }
    }
}
