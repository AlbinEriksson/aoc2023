pub struct Interleave<I, S> {
    iter: I,
    separator: S,
    spacing: usize,
    next_sep: usize,
}

impl<I, S> Interleave<I, S> {
    pub fn new(iter: I, sep: S, spacing: usize) -> Interleave<I, S> {
        Interleave {
            iter,
            separator: sep,
            spacing,
            next_sep: spacing + 1,
        }
    }
}

impl<I: Iterator<Item = S>, S: Copy> Iterator for Interleave<I, S> {
    type Item = S;

    fn next(&mut self) -> Option<S> {
        self.next_sep -= 1;
        if self.next_sep > 0 {
            self.iter.next()
        } else {
            self.next_sep = self.spacing + 1;
            Some(self.separator)
        }
    }
}

pub trait InterleaveIter {
    /// Interleaves an iterator with a separator `sep` such that after `spacing` items, the `sep` will the next item.
    fn interleave<S: Copy>(self, sep: S, spacing: usize) -> Interleave<Self, S>
    where
        Self: Sized,
    {
        Interleave::new(self, sep, spacing)
    }
}

impl<I: Iterator> InterleaveIter for I {}
