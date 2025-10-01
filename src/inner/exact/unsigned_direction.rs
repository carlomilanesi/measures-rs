#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_unsigned_direction {
    { $with_points:ident } => {
        pub struct UnsignedDirection<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Returns the only value that in the current Unit represents `x` and
            /// is between minus half cycle included and plus half cycle excluded.
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

                pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                    MeasurePoint::<Unit, Number>::new(self.value)
                }
            }

            pub fn to_signed_direction(self) -> SignedDirection<Unit, Number> {
                SignedDirection::<Unit, Number>::new(self.value)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> UnsignedDirection<DestUnit, Number> {
                UnsignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            /// UnsignedDirection.lossless_into() -> UnsignedDirection
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> UnsignedDirection<Unit, DestNumber> {
                UnsignedDirection::<Unit, DestNumber>::new(DestNumber::from(self.value))
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> UnsignedDirection<Unit, DestNumber> {
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
            // It returns the zero direction (eastbound).
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        impl<Unit> From<UnsignedDirection<Unit, f32>> for UnsignedDirection<Unit, f64>
        where
            Unit: AngleMeasurementUnit,
        {
            fn from(m: UnsignedDirection<Unit, f32>) -> Self {
                Self::new(m.value as f64)
            }
        }

        // Unsigned direction + angle measure -> Unsigned direction
        impl<Unit, Number> Add<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Unsigned direction += angle measure
        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self + other;
            }
        }

        // Unsigned direction - angle measure -> Unsigned direction
        impl<Unit, Number> Sub<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Unsigned direction -= angle measure
        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self - other;
            }
        }

        // unsigned direction - unsigned direction -> angle measure
        impl<Unit, Number> Sub<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<Unit, Number>;
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
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.tan()
            }
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialEq<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            fn eq(&self, other: &UnsignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            fn partial_cmp(&self, other: &UnsignedDirection<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for UnsignedDirection<Unit, Number> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit, Number: ArithmeticOps> Copy for UnsignedDirection<Unit, Number> {}

        // format!("{}", UnsignedDirection)
        impl<Unit, Number> fmt::Display for UnsignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }

        // format!("{:?}", UnsignedDirection)
        impl<Unit, Number> fmt::Debug for UnsignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }
    };
}
