#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_unsigned_direction {
    { $with_points:ident } => {
        /// Direction in a plane, represented by an angle with value
        /// between zero (included) and a cycle (excluded),
        /// with generic angular unit of measurement, generic value type,
        /// and with a dynamic value.
        pub struct UnsignedDirection<Unit, Number = f64>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            // Returns the only value that in the current Unit represents `x` and
            // is between minus half cycle (included) and plus half cycle (excluded).
            fn normalize(x: Number) -> Number {
                let one_cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let x2 = x % one_cycle;
                if x2 >= Number::ZERO {
                    x2
                } else {
                    x2 + one_cycle
                }
            }

            /// UnsignedDirection::new(Number) -> UnsignedDirection
            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            measures::if_all_true! { { $with_points }
                /// UnsignedDirection::from_measure_point(MeasurePoint) -> UnsignedDirection
                pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                    Self::new(m.value)
                }

                /// UnsignedDirection.to_measure_point() -> MeasurePoint
                pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                    MeasurePoint::<Unit, Number>::new(self.value)
                }
            }

            /// UnsignedDirection.to_signed_direction() -> SignedDirection
            pub fn to_signed_direction(self) -> SignedDirection<Unit, Number> {
                SignedDirection::<Unit, Number>::new(self.value)
            }

            /// UnsignedDirection.convert() -> UnsignedDirection
            pub fn convert<DestUnit>(self) -> UnsignedDirection<DestUnit, Number>
            where
                DestUnit: AngleMeasurementUnit<Property = Unit::Property>,
            {
                UnsignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            /// UnsignedDirection.lossless_into() -> UnsignedDirection
            pub fn lossless_into<DestNumber>(self) -> UnsignedDirection<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                UnsignedDirection::<Unit, DestNumber>::new(DestNumber::from(self.value))
            }

            /// UnsignedDirection.lossy_into() -> UnsignedDirection
            pub fn lossy_into<DestNumber>(self) -> UnsignedDirection<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                UnsignedDirection::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        impl<Unit, Number> Default for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection::default() -> UnsignedDirection
            /// It returns the zero direction (to the right).
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        measures::if_all_true! { { $with_points }
            impl<Unit, Number> From<UnsignedDirection<Unit, Number>> for MeasurePoint<Unit, Number>
            where
                Unit: AngleMeasurementUnit,
                Number: ArithmeticOps,
            {
                /// MeasurePoint::from(UnsignedDirection) -> MeasurePoint
                /// UnsignedDirection.into() -> MeasurePoint
                fn from(m: UnsignedDirection<Unit, Number>) -> Self {
                    MeasurePoint::<Unit, Number>::new(m.value)
                }
            }
        }

        impl<Unit> From<UnsignedDirection<Unit, f32>> for UnsignedDirection<Unit, f64>
        where
            Unit: AngleMeasurementUnit,
        {
            /// UnsignedDirection<f64>::from(UnsignedDirection<f32>) -> UnsignedDirection<f64>
            /// UnsignedDirection<f32>.into() -> UnsignedDirection<f64>
            fn from(m: UnsignedDirection<Unit, f32>) -> Self {
                Self::new(m.value as f64)
            }
        }

        impl<Unit, Number> Add<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// UnsignedDirection + AngleMeasure -> UnsignedDirection
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection += AngleMeasure
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self + other;
            }
        }

        impl<Unit, Number> Sub<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// UnsignedDirection - AngleMeasure -> UnsignedDirection
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection -= AngleMeasure
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self - other;
            }
        }

        impl<Unit, Number> Sub<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<Unit, Number>;

            /// UnsignedDirection - UnsignedDirection -> AngleMeasure
            fn sub(self, other: UnsignedDirection<Unit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let half_cycle = cycle * Number::HALF;
                Self::Output::new(if diff > half_cycle {
                    diff - cycle
                } else if diff < -half_cycle {
                    diff + cycle
                } else {
                    diff
                })
            }
        }

        impl<Unit, Number> Trigonometry for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Number;

            /// UnsignedDirection.cos() -> Number
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }

            /// UnsignedDirection.sin() -> Number
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }

            /// UnsignedDirection.tan() -> Number
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.tan()
            }

            /// UnsignedDirection.sin_cos() -> (Number, Number)
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        impl<Unit, Number> PartialEq<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection == UnsignedDirection -> bool
            fn eq(&self, other: &UnsignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number> PartialOrd<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection < UnsignedDirection -> bool
            fn partial_cmp(&self, other: &UnsignedDirection<Unit, Number>) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number> Clone for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// UnsignedDirection.clone() -> UnsignedDirection
            fn clone(&self) -> Self {
                *self
            }
        }

        /// UnsignedDirection = UnsignedDirection
        impl<Unit, Number> Copy for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{}", UnsignedDirection) -> String
            /// UnsignedDirection.to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }

        impl<Unit, Number> fmt::Debug for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", UnsignedDirection) -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }
    };
}
