// -----------------------------------------
//       Testing Vector 2d Module
// -----------------------------------------
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use assert_approx_eq::*;
#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use vectorlib::math::vector2d_module::Vector2d;
use vectorlib::math::vector2d_verbose_module::VerboseVector2d;

macro_rules! assert_vec2_equal {
    ($expected:expr, $actual:expr) => {
        let tolerance = 0.0001;
        assert_approx_eq!($expected.x, $actual.x, tolerance);
        assert_approx_eq!($expected.y, $actual.y, tolerance);
    };
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn vector_intializtion() {
    let v = Vector2d::new(10.0 as f32, 20.0 as f32);
    let w = Vector2d::new(20.0 as f32, 30.0 as f32);
    let s1 = v + w;
    let mut s2 = v + 2.3;
    let expected_output1 = Vector2d::new(30.0, 50.0);
    let expected_output2 = Vector2d::new(22.3, 42.3);
    println!("{s1:#?}");
    println!("{s2:#?}");
    assert_vec2_equal!(s1, expected_output1);
    s2 += v;
    assert_vec2_equal!(s2, expected_output2);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn vector_basic_operators() {
    let v = Vector2d::new(10.0, 20.0);
    let w = Vector2d::new(5.0, 5.0);
    let expected_output1 = Vector2d::new(13.3, 23.3);
    let scalar: f32 = 3.3;
    let s1 = v.add(scalar);
    let expected_output2 = Vector2d::new(15.0, 25.0);
    let s2 = v.add(w);
    assert_vec2_equal!(s1, expected_output1);
    assert_vec2_equal!(s2, expected_output2);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn zero_vector() {
    let v: Vector2d<f32> = Vector2d::zero_vector();
    let expected_output2 = Vector2d::new(0.0, 0.0);
    println!("{v}");
    println!("{}, {}", v.x, v.y);
    assert_vec2_equal!(v, expected_output2);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn ones_vector() {
    let v: Vector2d<f32> = Vector2d::ones_vector();
    let expected_output1 = Vector2d::new(1.00, 1.00);
    println!("v = {v}");
    assert_vec2_equal!(v, expected_output1);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn verbose_vector_vector() {
    let _v = VerboseVector2d::new(Vector2d::new(10.0, 20.0), true);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn testing_vector_fn() {
    let v: Vector2d<f32> = Vector2d::new(0.0, 0.0);
    let w: Vector2d<f32> = Vector2d::new(3.0, 4.0);
    let distance = v.distance(w);
    println!("{distance:#?} between {v} and {w}");
    assert_eq!(distance, 5.0);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn testing_rotation() {
    let w: Vector2d<f32> = Vector2d::new(3.0, 4.0);
    let rotated_w1 = Vector2d::rotate(w, 10.12);
    let rotated_w2 = w.rotate(10.12);
    let expected_output1 = Vector2d::new(0.25848866, -4.993314);
    println!("{rotated_w1} vs {rotated_w2}");
    assert_vec2_equal!(rotated_w1, expected_output1);
    // rotate by 45-degree for vector v
    let v: Vector2d<f32> = Vector2d::new(3.0, 3.0);
    let rotated_45_degree_v = v.rotate(FRAC_PI_2);
    let expected_output2 = Vector2d::new(-3.0, 3.0);
    println!("{v} vs {rotated_45_degree_v}");
    assert_vec2_equal!(rotated_45_degree_v, expected_output2);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn testing_projection() {
    let a = Vector2d::new(2.0, 4.0);
    let b = Vector2d::new(5.0, 2.0);
    let m = b.magnitude();
    let projection = b.normalize().unwrap().mul((a.dot_product(b)) / m);
    let r: Vector2d<f32> = a.projection_onto(b).unwrap_or(Vector2d::zero_vector());
    assert_vec2_equal!(r, projection);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn testing_lerp() {
    let v = Vector2d::new(0.0, 0.0);
    let w = Vector2d::new(6.0, 6.0);
    let v_lerp = v.lerp(Some(w), 0.5, None);
    let v_lerp_itself = w.lerp(None, 2.0, Some(true));
    let expected_output1 = Vector2d::new(3.0 as f64, 3.0 as f64);
    println!("{v_lerp}");
    println!("{v_lerp_itself}");
    assert_vec2_equal!(v_lerp, expected_output1);
    assert_vec2_equal!(v_lerp_itself, expected_output1);
}
