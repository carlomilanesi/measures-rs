use measures::{assert_eq_32, assert_eq_64};
use units::{
    barycentric_combination_2d, midpoint_2d, weighted_midpoint_2d, ApproxMeasurePoint2d, Measure2d,
    MeasurePoint, MeasurePoint2d, Metre, Millimetre,
};

mod units {
    measures::define_measure_types! {
        with_points with_2d exact with_approx,
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
fn measure_point_2d_new() {
    let m1: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m1.values, [12_f32, 23_f32]);

    let m1: MeasurePoint2d<Metre> = MeasurePoint2d::<Metre>::new([12., 23.]);
    assert_eq!(m1.values, [12_f64, 23_f64]);
}

#[test]
fn measure_point_2d_convert() {
    let m1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m2: MeasurePoint2d<Millimetre, f32> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12000_f32, 23000_f32]);

    let m1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m2: MeasurePoint2d<Millimetre> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12000_f64, 23000_f64]);
}

#[test]
fn measure_point_2d_xy_functions() {
    let m: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mx: MeasurePoint<Metre, f32> = m.x();
    let my: MeasurePoint<Metre, f32> = m.y();
    assert_eq!(mx.value, 12_f32);
    assert_eq!(my.value, 23_f32);

    let m: MeasurePoint2d<Metre> = MeasurePoint2d::<Metre>::new([12., 23.]);
    let mx: MeasurePoint<Metre> = m.x();
    let my: MeasurePoint<Metre> = m.y();
    assert_eq!(mx.value, 12_f64);
    assert_eq!(my.value, 23_f64);
}

#[test]
fn measure_point_2d_lossless_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint2d<Metre, f32> = mp1.into();
    assert_eq!(mp2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_point_2d_lossless_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_point_2d_lossless_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let mp2: MeasurePoint2d<Metre, f64> = mp1.into();
    assert_eq!(mp2.values, [12_f64, 23_f64]);
}

