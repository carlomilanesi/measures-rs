use measures::dimensionless::One;
use measures::{assert_eq_32, assert_eq_64};
use units::{ApproxMeasure3d, Measure, Measure3d, Metre, Millimetre};

mod units {
    measures::define_measure_types! {
        with_3d exact with_approx,
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
fn measure_3d_new() {
    let m1: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);

    let m1: Measure3d<Metre> = Measure3d::<Metre>::new([12., 23., 34.]);
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_convert() {
    let m1: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: Measure3d<Millimetre, f32> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12., 23., 34.]);
    assert_eq!(m2.values, [12000., 23000., 34000.]);
}

#[test]
fn measure_3d_xyz_functions() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let mx: Measure<Metre, f32> = m.x();
    let my: Measure<Metre, f32> = m.y();
    let mz: Measure<Metre, f32> = m.z();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
    assert_eq!(mz.value, 34.);
}

#[test]
fn measure_3d_lossless_into_32_to_32() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure3d<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_3d_lossless_into_32_to_64() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_lossless_into_64_to_64() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., 34.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure3d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

/*
ILLEGAL
#[test]
fn measure_3d_lossless_into_64_to_32() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., 34.]);
    _ = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_3d_lossy_into_32_to_32() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_3d_lossy_into_32_to_64() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_lossy_into_64_to_32() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_3d_lossy_into_64_to_64() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_norm_positive() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2: Measure<Metre, f32> = m1.norm();
    assert_eq!(
        m2.value,
        (12_f32 * 12_f32 + 23_f32 * 23_f32 + 34_f32 * 34_f32).sqrt()
    );
}

#[test]
fn measure_3d_norm_negative() {
    let m1 = Measure3d::<Metre, f64>::new([-12., -23., -34.]);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(
        m2.value,
        (12_f64 * 12_f64 + 23_f64 * 23_f64 + 34_f64 * 34_f64).sqrt()
    );
}

#[test]
fn measure_3d_norm_zero() {
    let m1 = Measure3d::<Metre, f64>::new([0., 0., 0.]);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(m2.value, 0_f64);
}

#[test]
fn measure_3d_squared_norm_positive() {
    let m = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let n: f32 = m.squared_norm();
    assert_eq!(n, 12_f32 * 12_f32 + 23_f32 * 23_f32 + 34_f32 * 34_f32);
}

#[test]
fn measure_3d_squared_norm_negative() {
    let m = Measure3d::<Metre, f64>::new([-12., -23., -34.]);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 12_f64 * 12_f64 + 23_f64 * 23_f64 + 34_f64 * 34_f64);
}

#[test]
fn measure_3d_squared_norm_zero() {
    let m = Measure3d::<Metre, f64>::new([0., 0., 0.]);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 0.);
}

#[test]
fn measure_3d_normalized_positive() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_32!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq!(m1.values[2].signum(), m2.values[2].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(m1.values[2] / m1.values[0], m2.values[2] / m2.values[0]);
}

#[test]
fn measure_3d_normalized_x_negative() {
    let m1 = Measure3d::<Metre, f64>::new([-12., 23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq!(m1.values[2].signum(), m2.values[2].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(m1.values[2] / m1.values[0], m2.values[2] / m2.values[0]);
}

#[test]
fn measure_3d_normalized_y_negative() {
    let m1 = Measure3d::<Metre, f64>::new([12., -23., 34.]);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq!(m1.values[2].signum(), m2.values[2].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(m1.values[2] / m1.values[0], m2.values[2] / m2.values[0]);
}

#[test]
fn measure_3d_normalized_z_negative() {
    let m1 = Measure3d::<Metre, f64>::new([12., 23., -34.]);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq!(m1.values[2].signum(), m2.values[2].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(m1.values[2] / m1.values[0], m2.values[2] / m2.values[0]);
}

#[test]
fn measure_3d_normalized_xyz_negative() {
    let m1 = Measure3d::<Metre, f64>::new([-12., -23., -34.]);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq!(m1.values[2].signum(), m2.values[2].signum());
    assert_eq_64!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_64!(m1.values[2] / m1.values[0], m2.values[2] / m2.values[0]);
}

#[test]
fn measure_3d_normalized_zero() {
    let m1 = Measure3d::<Metre, f32>::new([0., 0., 0.]);
    let m2: Measure3d<Metre, f32> = m1.normalized();
    assert!(m2.values[0].is_nan());
    assert!(m2.values[1].is_nan());
    assert!(m2.values[2].is_nan());
}

#[test]
fn measure_3d_default() {
    let m: Measure3d<Metre, f32> = Measure3d::default();
    assert_eq!(m.values, [0_f32, 0_f32, 0_f32]);

    let m = Measure3d::<Metre>::default();
    assert_eq!(m.values, [0_f64, 0_f64, 0_f64]);
}

#[test]
fn measure_3d_from_three_measures() {
    let three_measures = [
        Measure::<Metre, f32>::new(12.),
        Measure::<Metre, f32>::new(23.),
        Measure::<Metre, f32>::new(34.),
    ];

    let m1 = Measure3d::<Metre, f32>::from(three_measures);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);

    let m2: Measure3d<Metre, f32> = three_measures.into();
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);
}

#[test]
fn measure_3d_from_f32_into_f64() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f64>::from(m1);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
    let m3: Measure3d<Metre, f64> = m1.into();
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m3.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_from_approx_into_measure() {
    let am = ApproxMeasure3d::<Metre, f32>::with_covariances(
        [12., 23., 34.],
        [[0.09, 0.01, 0.03], [0.02, 0.16, 0.01], [0.02, 0.03, 0.16]],
    );
    let m1 = Measure3d::<Metre, f32>::from(am);
    assert_eq!(am.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    let m2: Measure3d<Metre, f32> = am.into();
    assert_eq!(am.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32, 34_f32]);

    let am = ApproxMeasure3d::<Metre>::with_covariances(
        [12., 23., 34.],
        [[0.09, 0.01, 0.03], [0.02, 0.16, 0.01], [0.02, 0.03, 0.16]],
    );
    let m1 = Measure3d::<Metre>::from(am);
    assert_eq!(am.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    let m2: Measure3d<Metre> = am.into();
    assert_eq!(am.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64, 34_f64]);
}

#[test]
fn measure_3d_unary_minus() {
    let m = -Measure3d::<Metre, f32>::new([12., 23., 34.]);
    assert_eq!(m.values, [-12_f32, -23_f32, -34_f32]);

    let m = -Measure3d::<Metre>::new([12., 23., 34.]);
    assert_eq!(m.values, [-12_f64, -23_f64, -34_f64]);

    let m = -Measure3d::<Metre, f32>::new([-12., -23., -34.]);
    assert_eq!(m.values, [12_f32, 23_f32, 34_f32]);

    let m = -Measure3d::<Metre>::new([-12., -23., -34.]);
    assert_eq!(m.values, [12_f64, 23_f64, 34_f64]);

    let m = -Measure3d::<Metre, f32>::new([0., 0., 0.]);
    assert_eq!(m.values, [0_f32, 0_f32, 0_f32]);

    let m = -Measure3d::<Metre>::new([-0., -0., -0.]);
    assert_eq!(m.values, [0_f64, 0_f64, 0_f64]);
}

#[test]
fn measure_3d_addition() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    let m3: Measure3d<Metre, f32> = m1 + m2;
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [45_f32, -56_f32, 67_f32]);
    assert_eq!(
        m3.values,
        [12_f32 + 45_f32, 23_f32 + -56_f32, 34_f32 + 67_f32]
    );

    let m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([45., -56., 67.]);
    let m3: Measure3d<Metre> = m1 + m2;
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [45_f64, -56_f64, 67_f64]);
    assert_eq!(
        m3.values,
        [12_f64 + 45_f64, 23_f64 + -56_f64, 34_f64 + 67_f64]
    );
}

#[test]
fn measure_3d_addition_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([34., -45., 67.]);
    m1 += m2;
    assert_eq!(
        m1.values,
        [12_f32 + 34_f32, 23_f32 + -45_f32, 34_f32 + 67_f32]
    );
    assert_eq!(m2.values, [34_f32, -45_f32, 67_f32]);

    let mut m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([34., -45., 67.]);
    m1 += m2;
    assert_eq!(
        m1.values,
        [12_f64 + 34_f64, 23_f64 + -45_f64, 34_f64 + 67_f64]
    );
    assert_eq!(m2.values, [34_f64, -45_f64, 67_f64]);
}

#[test]
fn measure_3d_subtraction() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    let m3 = m1 - m2;
    assert_eq!(m1.values, [12_f32, 23_f32, 34_f32]);
    assert_eq!(m2.values, [45_f32, -56_f32, 67_f32]);
    assert_eq!(
        m3.values,
        [12_f32 - 45_f32, 23_f32 - -56_f32, 34_f32 - 67_f32]
    );

    let m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([45., -56., 67.]);
    let m3 = m1 - m2;
    assert_eq!(m1.values, [12_f64, 23_f64, 34_f64]);
    assert_eq!(m2.values, [45_f64, -56_f64, 67_f64]);
    assert_eq!(
        m3.values,
        [12_f64 - 45_f64, 23_f64 - -56_f64, 34_f64 - 67_f64]
    );
}

#[test]
fn measure_3d_subtraction_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([45., -56., 67.]);
    m1 -= m2;
    assert_eq!(
        m1.values,
        [12_f32 - 45_f32, 23_f32 - -56_f32, 34_f32 - 67_f32]
    );
    assert_eq!(m2.values, [45_f32, -56_f32, 67_f32]);

    let mut m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([45., -56., 67.]);
    m1 -= m2;
    assert_eq!(
        m1.values,
        [12_f64 - 45_f64, 23_f64 - -56_f64, 34_f64 - 67_f64]
    );
    assert_eq!(m2.values, [45_f64, -56_f64, 67_f64]);
}

#[test]
fn measure_3d_scalar_post_multiplication() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new([12., 23., 34.]) * 3.;
    assert_eq!(m.values, [12_f32 * 3_f32, 23_f32 * 3_f32, 34_f32 * 3_f32]);

    let m: Measure3d<Metre, f64> = Measure3d::<Metre, f64>::new([12., 23., 34.]) * 3.;
    assert_eq!(m.values, [12_f64 * 3_f64, 23_f64 * 3_f64, 34_f64 * 3_f64]);
}

#[test]
fn measure_3d_one_post_multiplication() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let m2 = Measure3d::<Metre, f64>::new([12., 23., 34.]) * Measure::<One, f64>::new(3.);
    assert_eq!(m2.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_one_3d_post_multiplication() {
    let m1 = Measure::<Metre, f32>::new(12.) * Measure3d::<One, f32>::new([3., 4., -7.]);
    assert_eq!(m1.values, [12_f32 * 3_f32, 12_f32 * 4_f32, 12_f32 * -7_f32]);

    let m2 = Measure::<Metre, f64>::new(12.) * Measure3d::<One, f64>::new([3., 4., -7.]);
    assert_eq!(m2.values, [12_f64 * 3_f64, 12_f64 * 4_f64, 12_f64 * -7_f64]);
}

#[test]
fn measure_3d_one_pre_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure3d::<Metre, f32>::new([3., 4., -7.]);
    assert_eq!(m1.values, [12_f32 * 3_f32, 12_f32 * 4_f32, 12_f32 * -7_f32]);

    let m2 = Measure::<One>::new(12.) * Measure3d::<Metre>::new([3., 4., -7.]);
    assert_eq!(m2.values, [12_f64 * 3_f64, 12_f64 * 4_f64, 12_f64 * -7_f64]);
}

#[test]
fn measure_3d_one_3d_pre_multiplication() {
    let m1 = Measure3d::<One, f32>::new([12., 23., 34.]) * Measure::<Metre, f32>::new(3.);
    assert_eq!(m1.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let m2 = Measure3d::<One>::new([12., 23., 34.]) * Measure::<Metre>::new(3.);
    assert_eq!(m2.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_one_one_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure3d::<One, f32>::new([3., 4., -7.]);
    assert_eq!(m1.values, [12_f32 * 3_f32, 12_f32 * 4_f32, 12_f32 * -7_f32]);

    let m2 = Measure::<One>::new(12.) * Measure3d::<One>::new([3., 4., -7.]);
    assert_eq!(m2.values, [12_f64 * 3_f64, 12_f64 * 4_f64, 12_f64 * -7_f64]);
}

#[test]
fn measure_3d_one_3d_one_multiplication() {
    let m1 = Measure3d::<One, f32>::new([12., 23., 34.]) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let m2 = Measure3d::<One>::new([12., 23., 34.]) * Measure::<One>::new(3.);
    assert_eq!(m2.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_scalar_multiplication_assignment() {
    let mut m = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    m *= 3.;
    assert_eq!(m.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let mut m = Measure3d::<Metre>::new([12., 23., 34.]);
    m *= 3.;
    assert_eq!(m.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_one_multiplication_assignment() {
    let mut m = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    m *= Measure::<One, f32>::new(3.);
    assert_eq!(m.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let mut m = Measure3d::<Metre>::new([12., 23., 34.]);
    m *= Measure::<One>::new(3.);
    assert_eq!(m.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_scalar_pre_multiplication() {
    let m: Measure3d<Metre, f32> = 3. * Measure3d::<Metre, f32>::new([12., 23., 34.]);
    assert_eq!(m.values, [12_f32 * 3., 23_f32 * 3., 34_f32 * 3.]);

    let m: Measure3d<Metre> = 3. * Measure3d::<Metre>::new([12., 23., 34.]);
    assert_eq!(m.values, [12_f64 * 3., 23_f64 * 3., 34_f64 * 3.]);
}

#[test]
fn measure_3d_scalar_division() {
    let m1 = Measure3d::<Metre, f32>::new([12., 48., -21.]) / 3.;
    assert_eq!(m1.values, [4_f32, 16_f32, -7_f32]);

    let m2 = Measure3d::<Metre>::new([12., 48., -21.]) / 3.;
    assert_eq!(m2.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_measure_division() {
    let m1 = Measure3d::<Metre, f32>::new([12., 48., -21.]);
    let m2 = Measure::<Metre, f32>::new(3.);
    let m3: Measure3d<One, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32, -7_f32]);

    let m4 = Measure3d::<Metre>::new([12., 48., -21.]);
    let m5 = Measure::<Metre>::new(3.);
    let m6: Measure3d<One> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_one_division() {
    let m1 = Measure3d::<Metre, f32>::new([12., 48., -21.]);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure3d<Metre, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32, -7_f32]);

    let m4 = Measure3d::<Metre>::new([12., 48., -21.]);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure3d<Metre> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_one_one_division() {
    let m1 = Measure3d::<One, f32>::new([12., 48., -21.]);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure3d<One, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32, -7_f32]);

    let m4 = Measure3d::<One>::new([12., 48., -21.]);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure3d<One> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_scalar_division_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new([12., 48., -21.]);
    m1 /= 3.;
    assert_eq!(m1.values, [4_f32, 16_f32, -7_f32]);

    let mut m2 = Measure3d::<Metre>::new([12., 48., -21.]);
    m2 /= 3.;
    assert_eq!(m2.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_one_division_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new([12., 48., -21.]);
    m1 /= Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [4_f32, 16_f32, -7_f32]);

    let mut m2 = Measure3d::<Metre>::new([12., 48., -21.]);
    m2 /= Measure::<One>::new(3.);
    assert_eq!(m2.values, [4_f64, 16_f64, -7_f64]);
}

#[test]
fn measure_3d_equals() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = Measure3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m4 = Measure3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m5 = Measure3d::<Metre, f32>::new([12., 23., 34.3]);
    let m6 = Measure3d::<Metre, f32>::new([12.1, 23.2, 34.3]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
    assert!(!(m1 == m6));

    let m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m3 = Measure3d::<Metre>::new([12.1, 23., 34.]);
    let m4 = Measure3d::<Metre>::new([12., 23.2, 34.]);
    let m5 = Measure3d::<Metre>::new([12., 23., 34.3]);
    let m6 = Measure3d::<Metre>::new([12.1, 23.2, 34.3]);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
    assert!(!(m1 == m6));
}

#[test]
fn measure_3d_differs() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m3 = Measure3d::<Metre, f32>::new([12.1, 23., 34.]);
    let m4 = Measure3d::<Metre, f32>::new([12., 23.2, 34.]);
    let m5 = Measure3d::<Metre, f32>::new([12., 23., 34.3]);
    let m6 = Measure3d::<Metre, f32>::new([12.1, 23.2, 34.3]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
    assert!(m1 != m6);

    let m1 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m2 = Measure3d::<Metre>::new([12., 23., 34.]);
    let m3 = Measure3d::<Metre>::new([12.1, 23., 34.]);
    let m4 = Measure3d::<Metre>::new([12., 23.2, 34.]);
    let m5 = Measure3d::<Metre>::new([12., 23., 34.3]);
    let m6 = Measure3d::<Metre>::new([12.1, 23.2, 34.3]);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
    assert!(m1 != m6);
}

#[test]
fn measure_3d_is_equal_to_its_copy() {
    let m1 = Measure3d::<Metre, f32>::new([12., 23., 34.]);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_3d_formatting() {
    let m = Measure3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{}", m), "(12.25, 23.5, 34.75) m");
}

#[test]
fn measure_3d_formatting_one_fractional_digit() {
    let m = Measure3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:.1}", m), "(12.2, 23.5, 34.8) m");
}

#[test]
fn measure_3d_formatting_for_debug() {
    let m = Measure3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:?}", m), "(12.25, 23.5, 34.75) m");
}

#[test]
fn measure_3d_formatting_for_debug_one_fractional_digit() {
    let m = Measure3d::<Metre, f32>::new([12.25, 23.50, 34.75]);
    assert_eq!(format!("{:.1?}", m), "(12.2, 23.5, 34.8) m");
}

#[test]
fn measure_3d_traits() {
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
    impl_common_traits::<Measure3d<Metre>>();
}
