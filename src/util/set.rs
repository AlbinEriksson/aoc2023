pub trait Set<Item> {
    /// Adds `item`. Nothing happens if `item` already exists.
    fn add(&mut self, item: Item);
    /// Removes `item`. Nothing happens if `item` does not exist.
    fn remove(&mut self, item: Item);
    /// Clears this set from all items.
    fn clear(&mut self);
    /// Returns whether `item` exists.
    fn contains(&self, item: Item) -> bool;
    /// Removes all items that do not exist in `other`.
    /// 
    /// # Panics
    /// 
    /// Panics if `other` has an incompatible configuration to `self`.
    fn intersect(&mut self, other: &Self);
}
