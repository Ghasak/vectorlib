# Legacy Code

- In the implementation of the `Vector2d`, you don't need to use the `add`,
`mul`, `div` and `sub` inside the `Vector2d` definition, since we have already
implemented them using the traits `Add, Sub, Mul and Div`.

```rust
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

```
