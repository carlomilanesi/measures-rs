//! Set of declarative macros to define types to handle quantities provided of units of measurement, and optional uncertainty.
//! The only macro to be used in application code is `define_measure_types`, which must be called at most once per module.
//! It is recommended to be called only once per application.
//! It is suggested to called it in a module named `units`.
//!
//! Here is an extensive [**tutorial**](https://github.com/carlomilanesi/measures-rs/blob/main/docs/Tutorial.md) for the last version of the library.
//!
//! # Example of usage
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
//!             Velocity [
//!                 MetrePerSecond {
//!                     suffix: " m/s",
//!                 }
//!             ]
//!         ]
//!         relationships [
//!             Metre 1 == MetrePerSecond 1 * Second 1,
//!         ]
//!     }
//! }
//!
//! let l = units::Measure::<units::Metre, f32>::new(10000.);
//! assert_eq!(
//!     format!("{} = {}", l, l.convert::<units::Foot>()),
//!     "10000 m = 32808.4 ft",
//! );
//!
//! let t = units::ApproxMeasure::<units::Second>::with_variance(5400., 0.81);
//! assert_eq!(
//!     format!("{} = {}", t, t.convert::<units::Hour>()),
//!     "5400 ± 0.9 s = 1.5 ± 0.00025 h",
//! );
//! let v = l.lossless_into::<f64>() / units::Measure::from(t);
//! assert_eq!(
//!     format!("{v}"),
//!     "1.8518518518518519 m/s",
//! );
//! ```
//!
//! Syntax of the macro `define_measure_types!`, in a kind of regular expression, where `[]` means "optional", `\[` and `\]` mean actual brackets, `()*` means "repeat zero or more times", `|` means "or", and items included in the angle brackets `<` and `>` are identifiers, literals or expressions:
//!
//! ```custom
//! measures::define_measure_types! {
//!     [with_points]
//!     [with_directions]
//!     [with_2d]
//!     [with_3d]
//!     [with_transformations]
//!     [exact]
//!     [with_approx],
//!     [
//!     scalar_properties \[
//!         (
//!         <Name of a property> \[
//!             (
//!             <Name of a unit of measurement> {
//!                 suffix: <string literal>,
//!                 [
//!                 ratio: <f64 const expression>,
//!                 [offset: <f64 const expression>,]
//!                 ]
//!             }
//!             )*
//!             \]
//!         )*
//!         \]
//!     ]
//!     [
//!     vector_properties \[
//!         (
//!         <Name of a property> \[
//!             (
//!             <Name of a unit of measurement> {
//!                 suffix: <string literal>,
//!                 [
//!                 ratio: <f64 const expression>,
//!                 ]
//!             }
//!             )*
//!             \]
//!         )*
//!         \]
//!     ]
//!     [
//!     vector_properties \[
//!         (
//!         <Name of a property> \[
//!             (
//!             <Name of a unit of measurement> {
//!                 suffix: <string literal>,
//!                 [
//!                 ratio: <f64 const expression>,
//!                 ]
//!             }
//!             )*
//!             \]
//!         )*
//!         \]
//!     ]
//!     [
//!     angle_measurement_units \[
//!         <Name of a unit of measurement> {
//!             suffix: <string literal>,
//!             cycle_fraction: <f64 const expression>,
//!         }
//!     \]
//!     ]
//!     [
//!     relationships \[
//!         (
//!         <unit> 1 == <unit> 1 * __ 1,
//!         | <unit> 1 == <unit> 1 * <unit> 1,
//!         | <unit> 2 == <unit> 1 * <unit> 2,
//!         | <unit> 2 == <unit> 2 * <unit> 1,
//!         | <unit> 3 == <unit> 1 * <unit> 3,
//!         | <unit> 3 == <unit> 3 * <unit> 1,
//!         | <unit> 1 == <unit> 2 * __ 2,
//!         | <unit> 1 == <unit> 2 * <unit> 2,
//!         | <unit> 1 == <unit> 3 * __ 3,
//!         | <unit> 1 == <unit> 3 * <unit> 3,
//!         | <unit> 1 == <unit> 2 X __ 2,
//!         | <unit> 1 == <unit> 2 X <unit> 2,
//!         | <unit> 3 == <unit> 3 X __ 3,
//!         | <unit> 3 == <unit> 3 X <unit> 3,
//!         )*
//!     \]
//!     ]
//! }
//! ```
pub mod angle;
pub mod define_measure_types;
pub mod define_units_relationship;
pub mod dimensionless;
pub mod inner;
pub mod matrix_utils;
pub mod test_utils;
pub mod traits;
