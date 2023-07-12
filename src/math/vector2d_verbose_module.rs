use crate::math::vector2d_module::Vector2d;
pub use core::fmt;
#[allow(unused_imports)]
use num_traits::{Float, FromPrimitive, Num, Zero};
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};



macro_rules! function_location {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}

// -----------------------------------------
//       Verbose Vector Implemnetation
// -----------------------------------------
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct VerboseVector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign,
{
    vector: Option<Vector2d<T>>,
    verbose: bool,
}

#[allow(dead_code)]
impl<T> VerboseVector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign,
{
    pub fn new(vector: Vector2d<T>, verbose: bool) -> Self {
        VerboseVector2d {
            vector: Some(vector),
            verbose,
        }
    }

    pub fn display_if_verbose(&self)
    where
        T: std::fmt::Display,
    {
        if self.verbose {
            if let Some(vector) = self.vector {
                println!("Vector2d: ({}, {})", vector.x, vector.y);
            }
        }
    }

    pub fn into_inner(mut self) -> Option<Vector2d<T>> {
        let vector = self.vector.take();
        if self.verbose && vector.is_some() {
            if let Some(vector) = vector {
                println!("Vector2d: ({}, {})", vector.x, vector.y);
            }
        }
        vector
    }
}

impl<T> Drop for VerboseVector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign,
{
    fn drop(&mut self) {
        if self.verbose {
            if let Some(vector) = self.vector.take() {
                let function_call = function_location!();
                println!(
                    "Vector2d: <{:.3}, {:.3}> has been droped from memeory",
                    vector.x, vector.y
                );
                println!("Last call before dropped: {function_call}")
            }
        }
    }
}

// -----------------------------------------
//       Implementing the fmt Display
// -----------------------------------------
impl<T> fmt::Display for VerboseVector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + std::fmt::Display
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(vector) = self.vector {
            write!(f, "Vector2d: <{:.3}, {:.3}>", vector.x, vector.y)?;
        }
        write!(f, " - Verbose: {}", self.verbose)
    }
}


