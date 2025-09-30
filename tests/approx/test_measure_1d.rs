measures::define_measure_types! {
    exact with_approx,
    []
}

measures::measurement_vector_property! { Length }

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 0.001;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

#[test]
fn approx_measure_default() {
    let am: ApproxMeasure<Metre, f32> = ApproxMeasure::default();
    assert_eq!(am.value, 0.);
    assert_eq!(am.variance, 0.);
    let am = ApproxMeasure::<Metre>::default();
    assert_eq!(am.value, 0.);
    assert_eq!(am.variance, 0.);
}

#[test]
fn approx_measure_with_variance() {
    let am: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(12., 2.5);
    assert_eq!(am.value, 12.);
    assert_eq!(am.variance, 2.5);
}

#[test]
fn approx_measure_with_uncertainty() {
    let am: ApproxMeasure<Metre, f32> =
        ApproxMeasure::<Metre, f32>::with_uncertainty(12., Measure::<Metre, f32>::new(3.));
    assert_eq!(am.value, 12.);
    assert_eq!(am.variance, 9.);
    assert_eq!(am.uncertainty(), Measure::<Metre, f32>::new(3.));
}

#[test]
fn approx_measure_from_measure_with_variance() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    let am = ApproxMeasure::<Metre, f32>::from_measure_with_variance(m, 9.);
    assert_eq!(am.value, 12.);
    assert_eq!(am.variance, 9.);
}

#[test]
fn approx_measure_from_measure_with_uncertainty() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    let am = ApproxMeasure::<Metre, f32>::from_measure_with_uncertainty(
        m,
        Measure::<Metre, f32>::new(3.),
    );
    assert_eq!(am.value, 12.);
    assert_eq!(am.variance, 9.);
}

#[test]
fn approx_measure_to_measure() {
    let am: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(12., 2.5);
    let m: Measure<Metre, f32> = am.to_measure();
    assert_eq!(m.value, 12.);
}

#[test]
fn approx_measure_convert() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<MilliMetre, f32> = am1.convert::<MilliMetre>();
    assert_eq!(am1.value, 12.);
    assert_eq!(am1.variance, 9.);
    assert_eq!(am2.value, 12_000.);
    assert_eq!(am2.variance, 9_000_000.);
}

#[test]
fn approx_measure_lossless_into_32_to_32() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    #[allow(clippy::useless_conversion)]
    let am2: ApproxMeasure<Metre, f32> = am1.into();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossless_into_32_to_64() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f64> = am1.into();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossless_into_64_to_64() {
    let am1 = ApproxMeasure::<Metre, f64>::with_variance(12., 9.);
    #[allow(clippy::useless_conversion)]
    let am2: ApproxMeasure<Metre, f64> = am1.into();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossy_into_32_to_32() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.lossy_into::<f32>();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossy_into_32_to_64() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f64> = am1.lossy_into::<f64>();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossy_into_64_to_32() {
    let am1 = ApproxMeasure::<Metre, f64>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.lossy_into::<f32>();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_lossy_into_64_to_64() {
    let am1 = ApproxMeasure::<Metre, f64>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f64> = am1.lossy_into::<f64>();
    assert_eq!(am2.value, 12.);
    assert_eq!(am2.variance, 9.);
}

#[test]
fn approx_measure_squared_norm_positive() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<One, f32> = am1.squared_norm();
    assert_eq!(am2.value, 12. * 12.);
    assert_eq!(am2.variance, 4. * 12. * 12. * 9.);
}

#[test]
fn approx_measure_squared_norm_negative() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(-12., 9.);
    let am2: ApproxMeasure<One, f32> = am1.squared_norm();
    assert_eq!(am2.value, 12. * 12.);
    assert_eq!(am2.variance, 4. * 12. * 12. * 9.);
}

#[test]
fn approx_measure_squared_norm_zero() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(0., 9.);
    let am2: ApproxMeasure<One, f32> = am1.squared_norm();
    assert_eq!(am2.value, 0.);
    assert_eq!(am2.variance, 0.);
}

#[test]
fn approx_measure_normalized_positive() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.normalized();
    assert_eq!(am2.value, 1.);
    assert_eq!(am2.variance, 9. / (12. * 12.));
}

#[test]
fn approx_measure_normalized_negative() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(-12., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.normalized();
    assert_eq!(am2.value, -1.);
    assert_eq!(am2.variance, 9. / (12. * 12.));
}

#[test]
fn approx_measure_normalized_positive_zero() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(0., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.normalized();
    assert_eq!(am2.value, 1.);
    assert_eq!(am2.variance, 9. / (0. * 0.));
}

#[test]
fn approx_measure_normalized_negative_zero() {
    let am1 = ApproxMeasure::<Metre, f32>::with_variance(-0., 9.);
    let am2: ApproxMeasure<Metre, f32> = am1.normalized();
    assert_eq!(am2.value, -1.);
    assert_eq!(am2.variance, 9. / (0. * 0.));
}

