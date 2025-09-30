measures::define_measure_types! {
    with_3d exact,
    []
}

measures::measurement_vector_property! { P1 }

pub struct U1;
impl MeasurementUnit for U1 {
    type Property = P1;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u1";
}

pub struct P2;

pub struct U2;
impl MeasurementUnit for U2 {
    type Property = P2;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u2";
}

measures::measurement_vector_property! { P3 }

pub struct U3;
impl MeasurementUnit for U3 {
    type Property = P3;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u3";
}

measures::define_units_relationship! { true false false, U1 3 == U2 1 * U3 3 }

fn main() {
    let u1: Measure3d<U1> = Measure::<U2>::new(3.) * Measure3d::<U3>::new(6., -2., 7.);
    assert_eq!(u1.x, 18.);
    assert_eq!(u1.y, -6.);
    assert_eq!(u1.z, 21.);
}
