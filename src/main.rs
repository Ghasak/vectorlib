pub mod math;
#[allow(unused_imports)]
use crate::math::vector2d_module::Vector2d;
#[allow(unused_imports)]
use crate::math::vector2d_verbose_module::VerboseVector2d;
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

fn main() {

    let v = Vector2d::new(0.0, 0.0);
    let w = Vector2d::new(6.0, 6.0);
    let v_lerp = v.lerp(Some(w), 0.5, None);
    let v_lerp_itself = w.lerp(None, 2.0, Some(true));
    println!("{v_lerp}");
    println!("{v_lerp_itself}")
}
