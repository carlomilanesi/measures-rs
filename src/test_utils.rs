#[macro_export]
macro_rules! assert_eq_tolerance {
    ($actual:expr, $expected:expr, $tolerance:expr) => {
        let a = $actual;
        let e = $expected;
        let abs_e = if e < 0. { -e } else { e };
        let t = $tolerance * if abs_e < 1. { 1. } else { abs_e };
        if (e - a).abs() > t {
            panic!(
                "Actual: {}, Expected: {}, Difference: {}, Tolerance: {}.",
                a,
                e,
                a - e,
                t,
            );
        }
    };
}

#[macro_export]
macro_rules! assert_eq_32 {
    ( $actual:expr, [ $expected1:expr , $expected2:expr , $expected3:expr $(,)? ] ) => {
        measures::assert_eq_tolerance!($actual[0] as f32, $expected1 as f32, f32::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[1] as f32, $expected2 as f32, f32::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[2] as f32, $expected3 as f32, f32::EPSILON * 64.);
    };
    ( $actual:expr, [ $expected1:expr , $expected2:expr $(,)? ] ) => {
        measures::assert_eq_tolerance!($actual[0] as f32, $expected1 as f32, f32::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[1] as f32, $expected2 as f32, f32::EPSILON * 64.);
    };
    ( $actual:expr, $expected:expr ) => {
        measures::assert_eq_tolerance!($actual as f32, $expected as f32, f32::EPSILON * 64.);
    };
}

#[macro_export]
macro_rules! assert_eq_64 {
    ( $actual:expr, [ $expected1:expr , $expected2:expr , $expected3:expr $(,)? ] ) => {
        measures::assert_eq_tolerance!($actual[0] as f64, $expected1 as f64, f64::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[1] as f64, $expected2 as f64, f64::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[2] as f64, $expected3 as f64, f64::EPSILON * 64.);
    };
    ( $actual:expr, [ $expected1:expr , $expected2:expr $(,)? ] ) => {
        measures::assert_eq_tolerance!($actual[0] as f64, $expected1 as f64, f64::EPSILON * 64.);
        measures::assert_eq_tolerance!($actual[1] as f64, $expected2 as f64, f64::EPSILON * 64.);
    };
    ($actual:expr, $expected:expr) => {
        measures::assert_eq_tolerance!($actual as f64, $expected as f64, f64::EPSILON * 64.);
    };
}
