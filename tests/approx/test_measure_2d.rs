measures::define_measure_types! {
    with_points with_directions with_2d exact,
    scalar_properties [ ]
    vector_properties [
        Length [
            Metre {
                suffix: " m",
            }
            MilliMetre {
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
    relationships [
    ]
}

use measures::{assert_eq_32, assert_eq_64};

#[test]
fn measure_2d_default() {
    let m: Measure2d<Metre, f32> = Measure2d::default();
    assert_eq!(m.values[0], 0.);
    assert_eq!(m.values[1], 0.);
    let m = Measure2d::<Metre>::default();
    assert_eq!(m.values[0], 0.);
    assert_eq!(m.values[1], 0.);
}

#[test]
fn measure_2d_new() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values[0], 12.);
    assert_eq!(m.values[1], 23.);
}

#[test]
fn measure_2d_xy_functions() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]);
    let mx: Measure<Metre, f32> = m.x();
    let my: Measure<Metre, f32> = m.y();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
}

#[test]
fn measure_2d_convert() {
    let m1: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.values[0], 12.);
    assert_eq!(m1.values[1], 23.);
    assert_eq!(m2.values[0], 12000.);
    assert_eq!(m2.values[1], 23000.);
}

#[test]
fn measure_2d_lossless_into_32_to_32() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure2d<Metre, f32> = m1.into();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossless_into_32_to_64() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.into();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossless_into_64_to_64() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure2d<Metre, f64> = m1.into();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossy_into_32_to_32() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossy_into_32_to_64() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossy_into_64_to_32() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    let m2: Measure2d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_lossy_into_64_to_64() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.values[0], 12.);
    assert_eq!(m2.values[1], 23.);
}

#[test]
fn measure_2d_squared_norm_positive() {
    let m = Measure2d::<Metre, f32>::new([12., 23.]);
    let n: f32 = m.squared_norm();
    assert_eq!(n, 12. * 12. + 23. * 23.);
}

#[test]
fn measure_2d_squared_norm_negative() {
    let m = Measure2d::<Metre, f64>::new([-12., -23.]);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 12. * 12. + 23. * 23.);
}

#[test]
fn measure_2d_squared_norm_zero() {
    let m = Measure2d::<Metre, f64>::new([0., 0.]);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 0.);
}

#[test]
fn measure_2d_normalized_positive() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.normalized();
    assert_eq_32!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
}

#[test]
fn measure_2d_normalized_x_negative() {
    let m1 = Measure2d::<Metre, f64>::new([-12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(
        m1.signed_direction::<Radian>().value,
        m2.signed_direction::<Radian>().value
    );
}

#[test]
fn measure_2d_normalized_y_negative() {
    let m1 = Measure2d::<Metre, f64>::new([12., -23.]);
    let m2: Measure2d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(
        m1.signed_direction::<Radian>().value,
        m2.signed_direction::<Radian>().value
    );
}

#[test]
fn measure_2d_normalized_xy_negative() {
    let m1 = Measure2d::<Metre, f64>::new([-12., -23.]);
    let m2: Measure2d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(
        m1.signed_direction::<Radian>().value,
        m2.signed_direction::<Radian>().value
    );
}

#[test]
fn measure_2d_normalized_zero() {
    let m1 = Measure2d::<Metre, f32>::new([0., 0.]);
    let m2: Measure2d<Metre, f32> = m1.normalized();
    assert!(m2.values[0].is_nan());
    assert!(m2.values[1].is_nan());
}

#[test]
fn measure_2d_from_direction_up() {
    let dir = MeasurePoint::<Degree, f32>::new(90.);
    println!("{}, {:?}.", dir, dir.value.sin_cos());
    let m = Measure2d::<Metre, f32>::from_direction(MeasurePoint::<Degree, f32>::new(90.));
    assert_eq_32!(m.values[0], 0.);
    assert_eq_32!(m.values[1], 1.);
}

#[test]
fn measure_2d_from_direction_down_left() {
    let m = Measure2d::<Metre, f32>::from_direction(MeasurePoint::<Degree, f32>::new(-135.));
    assert_eq_32!(m.values[0], -1f32 / 2f32.sqrt());
    assert_eq_32!(m.values[1], -1f32 / 2f32.sqrt());
}

#[test]
fn measure_2d_signed_direction_up() {
    let m = Measure2d::<Metre, f32>::new([0., 12.]);
    let signed_dir = m.signed_direction::<Degree>();
    assert_eq_32!(signed_dir.value, 90.);
}

#[test]
fn measure_2d_signed_direction_down_left() {
    let m = Measure2d::<Metre, f32>::new([-23., -23.]);
    let signed_dir = m.signed_direction::<Degree>();
    assert_eq_32!(signed_dir.value, -135.);
}

#[test]
fn measure_2d_unsigned_direction_up() {
    let m = Measure2d::<Metre, f32>::new([0., 12.]);
    let unsigned_dir = m.unsigned_direction::<Degree>();
    assert_eq_32!(unsigned_dir.value, 90.);
}

#[test]
fn measure_2d_unsigned_direction_down_left() {
    let m = Measure2d::<Metre, f32>::new([-23., -23.]);
    let unsigned_dir = m.unsigned_direction::<Degree>();
    assert_eq_32!(unsigned_dir.value, 225.);
}

#[test]
fn measure_2d_unary_minus() {
    let m = -Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values[0], -12.);
    assert_eq!(m.values[1], -23.);
}

#[test]
fn measure_2d_addition() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    let m3 = m1 + m2;
    assert_eq!(m1.values[0], 12.);
    assert_eq!(m1.values[1], 23.);
    assert_eq!(m2.values[0], 34.);
    assert_eq!(m2.values[1], -45.);
    assert_eq!(m3.values[0], 12. + 34.);
    assert_eq!(m3.values[1], 23. + -45.);
}