#[test]
fn approx_measure_unary_minus() {
    let am: ApproxMeasure<Metre, f32> = -ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    assert_eq!(am.value, -12.);
    assert_eq!(am.variance, 9.);
}

#[test]
fn approx_measure_addition() {
    let m1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let m2 = ApproxMeasure::<Metre, f32>::with_variance(7., 4.);
    let m3: ApproxMeasure<Metre, f32> = m1 + m2;
    assert_eq!(m3.value, 19.);
    assert_eq!(m3.variance, 9. + 4.);
}

#[test]
fn approx_measure_addition_assignment() {
    let mut am = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    am += ApproxMeasure::<Metre, f32>::with_variance(7., 4.);
    assert_eq!(am.value, 19.);
    assert_eq!(am.variance, 9. + 4.);
}

#[test]
fn approx_measure_subtraction() {
    let m1 = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let m2 = ApproxMeasure::<Metre, f32>::with_variance(7., 4.);
    let m3: ApproxMeasure<Metre, f32> = m1 - m2;
    assert_eq!(m3.value, 5.);
    assert_eq!(m3.variance, 9. + 4.);
}

#[test]
fn approx_measure_subtraction_assignment() {
    let mut am = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    am -= ApproxMeasure::<Metre, f32>::with_variance(7., 4.);
    assert_eq!(am.value, 5.);
    assert_eq!(am.variance, 9. + 4.);
}

#[test]
fn approx_measure_scalar_multiplication_after_32() {
    let am: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(12., 9.) * 5.;
    assert_eq!(am.value, 12. * 5.);
    assert_eq!(am.variance, 9. * (5. * 5.));
}

#[test]
fn approx_measure_scalar_multiplication_after_64() {
    let am: ApproxMeasure<Metre, f64> = ApproxMeasure::<Metre, f64>::with_variance(12., 9.) * 5.;
    assert_eq!(am.value, 12. * 5.);
    assert_eq!(am.variance, 9. * (5. * 5.));
}

#[test]
fn approx_measure_scalar_multiplication_before_32() {
    let am: ApproxMeasure<Metre, f32> = 5. * ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    assert_eq!(am.value, 5. * 12.);
    assert_eq!(am.variance, (5. * 5.) * 9.);
}

#[test]
fn approx_measure_scalar_multiplication_before_64() {
    let am: ApproxMeasure<Metre, f64> = 5. * ApproxMeasure::<Metre, f64>::with_variance(12., 9.);
    assert_eq!(am.value, 5. * 12.);
    assert_eq!(am.variance, (5. * 5.) * 9.);
}

#[test]
fn approx_measure_scalar_multiplication_assignment() {
    let mut am = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    am *= 5.;
    assert_eq!(am.value, 12. * 5.);
    assert_eq!(am.variance, 9. * (5. * 5.));
}

#[test]
fn approx_measure_scalar_division() {
    let am: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(12., 9.) / 5.;
    assert_eq!(am.value, 12. / 5.);
    assert_eq!(am.variance, 9. / (5. * 5.));
}

#[test]
fn approx_measure_scalar_division_assignment() {
    let mut am = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    am /= 5.;
    assert_eq!(am.value, 12. / 5.);
    assert_eq!(am.variance, 9. / (5. * 5.));
}

#[test]
fn approx_measure_measure_division() {
    let am1: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(12., 9.);
    let am2: ApproxMeasure<Metre, f32> = ApproxMeasure::<Metre, f32>::with_variance(5., 4.);
    let am3: ApproxMeasure<One, f32> = am1 / am2;
    assert_eq!(am3.value, 2.4);
    let self_ratio = 9. / (12. * 12.);
    let other_ratio = 4. / (5. * 5.);
    let value_ratio = 12. / 5.;
    let quotient_variance = value_ratio * value_ratio * (self_ratio + other_ratio);
    assert_eq!(am3.variance, quotient_variance);
}

#[test]
fn approx_measure_equals() {
    let m1 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.4);
    let m2 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.4);
    let m3 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.41);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
}

#[test]
fn approx_measure_differ() {
    let m1 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.4);
    let m2 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.4);
    let m3 = ApproxMeasure::<Metre, f32>::with_variance(12.6, 0.41);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
}

//TODO CONTINUE
/*

#[test]
fn measure_partial_cmp() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    let m4 = Measure::<Metre, f32>::new(f32::NAN);
    use std::cmp::Ordering;
    assert_eq!(m1.partial_cmp(&m2), Some(Ordering::Equal));
    assert_eq!(m1.partial_cmp(&m3), Some(Ordering::Less));
    assert_eq!(m3.partial_cmp(&m1), Some(Ordering::Greater));
    assert_eq!(m1.partial_cmp(&m4), None);
}

#[test]
fn measure_is_equal_to_its_copy() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_formatting_in_metres() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{}", m), "12.25 m");
}

#[test]
fn measure_formatting_in_metres_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1}", m), "12.2 m");
}

#[test]
fn measure_formatting_for_debug_in_metres() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:?}", m), "12.25 m");
}

#[test]
fn measure_formatting_for_debug_in_metres_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", m), "12.2 m");
}
*/
