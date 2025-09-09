use crate::traits::{MeasurementUnit, VectorProperty};

pub struct Dimensionless;
impl VectorProperty for Dimensionless {}

pub struct One;
impl MeasurementUnit for One {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
