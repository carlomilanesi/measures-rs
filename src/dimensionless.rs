use crate::traits::{MeasurementUnit, VectorProperty};

// Built-in measure property.
pub struct Dimensionless;
impl VectorProperty for Dimensionless {}

// The only built-in measurement unit for the property Dimensionless.
pub struct One;
impl MeasurementUnit for One {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
