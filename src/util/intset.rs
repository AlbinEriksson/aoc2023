use super::set::Set;

/// Contains a set of `isize` within the range `min..max`.
///
/// Implemented as a bit field of size `max - min`.
pub struct IntSet {
    min: isize,
    max: isize,
    items: Vec<usize>,
}

impl IntSet {
    pub fn new(min: isize, max: isize) -> IntSet {
        let num_items: usize = (max - min).try_into().unwrap();
        let num_items = (num_items + usize::BITS as usize - 1) / usize::BITS as usize;
        IntSet {
            min,
            max,
            items: vec![0; num_items],
        }
    }

    /// Returns a tuple of the position a number would have in this [`IntSet`].
    ///
    /// The first tuple value is the index of a bit field in the `items` Vec, whereas the second value is the bit position
    /// within that bit field.
    fn bit_position(&self, item: isize) -> (usize, usize) {
        let offset: usize = (item - self.min).try_into().unwrap();
        (offset / usize::BITS as usize, offset % usize::BITS as usize)
    }

    /// Returns the minimum number that can exist in this [`IntSet`].
    pub fn get_min(&self) -> isize {
        self.min
    }

    /// Returns the maximum number that can exist in this [`IntSet`].
    pub fn get_max(&self) -> isize {
        self.max
    }

    /// Returns the amount of numbers in this [`IntSet`].
    pub fn count(&self) -> usize {
        self.items.iter().map(|item| item.count_ones() as usize).sum()
    }
}

impl Set<isize> for IntSet {
    fn add(&mut self, item: isize) {
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index] |= 1 << bit_offset;
    }

    fn remove(&mut self, item: isize) {
        if item < self.min || item >= self.max {
            return;
        }
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index] &= !(1 << bit_offset);
    }

    fn clear(&mut self) {
        self.items.fill(0);
    }

    fn contains(&self, item: isize) -> bool {
        if item < self.min || item >= self.max {
            return false;
        }
        let (item_index, bit_offset) = self.bit_position(item);
        (self.items[item_index] & (1 << bit_offset)) != 0
    }

    fn intersect(&mut self, other: &Self) {
        if self.min != other.min || self.max != other.max {
            panic!("Other IntSet must have the same min/max ranges");
        }
        for (item, other) in self.items.iter_mut().zip(other.items.iter()) {
            *item &= other;
        }
    }
}
