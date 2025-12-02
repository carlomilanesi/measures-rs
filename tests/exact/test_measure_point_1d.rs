use measures::{assert_eq_32, assert_eq_64, traits::Trigonometry};
use units::{
    barycentric_combination, midpoint, weighted_midpoint, ApproxMeasurePoint, Celsius, Degree,
    Fahrenheit, Measure, MeasurePoint,
};

mod units {
    measures::define_measure_types! {
        with_points exact with_approx,
        scalar_properties [
            Temperature [
                Celsius {
                    suffix: " \u{B0}C",
                    ratio: 1.,
                    offset: 273.15,
                }
                Fahrenheit {
                    suffix: " \u{B0}F",
                    ratio: 5. / 9.,
                    offset: 273.15 - 32. * 5. / 9.,
                }
            ]
        ]
        angle_measurement_units [
            Degree {
                suffix: " deg",
                cycle_fraction: 360.,
            }
        ]
    }
}

#[test]
fn measure_point_1d_new() {
    let m1: MeasurePoint<Celsius, f32> = MeasurePoint::<Celsius, f32>::new(12.);
    assert_eq!(m1.value, 12_f32);
    let m2: MeasurePoint<Celsius> = MeasurePoint::<Celsius>::new(12.);
    assert_eq!(m2.value, 12_f64);
}

#[test]
fn measure_point_1d_convert() {
    // 0 °C is 32 °F
    let mp1 = MeasurePoint::<Celsius, f32>::new(0.);
    let mp2: MeasurePoint<Fahrenheit, f32> = mp1.convert::<Fahrenheit>();
    assert_eq_32!(mp2.value, 32_f32);

    // 68 °F is 20 °C
    let mp3 = MeasurePoint::<Fahrenheit, f32>::new(68.);
    let mp4: MeasurePoint<Celsius, f32> = mp3.convert::<Celsius>();
    assert_eq_32!(mp4.value, 20_f32);

    // 0 °C is 32 °F
    let mp5 = MeasurePoint::<Celsius>::new(0.);
    let mp6: MeasurePoint<Fahrenheit> = mp5.convert::<Fahrenheit>();
    assert_eq_64!(mp6.value, 32_f64);

    // 68 °F is 20 °C
    let mp7 = MeasurePoint::<Fahrenheit>::new(68.);
    let mp8: MeasurePoint<Celsius> = mp7.convert::<Celsius>();
    assert_eq_64!(mp8.value, 20_f64);
}