#[test]
fn measure_2d_addition_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    m1 += m2;
    assert_eq!(m1.values[0], 12. + 34.);
    assert_eq!(m1.values[1], 23. + -45.);
    assert_eq!(m2.values[0], 34.);
    assert_eq!(m2.values[1], -45.);
}

#[test]
fn measure_2d_subtraction() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    let m3 = m1 - m2;
    assert_eq!(m1.values[0], 12.);
    assert_eq!(m1.values[1], 23.);
    assert_eq!(m2.values[0], 34.);
    assert_eq!(m2.values[1], -45.);
    assert_eq!(m3.values[0], 12. - 34.);
    assert_eq!(m3.values[1], 23. - -45.);
}

#[test]
fn measure_2d_subtraction_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    m1 -= m2;
    assert_eq!(m1.values[0], 12. - 34.);
    assert_eq!(m1.values[1], 23. - -45.);
    assert_eq!(m2.values[0], 34.);
    assert_eq!(m2.values[1], -45.);
}

#[test]
fn measure_2d_scalar_multiplication_after_32() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]) * 3.;
    assert_eq!(m.values[0], 12. * 3.);
    assert_eq!(m.values[1], 23. * 3.);
}

#[test]
fn measure_2d_scalar_multiplication_after_64() {
    let m: Measure2d<Metre, f64> = Measure2d::<Metre, f64>::new([12., 23.]) * 3.;
    assert_eq!(m.values[0], 12. * 3.);
    assert_eq!(m.values[1], 23. * 3.);
}

#[test]
fn measure_2d_scalar_multiplication_before_32() {
    let m: Measure2d<Metre, f32> = 3. * Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values[0], 12. * 3.);
    assert_eq!(m.values[1], 23. * 3.);
}

#[test]
fn measure_2d_scalar_multiplication_before_64() {
    let m: Measure2d<Metre, f64> = 3. * Measure2d::<Metre, f64>::new([12., 23.]);
    assert_eq!(m.values[0], 12. * 3.);
    assert_eq!(m.values[1], 23. * 3.);
}

#[test]
fn measure_2d_scalar_multiplication_assignment() {
    let mut m = Measure2d::<Metre, f32>::new([12., 23.]);
    m *= 3.;
    assert_eq!(m.values[0], 12. * 3.);
    assert_eq!(m.values[1], 23. * 3.);
}

#[test]
fn measure_2d_scalar_division() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]) / 3.;
    assert_eq_32!(m.values[0], 12. / 3.);
    assert_eq_32!(m.values[1], 23. / 3.);
}

#[test]
fn measure_2d_scalar_division_assignment() {
    let mut m = Measure2d::<Metre, f32>::new([12., 23.]);
    m /= 3.;
    assert_eq_32!(m.values[0], 12. / 3.);
    assert_eq_32!(m.values[1], 23. / 3.);
}

#[test]
fn measure_2d_equals() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m3 = Measure2d::<Metre, f32>::new([12., 23.2]);
    let m4 = Measure2d::<Metre, f32>::new([12.1, 23.]);
    let m5 = Measure2d::<Metre, f32>::new([12.1, 23.2]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
}

#[test]
fn measure_2d_differs() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m3 = Measure2d::<Metre, f32>::new([12., 23.2]);
    let m4 = Measure2d::<Metre, f32>::new([12.1, 23.]);
    let m5 = Measure2d::<Metre, f32>::new([12.1, 23.2]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
}

#[test]
fn measure_2d_is_equal_to_its_copy() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_2d_formatting_in_metres() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5020]);
    assert_eq!(format!("{}", m), "(12.25, 23.502) m");
}

#[test]
fn measure_2d_formatting_in_metres_one_fractional_digit() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5498]);
    assert_eq!(format!("{:.1}", m), "(12.2, 23.5) m");
}

#[test]
fn measure_2d_formatting_for_debug_in_metres() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5020]);
    assert_eq!(format!("{:?}", m), "(12.25, 23.502) m");
}

#[test]
fn measure_2d_formatting_in_metres_for_debug_one_fractional_digit() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5498]);
    assert_eq!(format!("{:.1?}", m), "(12.2, 23.5) m");
}
