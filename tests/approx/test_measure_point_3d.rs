measures::define_measure_types! {
    with_points with_3d exact,
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
    dimensionless_measurement_units [ ]
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
fn measure_point_3d_default() {
    let mp: MeasurePoint3d<Metre, f32> = MeasurePoint3d::default();
    assert_eq!(mp.values, [0., 0., 0.]);
    let mp = MeasurePoint3d::<Metre>::default();
    assert_eq!(mp.values, [0., 0., 0.]);
}

#[test]
fn measure_point_3d_new() {
    let m: MeasurePoint3d<Metre, f32> = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    assert_eq!(m.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_xy_functions() {
    let m: MeasurePoint3d<Metre, f32> = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mx: MeasurePoint<Metre, f32> = m.x();
    let my: MeasurePoint<Metre, f32> = m.y();
    let mz: MeasurePoint<Metre, f32> = m.z();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
    assert_eq!(mz.value, 34.);
}

#[test]
fn measure_point_3d_convert() {
    let m1: MeasurePoint3d<Metre, f32> = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: MeasurePoint3d<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.values, [12., 23., 34.]);
    assert_eq!(m2.values, [12000., 23000., 34000.]);
}

#[test]
fn measure_point_3d_lossless_into_32_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint3d<Metre, f32> = mp1.into();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossless_into_32_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossless_into_64_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint3d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossy_into_32_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossy_into_32_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossy_into_64_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_lossy_into_64_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12., 23., 34.]);
}

#[test]
fn measure_point_3d_addition_of_measure_3d() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    let mp2 = mp1 + m;
    assert_eq!(mp2.values, [12. + 45., 23. + -56., 34. + 67.]);
}

#[test]
fn measure_point_3d_addition_of_measure_3d_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    mp += m;
    assert_eq!(mp.values, [12. + 45., 23. + -56., 34. + 67.]);
}

#[test]
fn measure_point_3d_subtraction_of_measure_3d() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    let mp2 = mp1 - m;
    assert_eq!(mp2.values, [12. - 45., 23. - -56., 34. - 67.]);
}

#[test]
fn measure_point_3d_subtraction_of_measure_3d_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    mp -= m;
    assert_eq!(mp.values, [12. - 45., 23. - -56., 34. - 67.]);
}

#[test]
fn measures_point_3d_subtraction() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([45., -56., 67.]);
    let m: Measure3d<Metre, f32> = mp1 - mp2;
    assert_eq!(m.values, [12. - 45., 23. - -56., 34. - 67.]);
}

#[test]
fn measures_point_3d_weighted_midpoint() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre, f32> = weighted_midpoint_3d(mp1, mp2, 0.4);
    assert_eq_32!(mp3.values, [26., -260., 78.]);
}

#[test]
fn measures_point_3d_midpoint() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre, f32> = midpoint_3d(mp1, mp2);
    assert_eq_32!(mp3.values, [25., -250., 75.]);
}

#[test]
fn measures_point_3d_barycentric_combination() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3 = MeasurePoint3d::<Metre, f32>::new([80., -800., 240.]);
    let mp4: MeasurePoint3d<Metre, f32> =
        barycentric_combination_3d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq_32!(mp4.values, [67., -670., 201.]);
}

#[test]
fn measure_point_3d_equals() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m4 = MeasurePoint3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m5 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.3]);
    let m6 = MeasurePoint3d::<Metre, f32>::new([12.1, 23.2, 34.3]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
    assert!(!(m1 == m6));
}

#[test]
fn measure_point_3d_differs() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m4 = MeasurePoint3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m5 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.3]);
    let m6 = MeasurePoint3d::<Metre, f32>::new([12.1, 23.2, 34.3]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
    assert!(m1 != m6);
}

#[test]
fn measure_point_3d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., -34.]);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_3d_formatting_in_metres() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.5, 34.75) m");
}

#[test]
fn measure_point_3d_formatting_in_metres_one_fractional_digit() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5, 34.8) m");
}

#[test]
fn measure_point_3d_formatting_for_debug_in_metres() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.5, 34.75) m");
}

#[test]
fn measure_point_3d_formatting_for_debug_in_metres_one_fractional_digit() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5, 34.8) m");
}
