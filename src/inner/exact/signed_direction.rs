#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_signed_direction {
    { $with_points:ident $with_serde:ident } => {
        /// Direction in a plane, represented by an angle with value
        /// between minus half cycle (included) and plus half cycle (excluded),
        /// with generic angular unit of measurement, generic value type,
        /// and with a dynamic value.
        pub struct SignedDirection<Unit, Number = f64>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            // Returns the only value that in the current Unit represents `x`, and
            // is between zero (included) and one cycle (excluded).
            fn normalize(x: Number) -> Number {
                let one_cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let half_cycle = one_cycle * Number::HALF;
                let x2 = (x + half_cycle) % one_cycle;
                if x2 >= Number::ZERO {
                    x2 - half_cycle
                } else {
                    x2 + half_cycle
                }
            }

            /// SignedDirection::new(Number) -> SignedDirection
            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            measures::if_all_true! { { $with_points }
                /// SignedDirection::from_measure_point(MeasurePoint) -> SignedDirection
                pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                    Self::new(m.value)
                }

                /// SignedDirection.to_measure_point() -> MeasurePoint
                pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                    MeasurePoint::<Unit, Number>::new(self.value)
                }
            }

            /// SignedDirection.to_unsigned_direction() -> UnsignedDirection
            pub fn to_unsigned_direction(self) -> UnsignedDirection<Unit, Number> {
                UnsignedDirection::<Unit, Number>::new(self.value)
            }

            /// SignedDirection.convert() -> SignedDirection
            pub fn convert<DestUnit>(self) -> SignedDirection<DestUnit, Number>
            where
                DestUnit: AngleMeasurementUnit<Property = Unit::Property>,
            {
                SignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            /// SignedDirection.lossless_into() -> SignedDirection
            pub fn lossless_into<DestNumber>(self) -> SignedDirection<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                SignedDirection::<Unit, DestNumber>::new(DestNumber::from(self.value))
            }

            /// SignedDirection.lossy_into() -> SignedDirection
            pub fn lossy_into<DestNumber>(self) -> SignedDirection<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                SignedDirection::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        impl<Unit, Number> Default for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection::default() -> SignedDirection
            /// It returns the zero direction (to the right).
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        measures::if_all_true! { { $with_points }
            impl<Unit, Number> From<SignedDirection<Unit, Number>> for MeasurePoint<Unit, Number>
            where
                Unit: AngleMeasurementUnit,
                Number: ArithmeticOps,
            {
                /// MeasurePoint::from(SignedDirection) -> MeasurePoint
                /// SignedDirection.into() -> MeasurePoint
                fn from(m: SignedDirection<Unit, Number>) -> Self {
                    MeasurePoint::<Unit, Number>::new(m.value)
                }
            }
        }

        impl<Unit> From<SignedDirection<Unit, f32>> for SignedDirection<Unit, f64>
        where
            Unit: AngleMeasurementUnit,
        {
            /// SignedDirection<f64>::from(SignedDirection<f32>) -> SignedDirection<f64>
            /// SignedDirection<f32>.into() -> SignedDirection<f64>
            fn from(m: SignedDirection<Unit, f32>) -> Self {
                Self::new(m.value as f64)
            }
        }

        measures::if_all_true! { { $with_serde }
            impl<Unit, Number> serde::Serialize for SignedDirection<Unit, Number>
            where
                Unit: AngleMeasurementUnit,
                Number: ArithmeticOps + serde::Serialize,
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer
                {
                    self.value.serialize(serializer)
                }
            }

            impl<'de, Unit, Number> serde::Deserialize<'de> for SignedDirection<Unit, Number>
            where
                Unit: AngleMeasurementUnit,
                Number: ArithmeticOps + serde::Deserialize<'de>,
            {
                fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
                where
                    De: serde::Deserializer<'de>,
                {
                    Ok(Self::new(serde::Deserialize::deserialize(deserializer)?))
                }
            }
        }

        impl<Unit, Number> Add<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// SignedDirection + AngleMeasure -> SignedDirection
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection += AngleMeasure
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self + other;
            }
        }

        impl<Unit, Number> Sub<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// SignedDirection - AngleMeasure -> SignedDirection
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection -= AngleMeasure
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self - other;
            }
        }

        impl<AngleUnit, Number> Sub<SignedDirection<AngleUnit, Number>>
            for SignedDirection<AngleUnit, Number>
        where
            AngleUnit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<AngleUnit, Number>;

            /// SignedDirection - SignedDirection -> AngleMeasure
            fn sub(self, other: SignedDirection<AngleUnit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let cycle = Number::from_f64(AngleUnit::CYCLE_FRACTION);
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

        impl<Unit, Number> Trigonometry for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Number;

            /// SignedDirection.cos() -> Number
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }

            /// SignedDirection.sin() -> Number
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }

            /// SignedDirection.tan() -> Number
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.tan()
            }

            /// SignedDirection.sin_cos() -> (Number, Number)
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        impl<Unit, Number> PartialEq<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection == SignedDirection -> bool
            fn eq(&self, other: &SignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number> PartialOrd<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection < SignedDirection -> bool
            fn partial_cmp(&self, other: &SignedDirection<Unit, Number>) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number> Clone for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// SignedDirection.clone() -> SignedDirection
            fn clone(&self) -> Self {
                *self
            }
        }

        /// SignedDirection = SignedDirection
        impl<Unit, Number> Copy for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{}", SignedDirection) -> String
            /// SignedDirection.to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in -180°..180°)")
            }
        }

        impl<Unit, Number> fmt::Debug for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", SignedDirection) -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in -180°..180°)")
            }
        }
    };
}
