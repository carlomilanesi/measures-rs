use measures::angle::Radian;
use measures::dimensionless::One;
use measures::{assert_eq_32, assert_eq_64};
use units::{
    ApproxMeasure2d, Degree, Measure, Measure2d, MeasurePoint, Metre, Millimetre, SignedDirection,
    UnsignedDirection,
};

mod units {
    measures::define_measure_types! {
        with_points with_directions with_2d exact with_approx,
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
fn measure_2d_new() {
    let m1: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m1.values, [12_f32, 23_f32]);

    let m1: Measure2d<Metre> = Measure2d::<Metre>::new([12., 23.]);
    assert_eq!(m1.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_convert() {
    let m1 = Measure2d::<Metre, f32>::new([12., 13.]);
    let m2: Measure2d<Millimetre, f32> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f32, 13_f32]);
    assert_eq!(m2.values, [12000_f32, 13000_f32]);

    let m1 = Measure2d::<Metre>::new([12., 13.]);
    let m2: Measure2d<Millimetre> = m1.convert::<Millimetre>();
    assert_eq!(m1.values, [12_f64, 13_f64]);
    assert_eq!(m2.values, [12000_f64, 13000_f64]);
}

#[test]
fn measure_2d_xy_functions() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]);
    let mx: Measure<Metre, f32> = m.x();
    let my: Measure<Metre, f32> = m.y();
    assert_eq!(mx.value, 12_f32);
    assert_eq!(my.value, 23_f32);

    let m: Measure2d<Metre> = Measure2d::<Metre>::new([12., 23.]);
    let mx: Measure<Metre> = m.x();
    let my: Measure<Metre> = m.y();
    assert_eq!(mx.value, 12_f64);
    assert_eq!(my.value, 23_f64);
}

#[test]
fn measure_2d_lossless_into_32_to_32() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure2d<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_2d_lossless_into_32_to_64() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_lossless_into_64_to_64() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    #[allow(clippy::useless_conversion)]
    let m2: Measure2d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

/*
ILLEGAL
#[test]
fn measure_2d_lossless_into_64_to_32() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    _ = m1.lossless_into::<f32>();
}
*/

#[test]
fn measure_2d_lossy_into_32_to_32() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_2d_lossy_into_32_to_64() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_lossy_into_64_to_32() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    let m2: Measure2d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12_f32, 23_f32]);
}

#[test]
fn measure_2d_lossy_into_64_to_64() {
    let m1 = Measure2d::<Metre, f64>::new([12., 23.]);
    let m2: Measure2d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_norm_positive() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure<Metre, f32> = m1.norm();
    assert_eq!(m2.value, (12_f32 * 12_f32 + 23_f32 * 23_f32).sqrt());
}

#[test]
fn measure_2d_norm_negative() {
    let m1 = Measure2d::<Metre, f64>::new([-12., -23.]);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(m2.value, (12_f64 * 12_f64 + 23_f64 * 23_f64).sqrt());
}

#[test]
fn measure_2d_norm_zero() {
    let m1 = Measure2d::<Metre, f64>::new([0., 0.]);
    let m2: Measure<Metre, f64> = m1.norm();
    assert_eq!(m2.value, 0.);
}

#[test]
fn measure_2d_squared_norm_positive() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let n: f32 = m1.squared_norm();
    assert_eq!(n, 12_f32 * 12_f32 + 23_f32 * 23_f32);
}

#[test]
fn measure_2d_squared_norm_negative() {
    let m1 = Measure2d::<Metre, f64>::new([-12., -23.]);
    let n: f64 = m1.squared_norm();
    assert_eq!(n, 12_f64 * 12_f64 + 23_f64 * 23_f64);
}

#[test]
fn measure_2d_squared_norm_zero() {
    let m1 = Measure2d::<Metre, f64>::new([0., 0.]);
    let m2: f64 = m1.squared_norm();
    assert_eq!(m2, 0_f64);
}

