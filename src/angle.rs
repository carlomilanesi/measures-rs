use crate::traits::{AngleMeasurementUnit, MeasurementUnit};

// Built-in measure property.
pub struct Angle;

// The only built-in measurement unit for the property Angle.
pub struct Radian;
impl MeasurementUnit for Radian {
    type Property = Angle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}

// Measure of a full circle in radians.
impl AngleMeasurementUnit for Radian {
    const CYCLE_FRACTION: f64 = std::f64::consts::TAU;
}
