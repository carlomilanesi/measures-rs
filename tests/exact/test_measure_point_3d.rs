use measures::{assert_eq_32, assert_eq_64};
use units::{
    barycentric_combination_3d, midpoint_3d, weighted_midpoint_3d, ApproxMeasurePoint3d, Measure3d,
    MeasurePoint, MeasurePoint3d, Metre, Millimetre,
};

mod units {
    measures::define_measure_types! {
        with_points with_3d exact with_approx,
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
fn measure_point_3d_new() {
    let m1: MeasurePoint3d<Metre, f32> = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);

    let m1: MeasurePoint3d<Metre> = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_point_3d_convert() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: MeasurePoint3d<Millimetre, f32> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12000_f32, 23000_f32, 34000_f32]);

    let m1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m2: MeasurePoint3d<Millimetre> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12000_f64, 23000_f64, 34000_f64]);
}

#[test]
fn measure_point_3d_xy_functions() {
    let m: MeasurePoint3d<Metre, f32> = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mx: MeasurePoint<Metre, f32> = m.x();
    let my: MeasurePoint<Metre, f32> = m.y();
    let mz: MeasurePoint<Metre, f32> = m.z();
    assert_eq!(mx.value, 12_f32);
    assert_eq!(my.value, 23_f32);
    assert_eq!(mz.value, 34_f32);

    let m: MeasurePoint3d<Metre> = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let mx: MeasurePoint<Metre> = m.x();
    let my: MeasurePoint<Metre> = m.y();
    let mz: MeasurePoint<Metre> = m.z();
    assert_eq!(mx.value, 12_f64);
    assert_eq!(my.value, 23_f64);
    assert_eq!(mz.value, 34_f64);
}

#[test]
fn measure_point_3d_lossless_into_32_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint3d<Metre, f32> = mp1.into();
    assert_eq!(mp2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_point_3d_lossless_into_32_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_point_3d_lossless_into_64_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint3d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12_f64, 23_f64, 34_f64]);
}