/*
// ILLEGAL
#[test]
fn measure_point_2d_lossless_into_64_to_32() {
    let m1 = MeasurePoint2d::<Metre, f64>::new(12.);
    let m2: MeasurePoint2d<Metre, f32> = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_point_2d_lossy_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_point_2d_lossy_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_point_2d_lossy_into_64_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_point_2d_lossy_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new([12., 23.]);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_point_2d_default() {
    let mp: MeasurePoint2d<Metre, f32> = MeasurePoint2d::default();
    assert_eq!(mp.values, [0_f32, 0_f32]);

    let mp = MeasurePoint2d::<Metre>::default();
    assert_eq!(mp.values, [0_f64, 0_f64]);
}

#[test]
fn measure_point_2d_from_two_measure_points() {
    let two_measure_points = [
        MeasurePoint::<Metre, f32>::new(12.),
        MeasurePoint::<Metre, f32>::new(23.),
    ];

    let m1 = MeasurePoint2d::<Metre, f32>::from(two_measure_points);
    assert_eq!(m1.values, [12_f32, 23_f32]);

    let m2: MeasurePoint2d<Metre, f32> = two_measure_points.into();
    assert_eq!(m2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_from_f32_into_f64() {
    let m1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m2 = MeasurePoint2d::<Metre, f64>::from(m1);
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
    let m3: MeasurePoint2d<Metre, f64> = m1.into();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m3.values, [12_f64, 23_f64]);
}

#[test]
fn measure_from_approx_into_measure() {
    let am =
        ApproxMeasurePoint2d::<Metre, f32>::with_covariances([12., 23.], [[0.4, 0.04], [0.1, 0.9]]);
    let m1 = MeasurePoint2d::<Metre, f32>::from(am);
    assert_eq!(am.values, [12_f32, 23_f32]);
    assert_eq!(m1.values, [12_f32, 23_f32]);
    let m2: MeasurePoint2d<Metre, f32> = am.into();
    assert_eq!(am.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32]);

    let am = ApproxMeasurePoint2d::<Metre>::with_covariances([12., 23.], [[0.4, 0.04], [0.1, 0.9]]);
    let m1 = MeasurePoint2d::<Metre>::from(am);
    assert_eq!(am.values, [12_f64, 23_f64]);
    assert_eq!(m1.values, [12_f64, 23_f64]);
    let m2: MeasurePoint2d<Metre> = am.into();
    assert_eq!(am.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_point_2d_addition_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    let mp2: MeasurePoint2d<Metre, f32> = mp1 + m;
    assert_eq!(mp2.values, [12_f32 + 34_f32, 23_f32 + -45_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m = Measure2d::<Metre>::new([34., -45.]);
    let mp2: MeasurePoint2d<Metre> = mp1 + m;
    assert_eq!(mp2.values, [12_f64 + 34_f64, 23_f64 + -45_f64]);
}

#[test]
fn measure_point_2d_addition_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    mp += m;
    assert_eq!(m.values, [34_f32, -45_f32]);
    assert_eq!(mp.values, [12_f32 + 34_f32, 23_f32 + -45_f32]);

    let mut mp = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m = Measure2d::<Metre>::new([34., -45.]);
    mp += m;
    assert_eq!(m.values, [34_f64, -45_f64]);
    assert_eq!(mp.values, [12_f64 + 34_f64, 23_f64 + -45_f64]);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    let mp2 = mp1 - m;
    assert_eq!(mp1.values, [12_f32, 23_f32]);
    assert_eq!(m.values, [34_f32, -45_f32]);
    assert_eq!(mp2.values, [12_f32 - 34_f32, 23_f32 - -45_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m = Measure2d::<Metre>::new([34., -45.]);
    let mp2 = mp1 - m;
    assert_eq!(mp1.values, [12_f64, 23_f64]);
    assert_eq!(m.values, [34_f64, -45_f64]);
    assert_eq!(mp2.values, [12_f64 - 34_f64, 23_f64 - -45_f64]);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let m = Measure2d::<Metre, f32>::new([34., -45.]);
    mp -= m;
    assert_eq!(m.values, [34_f32, -45_f32]);
    assert_eq!(mp.values, [12_f32 - 34_f32, 23_f32 - -45_f32]);

    let mut mp = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m = Measure2d::<Metre>::new([34., -45.]);
    mp -= m;
    assert_eq!(m.values, [34_f64, -45_f64]);
    assert_eq!(mp.values, [12_f64 - 34_f64, 23_f64 - -45_f64]);
}

#[test]
fn measures_point_2d_subtraction() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([12., 23.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([34., -45.]);
    let m: Measure2d<Metre, f32> = mp1 - mp2;
    assert_eq!(mp1.values, [12_f32, 23_f32]);
    assert_eq!(mp2.values, [34_f32, -45_f32]);
    assert_eq!(m.values, [12_f32 - 34_f32, 23_f32 - -45_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let mp2 = MeasurePoint2d::<Metre>::new([34., -45.]);
    let m: Measure2d<Metre> = mp1 - mp2;
    assert_eq!(mp1.values, [12_f64, 23_f64]);
    assert_eq!(mp2.values, [34_f64, -45_f64]);
    assert_eq!(m.values, [12_f64 - 34_f64, 23_f64 - -45_f64]);
}

#[test]
fn measures_point_2d_weighted_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre, f32> = weighted_midpoint_2d(mp1, mp2, 0.4);
    assert_eq!(mp1.values, [20_f32, -200_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32]);
    assert_eq_32!(mp3.values, [26_f32, -260_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre> = weighted_midpoint_2d(mp1, mp2, 0.4);
    assert_eq!(mp1.values, [20_f64, -200_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64]);
    assert_eq_32!(mp3.values, [26_f64, -260_f64]);
}

#[test]
fn measures_point_2d_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre, f32> = midpoint_2d(mp1, mp2);
    assert_eq!(mp1.values, [20_f32, -200_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32]);
    assert_eq_32!(mp3.values, [25_f32, -250_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre>::new([30., -300.]);
    let mp3: MeasurePoint2d<Metre> = midpoint_2d(mp1, mp2);
    assert_eq!(mp1.values, [20_f64, -200_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64]);
    assert_eq_64!(mp3.values, [25_f64, -250_f64]);
}

#[test]
fn measures_point_2d_barycentric_combination() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre, f32>::new([30., -300.]);
    let mp3 = MeasurePoint2d::<Metre, f32>::new([80., -800.]);
    let mp4: MeasurePoint2d<Metre, f32> =
        barycentric_combination_2d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq!(mp1.values, [20_f32, -200_f32]);
    assert_eq!(mp2.values, [30_f32, -300_f32]);
    assert_eq!(mp3.values, [80_f32, -800_f32]);
    assert_eq_32!(mp4.values, [67_f32, -670_f32]);

    let mp1 = MeasurePoint2d::<Metre>::new([20., -200.]);
    let mp2 = MeasurePoint2d::<Metre>::new([30., -300.]);
    let mp3 = MeasurePoint2d::<Metre>::new([80., -800.]);
    let mp4: MeasurePoint2d<Metre> = barycentric_combination_2d(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq!(mp1.values, [20_f64, -200_f64]);
    assert_eq!(mp2.values, [30_f64, -300_f64]);
    assert_eq!(mp3.values, [80_f64, -800_f64]);
    assert_eq_64!(mp4.values, [67_f64, -670_f64]);
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

    let m1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m2 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m3 = MeasurePoint2d::<Metre>::new([12., 23.2]);
    let m4 = MeasurePoint2d::<Metre>::new([12.1, 23.]);
    let m5 = MeasurePoint2d::<Metre>::new([12.1, 23.2]);
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

    let m1 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m2 = MeasurePoint2d::<Metre>::new([12., 23.]);
    let m3 = MeasurePoint2d::<Metre>::new([12., 23.2]);
    let m4 = MeasurePoint2d::<Metre>::new([12.1, 23.]);
    let m5 = MeasurePoint2d::<Metre>::new([12.1, 23.2]);
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

    let mp1 = MeasurePoint2d::<Metre>::new([12., 3.]);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_2d_formatting() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.50203) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{}", mp), "at (12.25, 23.50203) m");
}

#[test]
fn measure_point_2d_formatting_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1}", mp), "at (12.2, 23.5) m");
}

#[test]
fn measure_point_2d_formatting_for_debug() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.50203) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:?}", mp), "at (12.25, 23.50203) m");
}

#[test]
fn measure_point_2d_formatting_for_debug_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1?}", mp), "at (12.2, 23.5) m");
}

#[test]
fn measure_point_2d_formatting_for_lowerexp() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:e}", mp), "at (1.225e1, 2.350203e1) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:e}", mp), "at (1.225e1, 2.350203e1) m");
}

#[test]
fn measure_point_2d_formatting_for_lowerexp_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1e}", mp), "at (1.2e1, 2.4e1) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1e}", mp), "at (1.2e1, 2.4e1) m");
}

#[test]
fn measure_point_2d_formatting_for_upperexp() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:E}", mp), "at (1.225E1, 2.350203E1) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:E}", mp), "at (1.225E1, 2.350203E1) m");
}

#[test]
fn measure_point_2d_formatting_for_upperexp_one_fractional_digit() {
    let mp = MeasurePoint2d::<Metre, f32>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1E}", mp), "at (1.2E1, 2.4E1) m");

    let mp = MeasurePoint2d::<Metre>::new([12.25, 23.50203]);
    assert_eq!(format!("{:.1E}", mp), "at (1.2E1, 2.4E1) m");
}

#[test]
fn measure_point_2d_traits() {
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
    impl_common_traits::<MeasurePoint2d<Metre>>();
}
