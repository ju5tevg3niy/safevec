use std::{
    cmp::Ordering,
    slice::{Iter, IterMut},
};

/// Generational index into SafeVec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenIdx {
    generation: u64,
    outer_idx: usize,
}

/// Vector with indexes that survive data push/remove in O(1) amortized
#[derive(Debug, PartialEq, Eq)]
pub struct SafeVec<T> {
    data: Vec<T>,
    gen: Vec<GenIdx>,
    outer2inner: Vec<usize>,
    first_unused: usize,
}

impl<T> SafeVec<T> {
    pub fn new() -> Self {
        const INITIAL_SIZE: usize = 256;
        Self::with_capacity(INITIAL_SIZE)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first_unused: 0,
            data: Vec::with_capacity(capacity),
            gen: Vec::with_capacity(capacity),
            outer2inner: Vec::with_capacity(capacity),
        }
    }

    /// Find unused GenData, push data there and return GenIdx to it, or create new if no unused
    ///
    /// Time: O(1) amortized (because Vec::push is)
    pub fn push(&mut self, data: T) -> GenIdx {
        let data_len = self.data.len();
        let first_unused = self.first_unused;
        match first_unused.cmp(&data_len) {
            Ordering::Greater => {
                unreachable!("first_unused should never point beyond one after last element")
            }
            Ordering::Equal => {
                // We are full, we use data_len as both inner & outer index
                self.data.push(data);
                self.gen.push(GenIdx {
                    generation: 0,
                    outer_idx: data_len,
                });
                // remember mapping between outer->inner indexes
                self.outer2inner.push(data_len);
            }
            Ordering::Less => {
                // We are not full, just use first unused
                self.data[first_unused] = data;
            }
        }
        // take unused gen_idx and mark it as used
        let gidx = self.gen[first_unused];
        self.first_unused += 1;
        gidx
    }

    /// Mark GenData for GenIdx as unused
    /// Return true if it was removed
    ///
    /// Time: O(1)
    pub fn remove(&mut self, gen_idx: GenIdx) -> bool {
        let inner_idx = self.outer2inner[gen_idx.outer_idx];
        let stored_gen_idx = &mut self.gen[inner_idx];
        if *stored_gen_idx != gen_idx {
            // double free or something else is wrong
            return false;
        }
        stored_gen_idx.generation += 1;
        let l = inner_idx;
        self.first_unused -= 1;
        let r = self.first_unused;
        self.outer2inner[self.gen[l].outer_idx] = r;
        self.outer2inner[self.gen[r].outer_idx] = l;
        self.data.swap(l, r);
        self.gen.swap(l, r);
        true
    }

    /// Get Rust's reference for GenIdx
    ///
    /// Time: O(1)
    pub fn get(&self, gen_idx: GenIdx) -> Option<&T> {
        let inner_idx = self.outer2inner[gen_idx.outer_idx];
        let stored_gen_idx = self.gen[inner_idx];
        if stored_gen_idx == gen_idx {
            Some(&self.data[inner_idx])
        } else {
            None
        }
    }

    /// Get Rust's mutable reference for GenIdx
    ///
    /// Time: O(1)
    pub fn get_mut(&mut self, gen_idx: GenIdx) -> Option<&mut T> {
        let inner_idx = self.outer2inner[gen_idx.outer_idx];
        let stored_gen_idx = self.gen[inner_idx];
        if stored_gen_idx == gen_idx {
            Some(&mut self.data[inner_idx])
        } else {
            None
        }
    }

    /// Get used data count
    pub fn len(&self) -> usize {
        self.first_unused
    }

    /// Get whether there is no used data
    pub fn is_empty(&self) -> bool {
        self.first_unused == 0
    }

    pub fn iter(&self) -> Iter<T> {
        self.data[..self.first_unused].iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data[..self.first_unused].iter_mut()
    }
}

impl<T> Default for SafeVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a SafeVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SafeVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
