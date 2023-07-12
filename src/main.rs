pub mod math;
use crate::math::vector2d_module::Vector2d;
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
fn main(){

    let v = Vector2d::new(10.0, 20.0);
    let w  = Vector2d::new(5.0, 5.0);
    let scalar : f32 = 3.3;
    let _s1 = v.add(scalar);
    let _s2 = v.add(w);



}