#[test]
fn measure_2d_normalized_positive() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2: Measure2d<Metre, f32> = m1.normalized();
    assert_eq_32!(m2.squared_norm(), 1.);
    assert_eq!(m1.values[0].signum(), m2.values[0].signum());
    assert_eq!(m1.values[1].signum(), m2.values[1].signum());
    assert_eq_32!(m1.values[1] / m1.values[0], m2.values[1] / m2.values[0]);
    assert_eq_32!(
        m1.signed_direction::<Radian>().value,
        m2.signed_direction::<Radian>().value
    );
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
fn measure_2d_normalized_positive_zero() {
    let m1 = Measure2d::<Metre, f32>::new([0., 0.]);
    let m2: Measure2d<Metre, f32> = m1.normalized();
    assert!(m2.values[0].is_nan());
    assert!(m2.values[1].is_nan());
}

#[test]
fn measure_2d_normalized_negative_zero() {
    let m1 = Measure2d::<Metre, f32>::new([-0., -0.]);
    let m2: Measure2d<Metre, f32> = m1.normalized();
    assert!(m2.values[0].is_nan());
    assert!(m2.values[1].is_nan());
}

#[test]
fn measure_2d_from_direction_up() {
    let dir = MeasurePoint::<Degree, f32>::new(90.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [0_f32, 1_f32]);
}

#[test]
fn measure_2d_from_direction_down_left() {
    let half_sqrt = 1_f32 / 2_f32.sqrt();
    let dir = MeasurePoint::<Degree, f32>::new(-135.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [-half_sqrt, -half_sqrt]);
}

#[test]
fn measure_2d_from_signed_direction_up() {
    let dir = SignedDirection::<Degree, f32>::new(90.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [0_f32, 1_f32]);
}

#[test]
fn measure_2d_from_signed_direction_down_left() {
    let half_sqrt = 1_f32 / 2_f32.sqrt();
    let dir = SignedDirection::<Degree, f32>::new(-135.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [-half_sqrt, -half_sqrt]);
}

#[test]
fn measure_2d_from_unsigned_direction_up() {
    let dir = UnsignedDirection::<Degree, f32>::new(90.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [0_f32, 1_f32]);
}

#[test]
fn measure_2d_from_unsigned_direction_down_left() {
    let half_sqrt = 1_f32 / 2_f32.sqrt();
    let dir = UnsignedDirection::<Degree, f32>::new(-135.);
    let m = Measure2d::<Metre, f32>::from_angle(dir);
    assert_eq_32!(m.values, [-half_sqrt, -half_sqrt]);
}

#[test]
fn measure_2d_measure_direction_up() {
    let m = Measure2d::<Metre, f32>::new([0., 12.]);
    let measure_dir: MeasurePoint<Degree, f32> = m.measure_direction::<Degree>();
    assert_eq_32!(measure_dir.value, 90_f32);
}

#[test]
fn measure_2d_measure_direction_down_left() {
    let m = Measure2d::<Metre, f32>::new([-23., -23.]);
    let measure_dir: MeasurePoint<Degree, f32> = m.measure_direction::<Degree>();
    assert_eq_32!(measure_dir.value, -135_f32);
}

#[test]
fn measure_2d_signed_direction_up() {
    let m = Measure2d::<Metre, f32>::new([0., 12.]);
    let signed_dir = m.signed_direction::<Degree>();
    assert_eq_32!(signed_dir.value, 90_f32);
}

#[test]
fn measure_2d_signed_direction_down_left() {
    let m = Measure2d::<Metre, f32>::new([-23., -23.]);
    let signed_dir = m.signed_direction::<Degree>();
    assert_eq_32!(signed_dir.value, -135_f32);
}

#[test]
fn measure_2d_unsigned_direction_up() {
    let m = Measure2d::<Metre, f32>::new([0., 12.]);
    let unsigned_dir = m.unsigned_direction::<Degree>();
    assert_eq_32!(unsigned_dir.value, 90_f32);
}

#[test]
fn measure_2d_unsigned_direction_down_left() {
    let m = Measure2d::<Metre, f32>::new([-23., -23.]);
    let unsigned_dir = m.unsigned_direction::<Degree>();
    assert_eq_32!(unsigned_dir.value, 225_f32);
}

