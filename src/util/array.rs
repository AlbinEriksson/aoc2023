pub trait AsArray<T, const N: usize> {
    /// Converts `self` to a constant-length array.
    fn as_array(&self) -> [T; N];
}

impl<const N: usize> AsArray<char, N> for str {
    fn as_array(&self) -> [char; N] {
        let mut arr = ['\0'; N];
        for (item, char) in arr.iter_mut().zip(self.chars()) {
            *item = char;
        }
        arr
    }
}
