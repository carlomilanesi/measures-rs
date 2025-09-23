use crate::traits::{MeasurementUnit, VectorProperty};

// A built-in vector measure property.
pub struct Dimensionless;
impl VectorProperty for Dimensionless {}

// The only built-in unit of measurement for the property Dimensionless.
pub struct One;
impl MeasurementUnit for One {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
