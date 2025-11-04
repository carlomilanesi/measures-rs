//! Set of declarative macros to define types to handle quantities provided of units of measurement, and optional uncertainty.
//! The only macro to be used in application code is `define_measure_types`, which must be used at most once per module.
//! It is recommended to be used only once per application.
//!
//! Here is an extensive [**tutorial**](https://github.com/carlomilanesi/measures-rs/blob/main/docs/Tutorial.md).
//!
//! # Example
//!
//! ```
//! mod units {
//!     measures::define_measure_types! {
//!         exact with_approx,
//!         scalar_properties [
//!             Time [
//!                 Second {
//!                     suffix: " s",
//!                 }
//!                 Hour {
//!                     suffix: " h",
//!                     ratio: 3600.,
//!                 }
//!             ]
//!         ]
//!         vector_properties [
//!             Length [
//!                 Metre {
//!                     suffix: " m",
//!                 }
//!                 Foot {
//!                     suffix: " ft",
//!                     ratio: 0.3048,
//!                 }
//!             ]
//!         ]
//!     }
//! }
//!
//! let l = units::Measure::<units::Metre, f32>::new(100.);
//! assert_eq!(
//!     format!("{} = {}", l, l.convert::<units::Foot>()),
//!     "100 m = 328.08398 ft",
//! );
//!
//! let t = units::ApproxMeasure::<units::Second>::with_variance(5400., 0.81);
//! assert_eq!(
//!     format!("{} = {}", t, t.convert::<units::Hour>()),
//!     "5400 ± 0.9 s = 1.5 ± 0.00025 h",
//! );
//! ```
pub mod angle;
pub mod define_measure_types;
pub mod define_units_relationship;
pub mod dimensionless;
pub mod inner;
pub mod matrix_utils;
pub mod test_utils;
pub mod traits;