#[test]
fn measure_2d_default() {
    let m: Measure2d<Metre, f32> = Measure2d::default();
    assert_eq!(m.values, [0_f32, 0_f32]);

    let m = Measure2d::<Metre>::default();
    assert_eq!(m.values, [0_f64, 0_f64]);
}

#[test]
fn measure_2d_from_f32_into_f64() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f64>::from(m1);
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
    let m3: Measure2d<Metre, f64> = m1.into();
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m3.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_from_approx_into_measure() {
    let am =
        ApproxMeasure2d::<Metre, f32>::with_covariances([12., 23.], [[0.09, 0.01], [0.02, 0.16]]);
    let m1 = Measure2d::<Metre, f32>::from(am);
    assert_eq!(am.values, [12_f32, 23_f32]);
    assert_eq!(m1.values, [12_f32, 23_f32]);
    let m2: Measure2d<Metre, f32> = am.into();
    assert_eq!(am.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [12_f32, 23_f32]);

    let am = ApproxMeasure2d::<Metre>::with_covariances([12., 23.], [[0.09, 0.01], [0.02, 0.16]]);
    let m1 = Measure2d::<Metre>::from(am);
    assert_eq!(am.values, [12_f64, 23_f64]);
    assert_eq!(m1.values, [12_f64, 23_f64]);
    let m2: Measure2d<Metre> = am.into();
    assert_eq!(am.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [12_f64, 23_f64]);
}

#[test]
fn measure_2d_unary_minus() {
    let m = -Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values, [-12_f32, -23_f32]);

    let m = -Measure2d::<Metre>::new([12., 23.]);
    assert_eq!(m.values, [-12_f64, -23_f64]);

    let m = -Measure2d::<Metre, f32>::new([-12., -23.]);
    assert_eq!(m.values, [12_f32, 23_f32]);

    let m = -Measure2d::<Metre>::new([-12., -23.]);
    assert_eq!(m.values, [12_f64, 23_f64]);

    let m = -Measure2d::<Metre, f32>::new([0., 0.]);
    assert_eq!(m.values, [0_f32, 0_f32]);

    let m = -Measure2d::<Metre>::new([-0., -0.]);
    assert_eq!(m.values, [0_f64, 0_f64]);
}

#[test]
fn measure_2d_addition() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    let m3: Measure2d<Metre, f32> = m1 + m2;
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [34_f32, -45_f32]);
    assert_eq!(m3.values, [12_f32 + 34_f32, 23_f32 + -45_f32]);

    let m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([34., -45.]);
    let m3: Measure2d<Metre> = m1 + m2;
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [34_f64, -45_f64]);
    assert_eq!(m3.values, [12_f64 + 34_f64, 23_f64 + -45_f64]);
}

#[test]
fn measure_2d_addition_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    m1 += m2;
    assert_eq!(m1.values, [12_f32 + 34_f32, 23_f32 + -45_f32]);
    assert_eq!(m2.values, [34_f32, -45_f32]);

    let mut m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([34., -45.]);
    m1 += m2;
    assert_eq!(m1.values, [12_f64 + 34_f64, 23_f64 + -45_f64]);
    assert_eq!(m2.values, [34_f64, -45_f64]);
}

#[test]
fn measure_2d_subtraction() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    let m3 = m1 - m2;
    assert_eq!(m1.values, [12_f32, 23_f32]);
    assert_eq!(m2.values, [34_f32, -45_f32]);
    assert_eq!(m3.values, [12_f32 - 34_f32, 23_f32 - -45_f32]);

    let m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([34., -45.]);
    let m3 = m1 - m2;
    assert_eq!(m1.values, [12_f64, 23_f64]);
    assert_eq!(m2.values, [34_f64, -45_f64]);
    assert_eq!(m3.values, [12_f64 - 34_f64, 23_f64 - -45_f64]);
}

