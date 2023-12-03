use std::ops::Index;

pub trait TryIndex
where
    Self: Index<usize> + InBounds,
{
    /// Tries to index using `idx`, returning `Some(...)` if `idx` is in bounds, or `None` otherwise.
    fn try_index(&self, idx: isize) -> Option<&Self::Output> {
        if self.in_bounds(idx) {
            Some(&self[idx.try_into().unwrap()])
        } else {
            None
        }
    }
}

impl<I: Index<usize> + InBounds> TryIndex for I {}

pub trait InBounds {
    /// Returns whether `idx` is in bounds, i.e. if `&foo[idx]` would return a valid reference.
    fn in_bounds(&self, idx: isize) -> bool;
}

impl<T> InBounds for Vec<T> {
    fn in_bounds(&self, idx: isize) -> bool {
        idx >= 0 && idx < self.len().try_into().unwrap()
    }
}
