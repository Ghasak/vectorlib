// -----------------------------------------
//       Testing Vector 2d Module
// -----------------------------------------
#[allow(unused_imports)]
use assert_approx_eq::*;
#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use super::*;
use vectorlib::math::vector2d_module::Vector2d;



macro_rules! assert_vec2_equal {
    ($expected:expr, $actual:expr) => {
        let tolerance = 0.0001;
        assert_approx_eq!($expected.x, $actual.x, tolerance);
        assert_approx_eq!($expected.y, $actual.y, tolerance);
    };
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn vector_intializtion(){
    let v = Vector2d::new(10.0 as f32, 20.0 as f32);
    let w = Vector2d::new(20.0 as f32, 30.0 as f32);
    let s1 = v + w;
    let mut s2 = v + 2.3;
    let expected_output1 = Vector2d::new(30.0, 50.0);
    let expected_output2 = Vector2d::new(22.3, 42.3);
    println!("{s1:#?}");
    println!("{s2:#?}");
    assert_vec2_equal!(s1, expected_output1);
    s2 +=v;
    assert_vec2_equal!(s2, expected_output2);
}



