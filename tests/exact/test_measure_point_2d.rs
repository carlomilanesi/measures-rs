measures::define_measure_types! {
    with_points with_2d exact,
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

use measures::assert_eq_32;

#[test]
fn measure_point_2d_default() {
    let mp: MeasurePoint2d<Metre, f32> = MeasurePoint2d::default();
    assert_eq!(mp.values, [0., 0.]);
    let mp = MeasurePoint2d::<Metre>::default();
    assert_eq!(mp.values, [0., 0.]);
}

#[test]
fn measure_point_2d_new() {
    let m: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values, [12., 23.]);
}

#[test]
fn measure_point_2d_xy_functions() {
    let m: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mx: MeasurePoint<Metre, f32> = m.x();
    let my: MeasurePoint<Metre, f32> = m.y();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
}

#[test]
fn measure_point_2d_convert() {
    let m1: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m2: MeasurePoint2d<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.values, [12., 23.]);
    assert_eq!(m2.values, [12000., 23000.]);
}

#[test]
fn measure_point_2d_lossless_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint2d<Metre, f32> = mp1.into();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossless_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossless_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint2d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossy_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossy_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossy_into_64_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_lossy_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12., 23.]);
}

#[test]
fn measure_point_2d_addition_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    let mp2 = mp1 + m;
    assert_eq!(mp2.values, [12. + 34., 23. + -45.]);
}

#[test]
fn measure_point_2d_addition_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    mp += m;
    assert_eq!(mp.values, [12. + 34., 23. + -45.]);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    let mp2 = mp1 - m;
    assert_eq!(mp2.values, [12. - 34., 23. - -45.]);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    mp -= m;
    assert_eq!(mp.values, [12. - 34., 23. - -45.]);
}

#[test]
fn measures_point_2d_subtraction() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([34., -45.]);
    let m: Measure2d<Metre, f32> = mp1 - mp2;
    assert_eq!(m.values, [12. - 34., 23. - -45.]);
}

#[test]
fn measures_point_2d_weighted_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre, f32> = weighted_midpoint_2d(mp1, mp2, 0.4);
    assert_eq_32!(mp3.values, [26., -260.]);
}

#[test]
fn measures_point_2d_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre, f32> = midpoint_2d(mp1, mp2);
    assert_eq_32!(mp3.values, [25., -250.]);
}

#[test]
fn measures_point_2d_barycentric_combination() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3 = MeasurePoint2d::<Metre, f32>::new([80., -800.]);
    let mp4: MeasurePoint2d<Metre, f32> =
        barycentric_combination_2d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq_32!(mp4.values, [67., -670.]);
}

#[test]
fn measure_point_2d_equals() {
    let m1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m2 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m3 = MeasurePoint2d::<Metre, f32>::new([12., 23.2]);
    let m4 = MeasurePoint2d::<Metre, f32>::new([12.1, 23.]);
    let m5 = MeasurePoint2d::<Metre, f32>::new([12.1, 23.2]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
}

#[test]
fn measure_point_2d_differs() {
    let m1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m2 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m3 = MeasurePoint2d::<Metre, f32>::new([12., 23.2]);
    let m4 = MeasurePoint2d::<Metre, f32>::new([12.1, 23.]);
    let m5 = MeasurePoint2d::<Metre, f32>::new([12.1, 23.2]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
}

#[test]
fn measure_point_2d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 3.]);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_2d_formatting_in_metres() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.50203) m");
}

#[test]
fn measure_point_2d_formatting_in_metres_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5) m");
}

#[test]
fn measure_point_2d_formatting_for_debug_in_metres() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.50203) m");
}

#[test]
fn measure_point_2d_formatting_for_debug_in_metres_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5) m");
}