#[test]
fn measure_point_1d_lossless_into_32_to_32() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    #[allow(clippy::useless_conversion)]
    let m2: MeasurePoint<Celsius, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_1d_lossless_into_32_to_64() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m2: MeasurePoint<Celsius, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_1d_lossless_into_64_to_64() {
    let m1 = MeasurePoint::<Celsius, f64>::new(12.);
    #[allow(clippy::useless_conversion)]
    let m2: MeasurePoint<Celsius, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

/*
// ILLEGAL
#[test]
fn measure_point_1d_lossless_into_64_to_32() {
    let m1 = MeasurePoint::<Celsius, f64>::new(12.);
    let m2: MeasurePoint<Celsius, f32> = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_point_1d_lossy_into_32_to_32() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m2: MeasurePoint<Celsius, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_1d_lossy_into_32_to_64() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m2: MeasurePoint<Celsius, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_1d_lossy_into_64_to_32() {
    let m1 = MeasurePoint::<Celsius, f64>::new(12.);
    let m2: MeasurePoint<Celsius, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_1d_lossy_into_64_to_64() {
    let m1 = MeasurePoint::<Celsius, f64>::new(12.);
    let m2: MeasurePoint<Celsius, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_min() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m2 = MeasurePoint::<Celsius, f32>::new(13.);
    let m3: MeasurePoint<Celsius, f32> = m1.min(m2);
    let m4: MeasurePoint<Celsius, f32> = m2.min(m1);
    let m5: MeasurePoint<Celsius, f32> = m1.min(m1);
    assert_eq!(m3.value, 12.);
    assert_eq!(m4.value, 12.);
    assert_eq!(m5.value, 12.);
}

#[test]
fn measure_max() {
    let m1 = MeasurePoint::<Celsius>::new(12.);
    let m2 = MeasurePoint::<Celsius>::new(13.);
    let m3: MeasurePoint<Celsius> = m1.max(m2);
    let m4: MeasurePoint<Celsius> = m2.max(m1);
    let m5: MeasurePoint<Celsius> = m1.max(m1);
    assert_eq!(m3.value, 13.);
    assert_eq!(m4.value, 13.);
    assert_eq!(m5.value, 12.);
}

#[test]
fn measure_clamp() {
    let m1 = MeasurePoint::<Celsius>::new(12.);
    let m2 = MeasurePoint::<Celsius>::new(13.2);
    let m3 = MeasurePoint::<Celsius>::new(14.);
    assert_eq!(m1.clamp(m2, m3), m2);
    assert_eq!(m1.clamp(m3, m2), m2);
    assert_eq!(m2.clamp(m1, m3), m2);
    assert_eq!(m2.clamp(m3, m1), m2);
    assert_eq!(m3.clamp(m1, m2), m2);
    assert_eq!(m3.clamp(m2, m1), m2);
}

#[test]
fn measure_point_1d_default() {
    let mp: MeasurePoint<Celsius, f32> = MeasurePoint::default();
    assert_eq!(mp.value, 0_f32);

    let mp = MeasurePoint::<Celsius>::default();
    assert_eq!(mp.value, 0_f64);
}

#[test]
fn measure_from_f32_into_f64() {
    let m1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m2 = MeasurePoint::<Celsius, f64>::from(m1);
    assert_eq!(m2.value, 12.);
    let m3: MeasurePoint<Celsius, f64> = m1.into();
    assert_eq!(m3.value, 12.);
}

#[test]
fn measure_from_approx_into_measure() {
    let am = ApproxMeasurePoint::<Celsius, f32>::with_variance(12., 0.09);
    let m4 = MeasurePoint::<Celsius, f32>::from(am);
    assert_eq!(m4.value, 12_f32);
    let m5: MeasurePoint<Celsius, f32> = am.into();
    assert_eq!(m5.value, 12_f32);

    let am = ApproxMeasurePoint::<Celsius>::with_variance(12., 0.09);
    let m4 = MeasurePoint::<Celsius>::from(am);
    assert_eq!(m4.value, 12_f64);
    let m5: MeasurePoint<Celsius> = am.into();
    assert_eq!(m5.value, 12_f64);
}

#[test]
fn measure_point_1d_addition_of_measure() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m = Measure::<Celsius, f32>::new(7.);
    let mp2: MeasurePoint<Celsius, f32> = mp1 + m;
    assert_eq!(mp2.value, 19_f32);

    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let m = Measure::<Celsius>::new(7.);
    let mp2: MeasurePoint<Celsius> = mp1 + m;
    assert_eq!(mp2.value, 19_f64);
}

#[test]
fn measure_point_1d_addition_of_measure_assignment() {
    let mut mp = MeasurePoint::<Celsius, f32>::new(12.);
    mp += Measure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 19_f32);

    let mut mp = MeasurePoint::<Celsius>::new(12.);
    mp += Measure::<Celsius>::new(7.);
    assert_eq!(mp.value, 19_f64);
}

#[test]
fn measure_point_1d_subtraction_of_measure() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m = Measure::<Celsius, f32>::new(7.);
    let mp2: MeasurePoint<Celsius, f32> = mp1 - m;
    assert_eq!(mp2.value, 5_f32);

    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let m = Measure::<Celsius>::new(7.);
    let mp2: MeasurePoint<Celsius> = mp1 - m;
    assert_eq!(mp2.value, 5_f64);
}

#[test]
fn measure_point_1d_subtraction_of_measure_assignment() {
    let mut mp = MeasurePoint::<Celsius, f32>::new(12.);
    mp -= Measure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 5_f32);

    let mut mp = MeasurePoint::<Celsius>::new(12.);
    mp -= Measure::<Celsius>::new(7.);
    assert_eq!(mp.value, 5_f64);
}

#[test]
fn measures_point_subtraction() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(7.);
    let m: Measure<Celsius, f32> = mp1 - mp2;
    assert_eq!(m.value, 5_f32);

    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let mp2 = MeasurePoint::<Celsius>::new(7.);
    let m: Measure<Celsius> = mp1 - mp2;
    assert_eq!(m.value, 5_f64);
}

#[test]
fn measures_point_weighted_midpoint() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3: MeasurePoint<Celsius, f32> = weighted_midpoint(mp1, mp2, 0.4);
    assert_eq_32!(mp3.value, 26_f32);

    let mp1 = MeasurePoint::<Celsius>::new(20.);
    let mp2 = MeasurePoint::<Celsius>::new(30.);
    let mp3: MeasurePoint<Celsius> = weighted_midpoint(mp1, mp2, 0.4);
    assert_eq_64!(mp3.value, 26_f64);
}

#[test]
fn measures_point_midpoint() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3: MeasurePoint<Celsius, f32> = midpoint(mp1, mp2);
    assert_eq_32!(mp3.value, 25_f32);

    let mp1 = MeasurePoint::<Celsius>::new(20.);
    let mp2 = MeasurePoint::<Celsius>::new(30.);
    let mp3: MeasurePoint<Celsius> = midpoint(mp1, mp2);
    assert_eq_64!(mp3.value, 25_f64);
}

#[test]
fn measures_point_barycentric_combination() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(80.);
    let mp4: MeasurePoint<Celsius, f32> =
        barycentric_combination(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq_32!(mp4.value, 67.);
}

#[test]
fn measure_trigonometry() {
    let half_sqrt = 1. / 2_f64.sqrt();
    let three_sqrt = 3_f64.sqrt();

    let m = MeasurePoint::<Degree>::new(0.);
    assert_eq!(m.cos(), 1_f64);
    assert_eq!(m.sin(), 0_f64);
    assert_eq!(m.tan(), 0_f64);
    assert_eq!(m.sin_cos(), (0_f64, 1_f64));

    let m = MeasurePoint::<Degree>::new(30.);
    assert_eq_64!(m.cos(), three_sqrt * 0.5);
    assert_eq_64!(m.sin(), 0.5_f64);
    assert_eq_64!(m.tan(), 1. / three_sqrt);
    assert_eq_64!(m.sin_cos().0, 0.5_f64);
    assert_eq_64!(m.sin_cos().1, three_sqrt * 0.5);

    let m = MeasurePoint::<Degree>::new(45.);
    assert_eq_64!(m.cos(), half_sqrt);
    assert_eq_64!(m.sin(), half_sqrt);
    assert_eq_64!(m.tan(), 1_f64);
    assert_eq_64!(m.sin_cos().0, half_sqrt);
    assert_eq_64!(m.sin_cos().1, half_sqrt);

    let m = MeasurePoint::<Degree>::new(60.);
    assert_eq_64!(m.cos(), 0.5_f64);
    assert_eq_64!(m.sin(), three_sqrt * 0.5);
    assert_eq_64!(m.tan(), three_sqrt);
    assert_eq_64!(m.sin_cos().0, three_sqrt * 0.5);
    assert_eq_64!(m.sin_cos().1, 0.5_f64);

    let m = MeasurePoint::<Degree>::new(90.);
    assert_eq_64!(m.cos(), 0_f64);
    assert_eq_64!(m.sin(), 1_f64);
    assert!(m.tan().abs() > 1e12_f64);
    assert_eq_64!(m.sin_cos().0, 1_f64);
    assert_eq_64!(m.sin_cos().1, 0_f64);

    let m = MeasurePoint::<Degree>::new(-45.);
    assert_eq_64!(m.cos(), half_sqrt);
    assert_eq_64!(m.sin(), -half_sqrt);
    assert_eq_64!(m.tan(), -1_f64);
    assert_eq_64!(m.sin_cos().0, -half_sqrt);
    assert_eq_64!(m.sin_cos().1, half_sqrt);

    let m = MeasurePoint::<Degree>::new(-135.);
    assert_eq_64!(m.cos(), -half_sqrt);
    assert_eq_64!(m.sin(), -half_sqrt);
    assert_eq_64!(m.tan(), 1_f64);
    assert_eq_64!(m.sin_cos().0, -half_sqrt);
    assert_eq_64!(m.sin_cos().1, -half_sqrt);
}

#[test]
fn measure_point_1d_equals() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));

    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let mp2 = MeasurePoint::<Celsius>::new(12.);
    let mp3 = MeasurePoint::<Celsius>::new(13.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));
}

#[test]
fn measure_point_1d_differs() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);

    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let mp2 = MeasurePoint::<Celsius>::new(12.);
    let mp3 = MeasurePoint::<Celsius>::new(13.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn measure_point_1d_partial_cmp() {
    let mp1 = MeasurePoint::<Celsius>::new(12.);
    let mp2 = MeasurePoint::<Celsius>::new(12.);
    let mp3 = MeasurePoint::<Celsius>::new(13.);
    let mp4 = MeasurePoint::<Celsius>::new(f64::NAN);
    use core::cmp::Ordering;
    assert_eq!(mp1.partial_cmp(&mp2), Some(Ordering::Equal));
    assert_eq!(mp1.partial_cmp(&mp3), Some(Ordering::Less));
    assert_eq!(mp3.partial_cmp(&mp1), Some(Ordering::Greater));
    assert_eq!(mp1.partial_cmp(&mp4), None);
}

#[test]
fn measure_point_1d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_1d_formatting_in_celsius() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{}", mp), "at 12.25 °C");
}

#[test]
fn measure_point_1d_formatting_in_celsius_with_one_fractional_digit() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1}", mp), "at 12.2 °C");
}

#[test]
fn measure_point_1d_formatting_for_debug_in_celsius() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:?}", mp), "at 12.25 °C");
}

#[test]
fn measure_point_1d_formatting_for_debug_in_celsius_with_one_fractional_digit() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", mp), "at 12.2 °C");
}

#[test]
fn measure_point_1d_formatting_for_lowerexp_in_celsius() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:e}", mp), "at 1.225e1 °C");
}

#[test]
fn measure_point_1d_formatting_for_lowerexp_in_celsius_with_one_fractional_digit() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1e}", mp), "at 1.2e1 °C");
}

#[test]
fn measure_point_1d_formatting_for_upperexp_in_celsius() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:E}", mp), "at 1.225E1 °C");
}

#[test]
fn measure_point_1d_formatting_for_upperexp_in_celsius_with_one_fractional_digit() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1E}", mp), "at 1.2E1 °C");
}

#[test]
fn measure_point_1d_traits() {
    fn impl_common_traits<
        T: Sized
            + Copy
            + Clone
            + Default
            + core::fmt::Debug
            + core::fmt::Display
            + Send
            + Sync
            + PartialEq
            + Unpin
            + PartialOrd,
    >() {
    }
    impl_common_traits::<MeasurePoint<Celsius>>();
}
