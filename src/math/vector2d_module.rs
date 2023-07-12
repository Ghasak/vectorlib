pub use core::fmt;
#[allow(unused_imports)]
use num_traits::{Float, FromPrimitive, Num, Zero};
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2d<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign
        + Copy
        + std::fmt::Display,
{
    pub x: T,
    pub y: T,
}

// ------------------------------------------------------------------

impl<T> AddAssign<Vector2d<T>> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Mul<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>,
{
    fn add_assign(&mut self, other: Vector2d<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> AddAssign<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Mul<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
    }
}

// --------------------------------------------------------------

impl<T> Add<Vector2d<T>> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Mul<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn add(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Add<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn add(self, scalar: T) -> Vector2d<T> {
        Vector2d {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl<T> Sub<Vector2d<T>> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn sub(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Sub<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn sub(self, scalar: T) -> Vector2d<T> {
        Vector2d {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
}

impl<T> Mul<Vector2d<T>> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn mul(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Mul<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn mul(self, scalar: T) -> Vector2d<T> {
        Vector2d {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> Div<Vector2d<T>> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn div(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Div<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn div(self, scalar: T) -> Vector2d<T> {
        Vector2d {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T> Vector2d<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + AddAssign
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Copy
        + std::fmt::Display,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // pub fn add<U>(&self, other: U) -> Vector2d<T>
    // where
    //     T: Add<U, Output = T>,
    //     U: Copy,
    // {
    //     Vector2d {
    //         x: self.x + other,
    //         y: self.y + other,
    //     }
    // }
    //
    // pub fn sub<U>(&self, other: U) -> Vector2d<T>
    // where
    //     T: Sub<U, Output = T>,
    //     U: Copy,
    // {
    //     Vector2d {
    //         x: self.x - other,
    //         y: self.y - other,
    //     }
    // }
    //
    // pub fn mul<U>(&self, other: U) -> Vector2d<T>
    // where
    //     T: Mul<U, Output = T>,
    //     U: Copy,
    // {
    //     Vector2d {
    //         x: self.x * other,
    //         y: self.y * other,
    //     }
    // }
    //
    // pub fn div<U>(&self, other: U) -> Vector2d<T>
    // where
    //     T: Div<U, Output = T>,
    //     U: Copy,
    // {
    //     Vector2d {
    //         x: self.x / other,
    //         y: self.y / other,
    //     }
    // }
}

// -----------------------------------------
//  Index access for both mut and imutable
// -----------------------------------------

impl<T> Index<usize> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign
        + Copy
        + std::fmt::Display,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            i => panic!("Index {} out of [0, 1] range", i),
        }
    }
}

impl<T> IndexMut<usize> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign
        + Copy
        + std::fmt::Display,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            i => panic!("Index {} out of [0, 1] range", i),
        }
    }
}
//-----------------------------------------
//      Implementing the fmt Display
//-----------------------------------------
impl<T> fmt::Display for Vector2d<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign
        + Copy
        + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< {}, {} >", self.x, self.y)
    }
}

