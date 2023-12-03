use std::ops::{Div, Mul, Rem};

/// Types that implement this trait can be used to calculate the greatest common denominator (GCD) and lowest common multiple
/// (LCM).
pub trait Gcd
where
    Self: Sized
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Rem<Self, Output = Self>
        + PartialEq<Self>
        + HasZero
        + Copy,
{
    fn gcd(self, other: Self) -> Self {
        if other == Self::ZERO {
            self
        } else {
            other.gcd(self % other)
        }
    }

    fn lcm(self, other: Self) -> Self {
        (self / self.gcd(other)) * other
    }
}

macro_rules! impl_gcd {
    ($($t:ty)*) => ($(
        impl Gcd for $t {}
    )*)
}

impl_gcd! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

/// Types that implement this trait indicates that they have a zero-value. The zero-value will be in `Self::ZERO`.
pub trait HasZero {
    const ZERO: Self;
}

macro_rules! impl_haszero {
    ($($t:ty)*) => ($(
        impl HasZero for $t {
            const ZERO: Self = 0;
        }
    )*)
}

impl_haszero! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
