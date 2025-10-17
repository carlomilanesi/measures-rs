// A built-in vector measure property.
crate::measurement_vector_property! { Dimensionless }

/// The only built-in unit of measurement for the property `Dimensionless`.
pub struct One;
impl crate::traits::MeasurementUnit for One {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
