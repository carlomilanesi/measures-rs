use measures::{assert_eq_64, dimensionless::One, traits::Trigonometry};
use units::{ApproxMeasure, Degree, Measure, Metre, Millimetre};

mod units {
    measures::define_measure_types! {
        exact with_approx,
        vector_properties [
            Length [
                Metre {
                    suffix: " m",
                }
                Millimetre {
                    suffix: " mm",
                    ratio: 1e-3,
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
fn measure_1d_new() {
    let m1: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    assert_eq!(m1.value, 12_f32);
    let m2: Measure<Metre> = Measure::<Metre>::new(12.);
    assert_eq!(m2.value, 12_f64);
}

#[test]
fn measure_1d_convert() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Millimetre, f32> = m1.convert::<Millimetre>();
    assert_eq!(m1.value, 12.);
    assert_eq!(m2.value, 12000.);
}

#[test]
fn measure_1d_lossless_into_32_to_32() {
    let m1 = Measure::<Metre, f32>::new(12.);
    #[allow(clippy::useless_conversion)]
    let m2: Measure<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_lossless_into_32_to_64() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_lossless_into_64_to_64() {
    let m1 = Measure::<Metre, f64>::new(12.);
    #[allow(clippy::useless_conversion)]
    let m2: Measure<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

/*
ILLEGAL
#[test]
fn measure_1d_lossless_into_64_to_32() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_1d_lossy_into_32_to_32() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_lossy_into_32_to_64() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_lossy_into_64_to_32() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_lossy_into_64_to_64() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_norm_positive() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.norm();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_norm_negative() {
    let m1 = Measure::<Metre, f64>::new(-12.);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_1d_norm_zero() {
    let m1 = Measure::<Metre, f64>::new(0.);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(m2.value, 0.);
}

#[test]
fn measure_1d_squared_norm_positive() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: f32 = m1.squared_norm();
    assert_eq!(m2, 12. * 12.);
}

#[test]
fn measure_1d_squared_norm_negative() {
    let m1 = Measure::<Metre, f64>::new(-12.);
    let m2: f64 = m1.squared_norm();
    assert_eq!(m2, 12. * 12.);
}

#[test]
fn measure_1d_squared_norm_zero() {
    let m1 = Measure::<Metre, f64>::new(0.);
    let m2: f64 = m1.squared_norm();
    assert_eq!(m2, 0.);
}

#[test]
fn measure_1d_normalized_positive() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, 1.);
}

#[test]
fn measure_1d_normalized_negative() {
    let m1 = Measure::<Metre, f64>::new(-12.);
    let m2: Measure<Metre, f64> = m1.normalized();
    assert_eq!(m2.value, -1.);
}

#[test]
fn measure_1d_normalized_positive_zero() {
    let m1 = Measure::<Metre, f32>::new(0.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, 1.);
}

#[test]
fn measure_1d_normalized_negative_zero() {
    let m1 = Measure::<Metre, f32>::new(-0.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, -1.);
}

#[test]
fn measure_1d_min() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(13.);
    let m3: Measure<Metre> = m1.min(m2);
    let m4: Measure<Metre> = m2.min(m1);
    let m5: Measure<Metre> = m1.min(m1);
    assert_eq!(m3.value, 12.);
    assert_eq!(m4.value, 12.);
    assert_eq!(m5.value, 12.);
}

#[test]
fn measure_1d_max() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(13.);
    let m3: Measure<Metre> = m1.max(m2);
    let m4: Measure<Metre> = m2.max(m1);
    let m5: Measure<Metre> = m1.max(m1);
    assert_eq!(m3.value, 13.);
    assert_eq!(m4.value, 13.);
    assert_eq!(m5.value, 12.);
}

#[test]
fn measure_1d_clamp() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(13.2);
    let m3 = Measure::<Metre>::new(14.);
    assert_eq!(m1.clamp(m2, m3), m2);
    assert_eq!(m1.clamp(m3, m2), m2);
    assert_eq!(m2.clamp(m1, m3), m2);
    assert_eq!(m2.clamp(m3, m1), m2);
    assert_eq!(m3.clamp(m1, m2), m2);
    assert_eq!(m3.clamp(m2, m1), m2);
}

#[test]
fn measure_1d_default() {
    let m: Measure<Metre, f32> = Measure::default();
    assert_eq!(m.value, 0_f32);
    let m = Measure::<Metre>::default();
    assert_eq!(m.value, 0_f64);
}

#[test]
fn measure_1d_from_f32_into_f64() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f64>::from(m1);
    assert_eq!(m2.value, 12.);
    let m3: Measure<Metre, f64> = m1.into();
    assert_eq!(m3.value, 12.);
}

#[test]
fn measure_1d_from_approx_into_measure() {
    let am = ApproxMeasure::<Metre, f32>::with_variance(12., 0.09);
    let m4 = Measure::<Metre, f32>::from(am);
    assert_eq!(m4.value, 12.);
    let m5: Measure<Metre, f32> = am.into();
    assert_eq!(m5.value, 12.);
}

#[test]
fn measure_1d_unary_minus() {
    let m1 = -Measure::<Metre>::new(12.);
    assert_eq!(m1.value, -12_f64);
    let m2 = -Measure::<Metre>::new(-13.);
    assert_eq!(m2.value, 13_f64);
    let m3 = -Measure::<Metre>::new(0.);
    assert_eq!(m3.value, 0_f64);
    let m4 = -Measure::<Metre>::new(-0.);
    assert_eq!(m4.value, 0_f64);
}

#[test]
fn measure_1d_addition() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(7.);
    let m3: Measure<Metre> = m1 + m2;
    assert_eq!(m3.value, 19_f64);
    let m4 = Measure::<Metre>::new(-7.);
    let m5: Measure<Metre> = m1 + m4;
    assert_eq!(m5.value, 5_f64);
}

#[test]
fn measure_1d_addition_assignment() {
    let mut m = Measure::<Metre>::new(12.);
    m += Measure::<Metre>::new(7.);
    assert_eq!(m.value, 19_f64);
    m += Measure::<Metre>::new(-3.);
    assert_eq!(m.value, 16_f64);
}

#[test]
fn measure_1d_subtraction() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(7.);
    let m3: Measure<Metre> = m1 - m2;
    assert_eq!(m3.value, 5_f64);
    let m4 = Measure::<Metre>::new(-7.);
    let m5: Measure<Metre> = m1 - m4;
    assert_eq!(m5.value, 19_f64);
}

#[test]
fn measure_1d_subtraction_assignment() {
    let mut m = Measure::<Metre>::new(12.);
    m -= Measure::<Metre>::new(7.);
    assert_eq!(m.value, 5_f64);
    m -= Measure::<Metre>::new(-3.);
    assert_eq!(m.value, 8_f64);
}

#[test]
fn measure_1d_scalar_post_multiplication() {
    let m1 = Measure::<Metre, f32>::new(12.) * 3.;
    assert_eq!(m1.value, 36_f32);

    let m2 = Measure::<Metre, f64>::new(12.) * 3.;
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_one_post_multiplication() {
    let m1 = Measure::<Metre, f32>::new(12.) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.value, 36_f32);

    let m2 = Measure::<Metre, f64>::new(12.) * Measure::<One, f64>::new(3.);
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_one_pre_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure::<Metre, f32>::new(3.);
    assert_eq!(m1.value, 36_f32);

    let m2 = Measure::<One>::new(12.) * Measure::<Metre>::new(3.);
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_one_one_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.value, 36_f32);

    let m2 = Measure::<One>::new(12.) * Measure::<One>::new(3.);
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_scalar_multiplication_assignment() {
    let mut m1 = Measure::<Metre, f32>::new(12.);
    m1 *= 3.;
    assert_eq!(m1.value, 36_f32);

    let mut m2 = Measure::<Metre>::new(12.);
    m2 *= 3.;
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_one_multiplication_assignment() {
    let mut m1 = Measure::<Metre, f32>::new(12.);
    m1 *= Measure::<One, f32>::new(3.);
    assert_eq!(m1.value, 36_f32);

    let mut m2 = Measure::<Metre>::new(12.);
    m2 *= Measure::<One>::new(3.);
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_scalar_pre_multiplication() {
    let m1 = 3. * Measure::<Metre, f32>::new(12.);
    assert_eq!(m1.value, 36_f32);

    let m2 = 3. * Measure::<Metre>::new(12.);
    assert_eq!(m2.value, 36_f64);
}

#[test]
fn measure_1d_scalar_division() {
    let m1 = Measure::<Metre, f32>::new(12.) / 3.;
    assert_eq!(m1.value, 4_f32);

    let m2 = Measure::<Metre>::new(12.) / 3.;
    assert_eq!(m2.value, 4_f64);
}

#[test]
fn measure_1d_measure_division() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(3.);
    let m3: Measure<One, f32> = m1 / m2;
    assert_eq!(m3.value, 4_f32);

    let m4 = Measure::<Metre>::new(12.);
    let m5 = Measure::<Metre>::new(3.);
    let m6: Measure<One> = m4 / m5;
    assert_eq!(m6.value, 4_f64);
}

#[test]
fn measure_1d_one_division() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure<Metre, f32> = m1 / m2;
    assert_eq!(m3.value, 4_f32);

    let m4 = Measure::<Metre>::new(12.);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure<Metre> = m4 / m5;
    assert_eq!(m6.value, 4_f64);
}

#[test]
fn measure_1d_one_one_division() {
    let m1 = Measure::<One, f32>::new(12.);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure<One, f32> = m1 / m2;
    assert_eq!(m3.value, 4_f32);

    let m4 = Measure::<One>::new(12.);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure<One> = m4 / m5;
    assert_eq!(m6.value, 4_f64);
}

#[test]
fn measure_1d_scalar_division_assignment() {
    let mut m1 = Measure::<Metre, f32>::new(12.);
    m1 /= 3.;
    assert_eq!(m1.value, 4_f32);

    let mut m2 = Measure::<Metre>::new(12.);
    m2 /= 3.;
    assert_eq!(m2.value, 4_f64);
}

#[test]
fn measure_1d_one_division_assignment() {
    let mut m1 = Measure::<Metre, f32>::new(12.);
    m1 /= Measure::<One, f32>::new(3.);
    assert_eq!(m1.value, 4_f32);

    let mut m2 = Measure::<Metre>::new(12.);
    m2 /= Measure::<One>::new(3.);
    assert_eq!(m2.value, 4_f64);
}

#[test]
fn measure_1d_trigonometry() {
    let half_sqrt = 1. / 2_f64.sqrt();
    let three_sqrt = 3_f64.sqrt();

    let m = Measure::<Degree>::new(0.);
    assert_eq!(m.cos(), 1_f64);
    assert_eq!(m.sin(), 0_f64);
    assert_eq!(m.tan(), 0_f64);
    assert_eq!(m.sin_cos(), (0_f64, 1_f64));

    let m = Measure::<Degree>::new(30.);
    assert_eq_64!(m.cos(), three_sqrt * 0.5);
    assert_eq_64!(m.sin(), 0.5_f64);
    assert_eq_64!(m.tan(), 1. / three_sqrt);
    assert_eq_64!(m.sin_cos().0, 0.5_f64);
    assert_eq_64!(m.sin_cos().1, three_sqrt * 0.5);

    let m = Measure::<Degree>::new(45.);
    assert_eq_64!(m.cos(), half_sqrt);
    assert_eq_64!(m.sin(), half_sqrt);
    assert_eq_64!(m.tan(), 1_f64);
    assert_eq_64!(m.sin_cos().0, half_sqrt);
    assert_eq_64!(m.sin_cos().1, half_sqrt);

    let m = Measure::<Degree>::new(60.);
    assert_eq_64!(m.cos(), 0.5_f64);
    assert_eq_64!(m.sin(), three_sqrt * 0.5);
    assert_eq_64!(m.tan(), three_sqrt);
    assert_eq_64!(m.sin_cos().0, three_sqrt * 0.5);
    assert_eq_64!(m.sin_cos().1, 0.5_f64);

    let m = Measure::<Degree>::new(90.);
    assert_eq_64!(m.cos(), 0_f64);
    assert_eq_64!(m.sin(), 1_f64);
    assert!(m.tan().abs() > 1e12_f64);
    assert_eq_64!(m.sin_cos().0, 1_f64);
    assert_eq_64!(m.sin_cos().1, 0_f64);

    let m = Measure::<Degree>::new(-45.);
    assert_eq_64!(m.cos(), half_sqrt);
    assert_eq_64!(m.sin(), -half_sqrt);
    assert_eq_64!(m.tan(), -1_f64);
    assert_eq_64!(m.sin_cos().0, -half_sqrt);
    assert_eq_64!(m.sin_cos().1, half_sqrt);

    let m = Measure::<Degree>::new(-135.);
    assert_eq_64!(m.cos(), -half_sqrt);
    assert_eq_64!(m.sin(), -half_sqrt);
    assert_eq_64!(m.tan(), 1_f64);
    assert_eq_64!(m.sin_cos().0, -half_sqrt);
    assert_eq_64!(m.sin_cos().1, -half_sqrt);
}

#[test]
fn measure_1d_equals() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(m1 == m2);
    assert!(!(m1 == m3));

    let m4 = Measure::<Metre>::new(12.);
    let m5 = Measure::<Metre>::new(12.);
    let m6 = Measure::<Metre>::new(13.);
    assert!(m4 == m5);
    assert!(!(m4 == m6));
}

#[test]
fn measure_1d_differs() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(!(m1 != m2));
    assert!(m1 != m3);

    let m4 = Measure::<Metre>::new(12.);
    let m5 = Measure::<Metre>::new(12.);
    let m6 = Measure::<Metre>::new(13.);
    assert!(!(m4 != m5));
    assert!(m4 != m6);
}

#[test]
fn measure_1d_partial_cmp() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = Measure::<Metre>::new(12.);
    let m3 = Measure::<Metre>::new(13.);
    let m4 = Measure::<Metre>::new(f64::NAN);
    use core::cmp::Ordering;
    assert_eq!(m1.partial_cmp(&m2), Some(Ordering::Equal));
    assert_eq!(m1.partial_cmp(&m3), Some(Ordering::Less));
    assert_eq!(m3.partial_cmp(&m1), Some(Ordering::Greater));
    assert_eq!(m1.partial_cmp(&m4), None);
}

#[test]
fn measure_1d_is_equal_to_its_copy() {
    let m1 = Measure::<Metre>::new(12.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_1d_formatting() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{}", m), "12.25 m");
}

#[test]
fn measure_1d_formatting_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1}", m), "12.2 m");
}

#[test]
fn measure_1d_formatting_for_debug() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:?}", m), "12.25 m");
}

#[test]
fn measure_1d_formatting_for_debug_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", m), "12.2 m");
}

#[test]
fn measure_1d_formatting_for_lowerexp() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:e}", m), "1.225e1 m");
}

#[test]
fn measure_1d_formatting_for_lowerexp_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1e}", m), "1.2e1 m");
}

#[test]
fn measure_1d_formatting_for_upperexp() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:E}", m), "1.225E1 m");
}

#[test]
fn measure_1d_formatting_for_upperexp_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1E}", m), "1.2E1 m");
}

#[test]
fn measure_1d_formatting_in_decibels() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{}", m.decibels_formatter()), "10.881361 dB m");
}

#[test]
fn measure_1d_formatting_in_decibels_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1}", m.decibels_formatter()), "10.9 dB m");
}

#[test]
fn measure_1d_formatting_for_debug_in_decibels() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:?}", m.decibels_formatter()), "10.881361 dB m");
}

#[test]
fn measure_1d_formatting_for_debug_in_decibels_with_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", m.decibels_formatter()), "10.9 dB m");
}

#[test]
fn measure_1d_traits() {
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
    impl_common_traits::<Measure<Metre>>();
}