#[test]
fn measure_2d_subtraction_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 23.]);
    let m2 = Measure2d::<Metre, f32>::new([34., -45.]);
    m1 -= m2;
    assert_eq!(m1.values, [12_f32 - 34_f32, 23_f32 - -45_f32]);
    assert_eq!(m2.values, [34_f32, -45_f32]);

    let mut m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([34., -45.]);
    m1 -= m2;
    assert_eq!(m1.values, [12_f64 - 34_f64, 23_f64 - -45_f64]);
    assert_eq!(m2.values, [34_f64, -45_f64]);
}

#[test]
fn measure_2d_scalar_post_multiplication() {
    let m: Measure2d<Metre, f32> = Measure2d::<Metre, f32>::new([12., 23.]) * 3.;
    assert_eq!(m.values, [12_f32 * 3_f32, 23_f32 * 3_f32]);

    let m: Measure2d<Metre, f64> = Measure2d::<Metre, f64>::new([12., 23.]) * 3.;
    assert_eq!(m.values, [12_f64 * 3_f64, 23_f64 * 3_f64]);
}

#[test]
fn measure_2d_one_post_multiplication() {
    let m1 = Measure2d::<Metre, f32>::new([12., 23.]) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [36_f32, 69_f32]);

    let m2 = Measure2d::<Metre, f64>::new([12., 23.]) * Measure::<One, f64>::new(3.);
    assert_eq!(m2.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_one_2d_post_multiplication() {
    let m1 = Measure::<Metre, f32>::new(12.) * Measure2d::<One, f32>::new([3., 4.]);
    assert_eq!(m1.values, [36_f32, 48_f32]);

    let m2 = Measure::<Metre, f64>::new(12.) * Measure2d::<One, f64>::new([3., 4.]);
    assert_eq!(m2.values, [36_f64, 48_f64]);
}

#[test]
fn measure_2d_one_pre_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure2d::<Metre, f32>::new([3., 4.]);
    assert_eq!(m1.values, [36_f32, 48_f32]);

    let m2 = Measure::<One>::new(12.) * Measure2d::<Metre>::new([3., 4.]);
    assert_eq!(m2.values, [36_f64, 48_f64]);
}

#[test]
fn measure_2d_one_2d_pre_multiplication() {
    let m1 = Measure2d::<One, f32>::new([12., 23.]) * Measure::<Metre, f32>::new(3.);
    assert_eq!(m1.values, [36_f32, 69_f32]);

    let m2 = Measure2d::<One>::new([12., 23.]) * Measure::<Metre>::new(3.);
    assert_eq!(m2.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_one_one_multiplication() {
    let m1 = Measure::<One, f32>::new(12.) * Measure2d::<One, f32>::new([3., 4.]);
    assert_eq!(m1.values, [36_f32, 48_f32]);

    let m2 = Measure::<One>::new(12.) * Measure2d::<One>::new([3., 4.]);
    assert_eq!(m2.values, [36_f64, 48_f64]);
}

#[test]
fn measure_2d_one_2d_one_multiplication() {
    let m1 = Measure2d::<One, f32>::new([12., 23.]) * Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [36_f32, 69_f32]);

    let m2 = Measure2d::<One>::new([12., 23.]) * Measure::<One>::new(3.);
    assert_eq!(m2.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_scalar_multiplication_assignment() {
    let mut m = Measure2d::<Metre, f32>::new([12., 23.]);
    m *= 3.;
    assert_eq!(m.values, [36_f32, 69_f32]);

    let mut m = Measure2d::<Metre>::new([12., 23.]);
    m *= 3.;
    assert_eq!(m.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_one_multiplication_assignment() {
    let mut m = Measure2d::<Metre, f32>::new([12., 23.]);
    m *= Measure::<One, f32>::new(3.);
    assert_eq!(m.values, [36_f32, 69_f32]);

    let mut m = Measure2d::<Metre>::new([12., 23.]);
    m *= Measure::<One>::new(3.);
    assert_eq!(m.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_scalar_pre_multiplication() {
    let m: Measure2d<Metre, f32> = 3. * Measure2d::<Metre, f32>::new([12., 23.]);
    assert_eq!(m.values, [36_f32, 69_f32]);

    let m: Measure2d<Metre> = 3. * Measure2d::<Metre>::new([12., 23.]);
    assert_eq!(m.values, [36_f64, 69_f64]);
}

#[test]
fn measure_2d_scalar_division() {
    let m1 = Measure2d::<Metre, f32>::new([12., 48.]) / 3.;
    assert_eq!(m1.values, [4_f32, 16_f32]);

    let m2 = Measure2d::<Metre>::new([12., 48.]) / 3.;
    assert_eq!(m2.values, [4_f64, 16_f64]);
}

#[test]
fn measure_2d_measure_division() {
    let m1 = Measure2d::<Metre, f32>::new([12., 48.]);
    let m2 = Measure::<Metre, f32>::new(3.);
    let m3: Measure2d<One, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32]);

    let m4 = Measure2d::<Metre>::new([12., 48.]);
    let m5 = Measure::<Metre>::new(3.);
    let m6: Measure2d<One> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64]);
}

#[test]
fn measure_2d_one_division() {
    let m1 = Measure2d::<Metre, f32>::new([12., 48.]);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure2d<Metre, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32]);

    let m4 = Measure2d::<Metre>::new([12., 48.]);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure2d<Metre> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64]);
}

