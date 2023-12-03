use std::{
    cmp::max,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::number::HasZero;

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Debug)]
pub struct Pos2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos2d<T> {
    pub fn new(x: T, y: T) -> Pos2d<T> {
        Pos2d { x, y }
    }

    /// Adds `x` and `y` with this position's `x` and `y` and returns the result in a new [`Pos2d<T>`].
    pub fn add(&self, x: T, y: T) -> Pos2d<T>
    where
        T: Add<Output = T> + Copy,
    {
        Pos2d {
            x: self.x + x,
            y: self.y + y,
        }
    }

    /// Returns a new [`Pos2d<T>`] with the absolute values of `x` and `y`.
    pub fn abs(&self) -> Pos2d<T>
    where
        T: Neg<Output = T> + PartialOrd<T> + HasZero + Copy,
    {
        Pos2d::new(
            if self.x < T::ZERO { -self.x } else { self.x },
            if self.y < T::ZERO { -self.y } else { self.y },
        )
    }

    /// Returns the maximum of `x` and `y`.
    pub fn max(&self) -> T
    where
        T: Ord + Copy,
    {
        max(self.x, self.y)
    }

    /// Returns the sum of `x` and `y`.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` can't be converted into type `S`.
    pub fn sum<S>(&self) -> S
    where
        S: Add<S, Output = S> + TryFrom<T>,
        <S as TryFrom<T>>::Error: Debug,
        T: Copy,
    {
        <S as TryFrom<T>>::try_from(self.x).unwrap() + <S as TryFrom<T>>::try_from(self.y).unwrap()
    }
}

impl<T: Display> Display for Pos2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<T, Output = T>> Add<Pos2d<T>> for Pos2d<T> {
    type Output = Pos2d<T>;

    fn add(self, rhs: Pos2d<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<T, Output = T>> Sub<Pos2d<T>> for Pos2d<T> {
    type Output = Pos2d<T>;

    fn sub(self, rhs: Pos2d<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Pos2d<T> {
    type Output = Pos2d<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for Pos2d<T> {
    type Output = Pos2d<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Pos3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pos3d<T> {
    pub fn new(x: T, y: T, z: T) -> Pos3d<T> {
        Pos3d { x, y, z }
    }
}

impl<T: Display> Display for Pos3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Add<T, Output = T>> Add<Pos3d<T>> for Pos3d<T> {
    type Output = Self;

    fn add(self, rhs: Pos3d<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
