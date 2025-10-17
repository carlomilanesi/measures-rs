use crate::measurement_scalar_property;
use crate::traits::{AngleMeasurementUnit, MeasurementUnit};

// A built-in scalar measure property.

measurement_scalar_property! { Angle }

/// The only built-in unit of measurement for the property `Angle`.
pub struct Radian;
impl MeasurementUnit for Radian {
    type Property = Angle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}

/// Measure of a full circle in radians.
impl AngleMeasurementUnit for Radian {
    const CYCLE_FRACTION: f64 = core::f64::consts::TAU;
}
