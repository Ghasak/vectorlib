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
        + Default
        + std::cmp::PartialEq
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Result<Vector2d<T>, String>;
    fn div(self, other: Vector2d<T>) -> Result<Vector2d<T>, String> {
        if other.x == T::default() || other.y == T::default() {
            let error_message = format!("[ERROR] -> Cannot divide by zero vector: {}", other);
            Err(error_message)
        } else {
            Ok(Vector2d {
                x: self.x / other.x,
                y: self.y / other.y,
            })
        }
    }
}

impl<T> Div<T> for Vector2d<T>
where
    T: Add<T, Output = T>
        + Copy
        + AddAssign
        + Default
        + std::cmp::PartialEq
        + std::fmt::Display
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>,
{
    type Output = Result<Vector2d<T>, String>;

    fn div(self, scalar: T) -> Result<Vector2d<T>, String> {
        if scalar == T::default() {
            let error_message = format!("[ERROR] -> Cannot divide by zero scalar : {}", scalar);
            Err(error_message)
        } else {
            Ok(Vector2d {
                x: self.x / scalar,
                y: self.y / scalar,
            })
        }
    }
}

impl<T> Vector2d<T>
where
    T: std::fmt::Display
        + Copy
        + AddAssign
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>,
{
    /// Create a new `Vector2d` with the given `x` and `y` components.
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v = Vector2d::new(3, 4);
    /// assert_eq!(v.x, 3);
    /// assert_eq!(v.y, 4);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    /// Constructs a zero 2D vector with both `x` and `y` components set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let zero: Vector2d<f32> = Vector2d::zero_vector();
    /// assert_eq!(zero.x, 0.0);
    /// assert_eq!(zero.y, 0.0);
    /// ```
    pub fn zero_vector() -> Self
    where
        T: num_traits::NumAssign
            + num_traits::Zero
            + std::fmt::Display
            + Copy
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
    /// Constructs a 2D vector with both `x` and `y` components set to one.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let ones: Vector2d<f32> = Vector2d::ones_vector();
    /// assert_eq!(ones.x, 1.0);
    /// assert_eq!(ones.y, 1.0);
    /// ```
    pub fn ones_vector() -> Self
    where
        T: num_traits::NumAssign
            + num_traits::One
            + std::fmt::Display
            + Copy
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }

    /// Scale the vector by a give value.
    /// accept only scalar
    /// The `scale` method applies scaling to the vector
    /// by multiply its coordinates by the specified value of `scale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    /// use num_traits::{Float, FromPrimitive};
    /// use std::ops::{Mul, Sub};
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    /// let scalaed_vector = v.scale(1.0); // scale by 1 unit
    /// println!("Rotated: {:?}", scalaed_vector);
    /// ```
    /// # Parameters
    ///
    /// - `scale`: The scale by which to scale the vector.
    ///
    /// # Returns
    ///
    /// A new vector with the coordinates adjusted based on the scale value.
    ///
    /// # Constraints
    ///
    /// - `T`: The type of the vector's coordinates, which must support multiplication and subtraction.
    pub fn scale(self, scale: T) -> Vector2d<T>
    where
        T: num_traits::NumAssign
            + num_traits::One
            + std::fmt::Display
            + Copy
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        Vector2d {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    /// Magnitude
    ///
    /// Computes the magnitude of the vector.
    ///
    /// # Concept
    ///
    /// The magnitude of a vector is calculated based on the Pythagorean theorem, which states
    /// that the magnitude of a vector (represented as `(x, y)`) is the square root of the sum of
    /// the squares of its components:
    ///
    /// `magnitude = sqrt((x * x) + (y * y))`
    ///
    /// The magnitude value is returned as a type `T`.
    ///
    /// # Constraints
    ///
    /// The type `T` must implement the `Float`, `Mul`, `Add`, and `Copy` traits.
    ///
    /// # Example
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    /// let magnitude = v.magnitude();
    ///
    /// assert_eq!(magnitude, 5.0);
    ///
    pub fn magnitude(self) -> T
    where
        T: std::fmt::Display
            + Copy
            + Float
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
    /// Calculates the Euclidean distance between two 2D vectors.
    ///
    /// The Euclidean distance is the straight-line distance between two points
    /// in a two-dimensional space.
    ///
    /// # Arguments
    ///
    /// * `self` - The first vector.
    /// * `other` - The second vector to calculate the distance to.
    ///
    /// # Returns
    ///
    /// The calculated distance as a value of type `T`.
    ///
    /// # Type Constraints
    ///
    /// - `T`: Must implement `std::fmt::Display`, `Copy`, `Float`, `AddAssign`,
    ///        `Add<T, Output = T>`, `Sub<T, Output = T>`, `Mul<T, Output = T>`,
    ///        and `Div<T, Output = T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v1 = Vector2d::new(1.0, 2.0);
    /// let v2 = Vector2d::new(4.0, 6.0);
    ///
    /// let distance = v1.distance(v2);
    /// println!("Distance: {}", distance);
    /// ```

    pub fn distance(self, other: Vector2d<T>) -> T
    where
        T: std::fmt::Display
            + Copy
            + Float
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        // let dx = other.x.into() - self.x.into();
        // let dy = other.y.into() - self.y.into();
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    /// Normalize the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    /// use num_traits::{Float, Zero};
    /// use std::ops::Div;
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    /// let normalized = v.normalize();
    /// println!("Normalized: {:#?}", normalized);
    /// ```
    ///
    /// # Returns
    ///
    /// - If the magnitude of the vector is not zero, it returns a `Result` containing the normalized vector.
    /// - If the magnitude is zero, it returns an `Err` with an error message.
    pub fn normalize(&self) -> Result<Self, String>
    where
        T: std::fmt::Display
            + Copy
            + Float
            + Zero
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        let m = self.magnitude();
        let zero = T::zero();
        if m != zero {
            Ok(Self {
                x: self.x / m,
                y: self.y / m,
            })
        } else {
            Err("[::ERROR::] -> Cannot normalize a zero-length vector.".to_string())
        }
    }

    /// Rotate the vector by a given angle.
    ///
    /// The `rotate` method applies a rotation transformation to the vector
    /// by rotating its coordinates by the specified angle `theta`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    /// use num_traits::{Float, FromPrimitive};
    /// use std::ops::{Mul, Sub};
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    /// let rotated = v.rotate(1.0); // Rotate by 1 radian
    /// println!("Rotated: {:?}", rotated);
    /// ```
    ///
    /// # Type Parameters
    ///
    /// - `N`: The type of the angle `theta`.
    ///
    /// # Parameters
    ///
    /// - `theta`: The angle by which to rotate the vector.
    ///
    /// # Returns
    ///
    /// The rotated vector with the coordinates adjusted based on the rotation angle.
    ///
    /// # Constraints
    ///
    /// - `T`: The type of the vector's coordinates, which must support multiplication and subtraction.
    /// - `N`: The type of the angle, which must implement the `Num`, `Float`, and `Sub` traits.
    ///
    pub fn rotate(self, theta: T) -> Self
    where
        T: std::fmt::Display
            + Copy
            + Float
            + Zero
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        let cos_theta: T = T::cos(theta);
        let sin_theta: T = T::sin(theta);
        Self {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }

    /// Computes the dot product between two vectors.
    ///
    /// The dot product of two vectors is calculated by multiplying the corresponding
    /// components of the vectors and summing the results.
    ///
    /// # Arguments
    ///
    /// * `self` - The first vector.
    /// * `other` - The second vector.
    ///
    /// # Returns
    ///
    /// The dot product as a value of type `T`.
    ///
    /// # Constraints
    ///
    /// The type `T` must implement the `Mul` trait with `Output = T` and the `Add` trait
    /// with `Output = T`. It must also be `Copy`.
    ///
    /// # Example
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    /// let v1 = Vector2d::new(1.0, 2.0);
    /// let v2 = Vector2d::new(3.0, 4.0);
    /// let dot_product = v1.dot_product(v2);
    ///
    /// assert_eq!(dot_product, 11.0);
    ///
    pub fn dot_product(self, other: Vector2d<T>) -> T
    where
        T: std::fmt::Display
            + Copy
            + Float
            + Zero
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        self.x * other.x + self.y * other.y
    }

    /// Computes the projection of the vector onto another vector.
    ///
    /// The projection of a vector `self` onto another vector `other` is calculated
    /// by taking the dot product of the two vectors and dividing it by the square
    /// of the magnitude of `other`. The resulting value is then multiplied by `other`
    /// to obtain the projection vector.
    ///
    /// # Arguments
    ///
    /// * `self` - The vector to be projected.
    /// * `other` - The vector onto which `self` is projected.
    ///
    /// # Returns
    ///
    /// The projected vector as a new `Vector2d` instance.
    ///
    /// # Constraints
    ///
    /// The type `T` must implement the `Mul` trait with `Output = T` and be `Copy`.
    ///
    /// # Example
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    /// let other = Vector2d::new(2.0, 1.0);
    /// let projection = v.projection_onto(other);
    ///
    /// //assert_eq!(projection, Vector2d::new(1, 2));
    /// ```
    pub fn projection_onto(self, other: Vector2d<T>) -> Result<Self, String>
    where
        T: std::fmt::Display
            + Copy
            + Float
            + Zero
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        let projection_dot = self.dot_product(other);
        let m = other.magnitude() * other.magnitude();
        let temp = projection_dot / m;
        if temp != Zero::zero() {
            Ok(Self {
                x: other.x * temp,
                y: other.y * temp,
            })
        } else {
            Err("[::ERROR::] -> Cannot project on a zero-length vector.".to_string())
        }
    }

    /// Calculates the orthogonal vector of `self` with respect to `other`.
    ///
    /// The orthogonal vector is obtained by subtracting the projection of `self`
    /// onto `other` from `self`.
    ///
    /// # Arguments
    ///
    /// * `self` - The vector for which to calculate the orthogonal vector.
    /// * `other` - The vector with respect to which the orthogonal vector is calculated.
    ///
    /// # Returns
    ///
    /// If the projection of `self` onto `other` is successful, it returns `Ok(orthogonal)`,
    /// where `orthogonal` is the resulting orthogonal vector.
    /// If the projection fails, it returns `Err(error)`, where `error` is an error message.
    ///
    /// # Type Constraints
    ///
    /// - `T`: Must implement `std::fmt::Display`, `Copy`, `Float`, `Zero`,
    ///        `AddAssign`, `Add<T, Output = T>`, `Sub<T, Output = T>`,
    ///        `Mul<T, Output = T>`, and `Div<T, Output = T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v1 = Vector2d::new(1.0, 2.0);
    /// let v2 = Vector2d::new(2.0, 1.0);
    ///
    /// let result = v1.orthogonal_on(v2);
    /// match result {
    ///     Ok(orthogonal) => println!("Orthogonal vector: {:?}", orthogonal),
    ///     Err(error) => println!("Error: {}", error),
    /// }
    /// ```
    pub fn orthogonal_on(self, other: Vector2d<T>) -> Result<Self, String>
    where
        T: std::fmt::Display
            + Copy
            + Float
            + Zero
            + AddAssign
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        let projection_vector = self.projection_onto(other);
        if let Ok(vector) = projection_vector {
            Ok(self.sub(vector))
        } else {
            Err("[::ERROR::] Cannot get orthogonal vector for zero vector length".to_string())
        }
    }

    /// Linearly interpolates between two vectors.
    ///
    /// The `lerp` function calculates the linear interpolation between `self` and `other`
    /// based on a given interpolation factor `t`. The interpolation factor `t` should be
    /// in the range [0, 1], where 0 represents `self` and 1 represents `other`. The resulting
    /// vector is a blend of the two vectors based on the interpolation factor.
    ///
    /// # Arguments
    ///
    /// * `self` - The starting vector for the interpolation.
    /// * `other` - The target vector for the interpolation.
    /// * `t` - The interpolation factor. Should be in the range [0, 1].
    ///
    /// # Returns
    ///
    /// The resulting vector after the linear interpolation between `self` and `other`.
    ///
    /// # Type Constraints
    ///
    /// - `T`: Must implement `std::fmt::Display`, `Copy`, `Add<T, Output = T>`,
    ///        `Sub<T, Output = T>`, and `Mul<T, Output = T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v1 = Vector2d::new(1.0, 2.0);
    /// let v2 = Vector2d::new(4.0, 6.0);
    ///
    /// let result = v1.lerp(Some(v2), 0.5, None);
    /// println!("Interpolated vector: {:?}", result);
    /// ```
    #[allow(clippy::collapsible_else_if)]
    pub fn lerp(self, other: Option<Self>, percentage: T, slop: Option<bool>) -> Self
    where
        T: std::fmt::Display
            + Copy
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Float,
    {
        if let Some(other) = other {
            if let Some(true) = slop {
                let x = self.x + (other.x - self.x) * percentage.powi(-1);
                let y = self.y + (other.y - self.y) * percentage.powi(-1);
                Self { x, y }
            } else {
                let x = self.x + (other.x - self.x) * percentage;
                let y = self.y + (other.y - self.y) * percentage;
                Self { x, y }
            }
        } else {
            if let Some(true) = slop {
                let x = self.x * percentage.powi(-1);
                let y = self.y * percentage.powi(-1);
                Self { x, y }
            } else {
                let x = self.x * percentage;
                let y = self.y * percentage;
                Self { x, y }
            }
        }
    }
    /// Rounds the vector's fields to the nearest whole numbers.
    ///
    /// The `round` method rounds the `x` and `y` fields of the vector to the nearest
    /// whole numbers. The rounding is done using the standard rounding rules.
    ///
    /// # Arguments
    ///
    /// * `self` - The vector to be rounded.
    ///
    /// # Returns
    ///
    /// The resulting vector with `x` and `y` rounded to the nearest whole numbers.
    ///
    /// # Type Constraints
    ///
    /// - `T`: Must implement `std::fmt::Display`, `Copy`, `Add<T, Output = T>`,
    ///        `Sub<T, Output = T>`, and `Mul<T, Output = T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vectorlib::math::vector2d_module::Vector2d;
    ///
    /// let v = Vector2d::new(3.4, -2.8);
    ///
    /// let rounded = v.round();
    /// println!("Rounded vector: {:?}", rounded);
    /// ```
    pub fn round(self) -> Self
    where
        T: std::fmt::Display
            + Copy
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Float,
    {
        let x = self.x.round();
        let y = self.y.round();
        Self { x, y }
    }
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
        write!(f, "< {:.3}, {:.3} >", self.x, self.y)
    }
}
