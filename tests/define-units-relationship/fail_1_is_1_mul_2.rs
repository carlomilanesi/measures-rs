measures::define_measure_types! {
    exact,
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

measures::measurement_vector_property! { P2 }

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

measures::define_units_relationship! { true false false, U1 1 == U2 1 * U3 2 }

fn main() {}
