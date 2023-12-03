use std::cmp::Ordering;

pub trait TopIter {
    /// Collects a Vec of the highest valued elements in the iterator.
    ///
    /// You can decide the number of returned elements by specifying `num_elems`. The resulting Vec is sorted in descending
    /// order.
    fn top<T>(self, num_elems: usize) -> Vec<T>
    where
        Self: Sized + Iterator<Item = T>,
        T: Ord + Copy,
    {
        self.fold(vec![None as Option<T>; num_elems], |mut top, item| {
            match top.binary_search_by(|probe| match probe {
                None => Ordering::Less,
                Some(probe) => probe.cmp(&item),
            }) {
                Ok(0) | Err(0) => (),
                Ok(index) | Err(index) => {
                    top.copy_within(1..index, 0);
                    top[index - 1] = Some(item);
                }
            };
            top
        })
        .iter()
        .map(|item| item.unwrap())
        .collect()
    }
}

impl<I: Iterator> TopIter for I {}
