# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

Implemented the fmt::LowerExp and fmt::UpperExp traits for Measure and
MeasurePoint, as well as their 2d and 3d counterparts. With this change,
the value of a measure or measure point can be printed in scientific
notation, with either `e` or `E` as a separator between mantissa and
exponent.

Added `From` trait implementations that allow a `Measure2d` (resp. `Measure3d`) to be directly obtained from a `[Measure; 2]` (resp. `[Measure; 3]`), and similarly for `MeasurePoint2d` and `MeasurePoint3d` as well.
