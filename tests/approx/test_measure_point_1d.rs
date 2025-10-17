use units::{ApproxMeasurePoint, Celsius, Fahrenheit};

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
    }
}

#[test]
fn approx_measure_point_default() {
    let mp: ApproxMeasurePoint<Celsius, f32> = ApproxMeasurePoint::default();
    assert_eq!(mp.value, 0.);
    assert_eq!(mp.variance, 0.);
    let mp = ApproxMeasurePoint::<Celsius>::default();
    assert_eq!(mp.value, 0.);
    assert_eq!(mp.variance, 0.);
}

#[test]
fn approx_measure_point_new() {
    let mp = ApproxMeasurePoint::<Celsius, f32>::with_variance(12., 0.04);
    assert_eq!(mp.value, 12.);
    assert_eq!(mp.variance, 0.04);
}

#[test]
fn approx_measure_point_convert() {
    // 0 ± 0.2 °C is 32 ± 0.36 °F
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::with_variance(0., 0.2 * 0.2);
    let mp2: ApproxMeasurePoint<Fahrenheit, f32> = mp1.convert::<Fahrenheit>();
    assert_eq!(mp2.value, 32.);
    assert_eq!(mp2.variance, 0.2 * 0.2 * (9. / 5.) * (9. / 5.));
    assert_eq!(mp2.variance, 0.36 * 0.36);

    // 68 ± 0.36 °F is 20 ± 0.2 °C
    let mp3 = ApproxMeasurePoint::<Fahrenheit, f32>::with_variance(68., 0.36 * 0.36);
    let mp4: ApproxMeasurePoint<Celsius, f32> = mp3.convert::<Celsius>();
    assert_eq!(mp4.value, 20.);
    assert_eq!(mp4.variance, 0.36 * 0.36 / (9. / 5.) / (9. / 5.));
    assert_eq!(mp4.variance, 0.2 * 0.2);
}

/* TODO
#[test]
fn approx_measure_point_lossless_into_32_to_32() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f32> = mp1.into();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossless_into_32_to_64() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f64> = mp1.into();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossless_into_64_to_64() {
    let mp1 = ApproxMeasurePoint::<Celsius, f64>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f64> = mp1.into();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossy_into_32_to_32() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossy_into_32_to_64() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossy_into_64_to_32() {
    let mp1 = ApproxMeasurePoint::<Celsius, f64>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_lossy_into_64_to_64() {
    let mp1 = ApproxMeasurePoint::<Celsius, f64>::new(12.);
    let mp2: ApproxMeasurePoint<Celsius, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn approx_measure_point_addition_of_measure() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let m = ApproxMeasure::<Celsius, f32>::new(7.);
    let mp2: ApproxMeasurePoint<Celsius, f32> = mp1 + m;
    assert_eq!(mp2.value, 19.);
}

#[test]
fn approx_measure_point_addition_of_measure_assignment() {
    let mut mp = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    mp += ApproxMeasure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 19.);
}

#[test]
fn approx_measure_point_subtraction_of_measure() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let m = ApproxMeasure::<Celsius, f32>::new(7.);
    let mp2: ApproxMeasurePoint<Celsius, f32> = mp1 - m;
    assert_eq!(mp2.value, 5.);
}

#[test]
fn approx_measure_point_subtraction_of_measure_assignment() {
    let mut mp = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    mp -= ApproxMeasure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 5.);
}

#[test]
fn approx_measures_point_subtraction() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(7.);
    let m: Measure<Celsius, f32> = mp1 - mp2;
    assert_eq!(m.value, 5.);
}

#[test]
fn approx_measures_point_weighted_midpoint() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(30.);
    let mp3: ApproxMeasurePoint<Celsius, f32> = weighted_midpoint(mp1, mp2, 0.4);
    assert_eq_32!(mp3.value, 26.);
}

#[test]
fn approx_measures_point_midpoint() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(30.);
    let mp3: ApproxMeasurePoint<Celsius, f32> = midpoint(mp1, mp2);
    assert_eq_32!(mp3.value, 25.);
}

#[test]
fn approx_measures_point_barycentric_combination() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(30.);
    let mp3 = ApproxMeasurePoint::<Celsius, f32>::new(80.);
    let mp4: ApproxMeasurePoint<Celsius, f32> =
        barycentric_combination(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq_32!(mp4.value, 67.);
}

#[test]
fn approx_measure_point_equals() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = ApproxMeasurePoint::<Celsius, f32>::new(13.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));
}

#[test]
fn approx_measure_point_differs() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = ApproxMeasurePoint::<Celsius, f32>::new(13.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn approx_measure_point_partial_cmp() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = ApproxMeasurePoint::<Celsius, f32>::new(13.);
    let mp4 = ApproxMeasurePoint::<Celsius, f32>::new(f32::NAN);
    use core::cmp::Ordering;
    assert_eq!(mp1.partial_cmp(&mp2), Some(Ordering::Equal));
    assert_eq!(mp1.partial_cmp(&mp3), Some(Ordering::Less));
    assert_eq!(mp3.partial_cmp(&mp1), Some(Ordering::Greater));
    assert_eq!(mp1.partial_cmp(&mp4), None);
}

#[test]
fn approx_measure_point_is_equal_to_its_copy() {
    let mp1 = ApproxMeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn approx_measure_point_formatting_in_celsius() {
    let mp = ApproxMeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{}", mp), "at 12.25 °C");
}

#[test]
fn approx_measure_point_formatting_in_celsius_one_fractional_digit() {
    let mp = ApproxMeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1}", mp), "at 12.2 °C");
}

#[test]
fn approx_measure_point_formatting_for_debug_in_celsius() {
    let mp = ApproxMeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:?}", mp), "at 12.25 °C");
}

#[test]
fn approx_measure_point_formatting_for_debug_in_celsius_one_fractional_digit() {
    let mp = ApproxMeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", mp), "at 12.2 °C");
}
*/