/*
// ILLEGAL
#[test]
fn measure_point_3d_lossless_into_64_to_32() {
    let m1 = MeasurePoint3d::<Metre, f64>::new(12.);
    let m2: MeasurePoint3d<Metre, f32> = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_point_3d_lossy_into_32_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_point_3d_lossy_into_32_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_point_3d_lossy_into_64_to_32() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_point_3d_lossy_into_64_to_64() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new([12., 23., 34.]);
    let mp2: MeasurePoint3d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_point_3d_default() {
    let mp: MeasurePoint3d<Metre, f32> = MeasurePoint3d::default();
    assert_eq!(mp.values, [0_f32, 0_f32, 0_f32]);

    let mp = MeasurePoint3d::<Metre>::default();
    assert_eq!(mp.values, [0_f64, 0_f64, 0_f64]);
}

#[test]
fn measure_point_3d_from_three_measure_points() {
    let three_measure_points = [
        MeasurePoint::<Metre, f32>::new(12.),
        MeasurePoint::<Metre, f32>::new(23.),
        MeasurePoint::<Metre, f32>::new(34.),
    ];

    let m1 = MeasurePoint3d::<Metre, f32>::from(three_measure_points);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);

    let m2: MeasurePoint3d<Metre, f32> = three_measure_points.into();
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_from_f32_into_f64() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre, f64>::from(m1);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
    let m3: MeasurePoint3d<Metre, f64> = m1.into();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m3.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_from_approx_into_measure() {
    let am = ApproxMeasurePoint3d::<Metre, f32>::with_covariances(
        [12., 23., 34.],
        [[0.4, 0.04, 0.01], [0.1, 0.9, 0.02], [0.1, 0.03, 0.9]],
    );
    let m1 = MeasurePoint3d::<Metre, f32>::from(am);
    assert_eq!(am.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    let m2: MeasurePoint3d<Metre, f32> = am.into();
    assert_eq!(am.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);

    let am = ApproxMeasurePoint3d::<Metre>::with_covariances(
        [12., 23., 34.],
        [[0.4, 0.04, 0.01], [0.1, 0.9, 0.02], [0.1, 0.03, 0.9]],
    );
    let m1 = MeasurePoint3d::<Metre>::from(am);
    assert_eq!(am.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    let m2: MeasurePoint3d<Metre> = am.into();
    assert_eq!(am.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_point_3d_addition_of_measure_3d() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([34., -45., 67.]);
    let mp2: MeasurePoint3d<Metre, f32> = mp1 + m;
    assert_eq!(
        mp2.values,
        [12_f32 + 34_f32, 23_f32 + -45_f32, 34_f32 + 67_f32]
    );

    let mp1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m = Measure3d::<Metre>::new([34., -45., 67.]);
    let mp2: MeasurePoint3d<Metre> = mp1 + m;
    assert_eq!(
        mp2.values,
        [12_f64 + 34_f64, 23_f64 + -45_f64, 34_f64 + 67_f64]
    );
}

#[test]
fn measure_point_3d_addition_of_measure_3d_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([34., -45., 67.]);
    mp += m;
    assert_eq!(m.values, [34_f32, -45_f32, 67_f32]);
    assert_eq!(
        mp.values,
        [12_f32 + 34_f32, 23_f32 + -45_f32, 34_f32 + 67_f32]
    );

    let mut mp = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m = Measure3d::<Metre>::new([34., -45., 67.]);
    mp += m;
    assert_eq!(m.values, [34_f64, -45_f64, 67_f64]);
    assert_eq!(
        mp.values,
        [12_f64 + 34_f64, 23_f64 + -45_f64, 34_f64 + 67_f64]
    );
}

#[test]
fn measure_point_3d_subtraction_of_measure_3d() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([34., -45., 67.]);
    let mp2 = mp1 - m;
    assert_eq!(mp1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m.values, [34_f32, -45_f32, 67_f32]);
    assert_eq!(
        mp2.values,
        [12_f32 - 34_f32, 23_f32 - -45_f32, 34_f32 - 67_f32]
    );

    let mp1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m = Measure3d::<Metre>::new([34., -45., 67.]);
    let mp2 = mp1 - m;
    assert_eq!(mp1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m.values, [34_f64, -45_f64, 67_f64]);
    assert_eq!(
        mp2.values,
        [12_f64 - 34_f64, 23_f64 - -45_f64, 34_f64 - 67_f64]
    );
}

#[test]
fn measure_point_3d_subtraction_of_measure_3d_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m = Measure3d::<Metre, f32>::new([34., -45., 67.]);
    mp -= m;
    assert_eq!(m.values, [34_f32, -45_f32, 67_f32]);
    assert_eq!(
        mp.values,
        [12_f32 - 34_f32, 23_f32 - -45_f32, 34_f32 - 67_f32]
    );

    let mut mp = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m = Measure3d::<Metre>::new([34., -45., 67.]);
    mp -= m;
    assert_eq!(m.values, [34_f64, -45_f64, 67_f64]);
    assert_eq!(
        mp.values,
        [12_f64 - 34_f64, 23_f64 - -45_f64, 34_f64 - 67_f64]
    );
}

#[test]
fn measures_point_3d_subtraction() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([34., -45., 67.]);
    let m: Measure3d<Metre, f32> = mp1 - mp2;
    assert_eq!(mp1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(mp2.values, [34_f32, -45_f32, 67_f32]);
    assert_eq!(
        m.values,
        [12_f32 - 34_f32, 23_f32 - -45_f32, 34_f32 - 67_f32]
    );

    let mp1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let mp2 = MeasurePoint3d::<Metre>::new([34., -45., 67.]);
    let m: Measure3d<Metre> = mp1 - mp2;
    assert_eq!(mp1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(mp2.values, [34_f64, -45_f64, 67_f64]);
    assert_eq!(
        m.values,
        [12_f64 - 34_f64, 23_f64 - -45_f64, 34_f64 - 67_f64]
    );
}

#[test]
fn measures_point_3d_weighted_midpoint() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre, f32> = weighted_midpoint_3d(mp1, mp2, 0.4);
    assert_eq!(mp1.values, [20_f32, -200_f32, 60_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32, 90_f32]);
    assert_eq_32!(mp3.values, [26_f32, -260_f32, 78_f32]);

    let mp1 = MeasurePoint3d::<Metre>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre> = weighted_midpoint_3d(mp1, mp2, 0.4);
    assert_eq!(mp1.values, [20_f64, -200_f64, 60_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64, 90_f64]);
    assert_eq_64!(mp3.values, [26_f64, -260_f64, 78_f64]);
}

#[test]
fn measures_point_3d_midpoint() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre, f32> = midpoint_3d(mp1, mp2);
    assert_eq!(mp1.values, [20_f32, -200_f32, 60_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32, 90_f32]);
    assert_eq_32!(mp3.values, [25_f32, -250_f32, 75_f32]);

    let mp1 = MeasurePoint3d::<Metre>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre>::new([30., -300., 90.]);
    let mp3: MeasurePoint3d<Metre> = midpoint_3d(mp1, mp2);
    assert_eq!(mp1.values, [20_f64, -200_f64, 60_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64, 90_f64]);
    assert_eq_64!(mp3.values, [25_f64, -250_f64, 75_f64]);
}

#[test]
fn measures_point_3d_barycentric_combination() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre, f32>::new([30., -300., 90.]);
    let mp3 = MeasurePoint3d::<Metre, f32>::new([80., -800., 240.]);
    let mp4: MeasurePoint3d<Metre, f32> =
        barycentric_combination_3d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq!(mp1.values, [20_f32, -200_f32, 60_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32, 90_f32]);
    assert_eq!(mp3.values, [80_f32, -800_f32, 240_f32]);
    assert_eq_32!(mp4.values, [67_f32, -670_f32, 201_f32]);

    let mp1 = MeasurePoint3d::<Metre>::new([20., -200., 60.]);
    let mp2 = MeasurePoint3d::<Metre>::new([30., -300., 90.]);
    let mp3 = MeasurePoint3d::<Metre>::new([80., -800., 240.]);
    let mp4: MeasurePoint3d<Metre> = barycentric_combination_3d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq!(mp1.values, [20_f64, -200_f64, 60_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64, 90_f64]);
    assert_eq!(mp3.values, [80_f64, -800_f64, 240_f64]);
    assert_eq_64!(mp4.values, [67_f64, -670_f64, 201_f64]);
}

#[test]
fn measure_point_3d_equals() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m4 = MeasurePoint3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m5 = MeasurePoint3d::<Metre, f32>::new([12.1, 23.2, 34.]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));

    let m1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre>::new([12., 23.2, 34.]);
    let m4 = MeasurePoint3d::<Metre>::new([12.1, 23., 34.]);
    let m5 = MeasurePoint3d::<Metre>::new([12.1, 23.2, 34.]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
}

#[test]
fn measure_point_3d_differs() {
    let m1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m4 = MeasurePoint3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m5 = MeasurePoint3d::<Metre, f32>::new([12.1, 23.2, 34.]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);

    let m1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m2 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let m3 = MeasurePoint3d::<Metre>::new([12., 23.2, 34.]);
    let m4 = MeasurePoint3d::<Metre>::new([12.1, 23., 34.]);
    let m5 = MeasurePoint3d::<Metre>::new([12.1, 23.2, 34.]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
}

#[test]
fn measure_point_3d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new([12., 23., 34.]);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);

    let mp1 = MeasurePoint3d::<Metre>::new([12., 23., 34.]);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_3d_formatting() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.50203, 34.75) m");

    let mp = MeasurePoint3d::<Metre>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.50203, 34.75) m");
}

#[test]
fn measure_point_3d_formatting_one_fractional_digit() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5, 34.8) m");

    let mp = MeasurePoint3d::<Metre>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5, 34.8) m");
}

#[test]
fn measure_point_3d_formatting_for_debug() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.50203, 34.75) m");

    let mp = MeasurePoint3d::<Metre>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.50203, 34.75) m");
}

#[test]
fn measure_point_3d_formatting_for_debug_one_fractional_digit() {
    let mp = MeasurePoint3d::<Metre, f32>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5, 34.8) m");

    let mp = MeasurePoint3d::<Metre>::new([12.25, 23.50203, 34.75]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5, 34.8) m");
}

#[test]
fn measure_point_3d_traits() {
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
            + Unpin,
    >() {
    }
    impl_common_traits::<MeasurePoint3d<Metre>>();
}
