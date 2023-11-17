use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

// TODO: another struct, tick buffer?
// current tick + RingBuffer<(usize, T)> ? can draw thread infer?
// only need current and prev drawn tick?
// only store most recent tick and allow storing none?
// NOTE: core decision: allow skipping ticks on client or not
// I think there needs to be skipping, in case ur internet poops out for like 4 secs
// and you need like 240 ticks sent -> bad
// how to handle skipping?
// prob simple slice search, [(time, tick)], accounting for missing ticks

// TODO: is that even nessessary?
// front = oldest, back = newest, think of it like normal Vec::push() order
/// Thin wrapper around a [`VecDeque`] that will try to never have any excess capacity.
#[derive(Clone, PartialEq, Default)]
pub struct RingBuffer<T>(pub VecDeque<T>);

impl<T: Debug> Debug for RingBuffer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

// previous n
// current
// "next" // just split off "next" into impl detail with threading
// update thread owns "next" and queues into buffer on draw thread?

// need someway to have update thread and draw thread go at once
// (update reads and rights to 1, gets update from server too)
// draw only reads (always "current" tick, never "next" tick)
// -> they never overlap but need to sync sending "next" tick to draw
// also also handle resizing
// TODO: hmmmm use: update generates time, draw consumes time?

impl<T> RingBuffer<T> {
    pub fn new() -> RingBuffer<T> {
        Self(VecDeque::new())
    }

    pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
        Self(VecDeque::with_capacity(capacity))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }
    pub fn get_back_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(self.0.len().wrapping_sub(1 + index))
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
    pub fn get_back(&self, index: usize) -> Option<&T> {
        self.0.get(self.0.len().wrapping_sub(1 + index))
    }
}

impl<T: Clone> RingBuffer<T> {

    pub fn fill_to_capacity(&mut self, value: &T) {
        for _ in 0..self.capacity() {
            self.push_back(value.clone())
        }
    }

    // pub fn fill(&mut self, to: usize, value: &T) {
    //     for elem in self.iter_mut() {
    //         *elem = value.clone();
    //     }
    // }

    /// Resize to new len. If `new_len` is larger, fill new space with the last element.
    /// If `new_len` is smaller, drop elements from the back.
    /// # Notes
    /// - Grows with [`VecDeque::reserve_exact()`], **O**(n) if repeatedly called
    /// - Allocates complety new [`VecDeque`] when shrinking.
    pub fn resize(&mut self, new_len: usize) {
        // if new_len is smaller, no-op
        // if let Some(additional) = new_len as isize - (self.len()) {
        // if let Some(additional) = new_len as isize - (self.len()) {
        let additional = new_len as isize - self.len() as isize;

        if additional >= 0 {
            // Growing
            let additional = additional as usize;
            self.reserve_exact(additional);
    
            let back = self.front().unwrap().clone();
            for _ in 0..additional {
                self.push_front(back.clone())
            }
        } else {
            // Shrinking
            // TODO: try std::mem::take to avoid cloning (is it optimized out?)
            let new_deque = self.iter().rev().take(new_len).rev().cloned().collect::<VecDeque<_>>();
            *self = Self(new_deque);
        };
    }
}

impl<T> AsRef<VecDeque<T>> for RingBuffer<T> {
    fn as_ref(&self) -> &VecDeque<T> {
        &self.0
    }
}
impl<T> AsMut<VecDeque<T>> for RingBuffer<T> {
    fn as_mut(&mut self) -> &mut VecDeque<T> {
        &mut self.0

    }
}

impl<T> Deref for RingBuffer<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for RingBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<VecDeque<T>> for RingBuffer<T> {
    fn from(value: VecDeque<T>) -> Self {
        RingBuffer(value)
    }
}
impl<T> From<RingBuffer<T>> for VecDeque<T> {
    fn from(value: RingBuffer<T>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer() {
        let mut ring = RingBuffer(VecDeque::with_capacity(8));
        for i in 0..8 {
            ring.push_back(i);
        }
        ring.resize(16);

        let mut truth = (0..8).collect::<Vec<_>>();
        truth.resize(16, *truth.last().unwrap());
        assert_eq!(truth, ring.make_contiguous())
    }
}