#[test]
fn measure_2d_one_one_division() {
    let m1 = Measure2d::<One, f32>::new([12., 48.]);
    let m2 = Measure::<One, f32>::new(3.);
    let m3: Measure2d<One, f32> = m1 / m2;
    assert_eq!(m3.values, [4_f32, 16_f32]);

    let m4 = Measure2d::<One>::new([12., 48.]);
    let m5 = Measure::<One>::new(3.);
    let m6: Measure2d<One> = m4 / m5;
    assert_eq!(m6.values, [4_f64, 16_f64]);
}

#[test]
fn measure_2d_scalar_division_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 48.]);
    m1 /= 3.;
    assert_eq!(m1.values, [4_f32, 16_f32]);

    let mut m2 = Measure2d::<Metre>::new([12., 48.]);
    m2 /= 3.;
    assert_eq!(m2.values, [4_f64, 16_f64]);
}

#[test]
fn measure_2d_one_division_assignment() {
    let mut m1 = Measure2d::<Metre, f32>::new([12., 48.]);
    m1 /= Measure::<One, f32>::new(3.);
    assert_eq!(m1.values, [4_f32, 16_f32]);

    let mut m2 = Measure2d::<Metre>::new([12., 48.]);
    m2 /= Measure::<One>::new(3.);
    assert_eq!(m2.values, [4_f64, 16_f64]);
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

    let m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([12., 23.]);
    let m3 = Measure2d::<Metre>::new([12., 23.2]);
    let m4 = Measure2d::<Metre>::new([12.1, 23.]);
    let m5 = Measure2d::<Metre>::new([12.1, 23.2]);
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

    let m1 = Measure2d::<Metre>::new([12., 23.]);
    let m2 = Measure2d::<Metre>::new([12., 23.]);
    let m3 = Measure2d::<Metre>::new([12., 23.2]);
    let m4 = Measure2d::<Metre>::new([12.1, 23.]);
    let m5 = Measure2d::<Metre>::new([12.1, 23.2]);
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
fn measure_2d_formatting() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5020]);
    assert_eq!(format!("{}", m), "(12.25, 23.502) m");
}

#[test]
fn measure_2d_formatting_with_one_fractional_digit() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5498]);
    assert_eq!(format!("{:.1}", m), "(12.2, 23.5) m");
}

#[test]
fn measure_2d_formatting_for_debug() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5020]);
    assert_eq!(format!("{:?}", m), "(12.25, 23.502) m");
}

#[test]
fn measure_2d_formatting_for_debug_with_one_fractional_digit() {
    let m = Measure2d::<Metre, f32>::new([12.25, 23.5498]);
    assert_eq!(format!("{:.1?}", m), "(12.2, 23.5) m");
}

#[test]
fn measure_2d_traits() {
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
    impl_common_traits::<Measure2d<Metre>>();
}
