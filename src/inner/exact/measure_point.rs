#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_point {
    { $with_approx:ident } => {
        /// 1D absolute measure with generic unit of measurement, generic value type,
        /// and with a dynamic value.
        pub struct MeasurePoint<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint::new(Number) -> MeasurePoint
            pub const fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }

            /// MeasurePoint.convert() -> MeasurePoint
            pub fn convert<DestUnit>(self) -> MeasurePoint<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                MeasurePoint::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            /// MeasurePoint.lossless_into() -> MeasurePoint
            pub fn lossless_into<DestNumber>(self) -> MeasurePoint<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                MeasurePoint::<Unit, DestNumber>::new(DestNumber::from(self.value))
            }

            /// MeasurePoint.lossy_into() -> MeasurePoint
            pub fn lossy_into<DestNumber>(self) -> MeasurePoint<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                MeasurePoint::<Unit, DestNumber>::new(DestNumber::lossy_from(self.value))
            }

            /// MeasurePoint.min(MeasurePoint) -> MeasurePoint
            pub fn min(self, other: Self) -> Self {
                if self <= other {
                    self
                } else {
                    other
                }
            }

            /// MeasurePoint.max(MeasurePoint) -> MeasurePoint
            pub fn max(self, other: Self) -> Self {
                if self >= other {
                    self
                } else {
                    other
                }
            }

            /// MeasurePoint.clamp(MeasurePoint, MeasurePoint) -> MeasurePoint
            pub fn clamp(self, lower_bound: Self, upper_bound: Self) -> Self {
                self.max(lower_bound).min(upper_bound)
            }
        }

        impl<Unit, Number> Default for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint::default() -> MeasurePoint
            /// It returns the origin.
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        /// The trivial conversions from `MeasurePoint<Unit, f32>` to `MeasurePoint<Unit, f32>`
        /// and from `MeasurePoint<Unit, f64>` to `MeasurePoint<Unit, f64>` are provided by the core library.
        /// The lossy conversion from `MeasurePoint<Unit, f64>` to `MeasurePoint<Unit, f32>`
        /// shouldn't be provided by the trait `From`. Use `MeasurePoint.lossy_into()` instead.
        /// This is the lossless conversion from `MeasurePoint<Unit, f32>` to `MeasurePoint<Unit, f64>`
        impl<Unit> From<MeasurePoint<Unit, f32>> for MeasurePoint<Unit, f64>
        where
            Unit: MeasurementUnit,
        {
            /// MeasurePoint<f64>::from(MeasurePoint<f32>) -> MeasurePoint<f64>
            fn from(measure_point: MeasurePoint<Unit, f32>) -> Self {
                Self::new(measure_point.value as f64)
            }
        }

        measures::if_all_true! { { $with_approx }
            impl<Unit, Number> From<ApproxMeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number>
            where
                Unit: MeasurementUnit,
                Number: ArithmeticOps,
            {
                /// MeasurePoint::from(ApproxMeasurePoint) -> MeasurePoint
                fn from(am: ApproxMeasurePoint<Unit, Number>) -> Self {
                    Self::new(am.value)
                }
            }
        }

        impl<Unit, Number> Add<Measure<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// MeasurePoint + Measure -> MeasurePoint
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint += Measure
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        impl<Unit, Number> Sub<Measure<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// MeasurePoint - Measure -> MeasurePoint
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint -= Measure
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        impl<Unit, Number> Sub<MeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<Unit, Number>;

            /// MeasurePoint - MeasurePoint -> Measure
            fn sub(self, other: MeasurePoint<Unit, Number>) -> Self::Output {
                Self::Output::new(self.value - other.value)
            }
        }

        /// weighted_midpoint(MeasurePoint, MeasurePoint, Number) -> MeasurePoint
        pub fn weighted_midpoint<Unit, Number>(
            p1: MeasurePoint<Unit, Number>,
            p2: MeasurePoint<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            MeasurePoint::<Unit, Number>::new(p1.value * weight1 + p2.value * (Number::ONE - weight1))
        }

        /// midpoint(MeasurePoint, MeasurePoint) -> MeasurePoint
        pub fn midpoint<Unit, Number>(
            p1: MeasurePoint<Unit, Number>,
            p2: MeasurePoint<Unit, Number>,
        ) -> MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            MeasurePoint::<Unit, Number>::new((p1.value + p2.value) * Number::HALF)
        }

        /// barycentric_combination([MeasurePoint], [Number]) -> MeasurePoint
        pub fn barycentric_combination<Unit, Number>(
            points: &[MeasurePoint<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            MeasurePoint::<Unit, Number>::new(points.iter().zip(weights).map(|(p, &w)| p.value * w).sum())
        }

        impl<Unit, Number> Trigonometry for MeasurePoint<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Number;

            /// MeasurePoint.cos() -> Number
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }

            /// MeasurePoint.sin() -> Number
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }

            /// MeasurePoint.tan() -> Number
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.tan()
            }

            /// MeasurePoint.sin_cos() -> Number
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        impl<Unit, Number> PartialEq<MeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint == MeasurePoint -> bool
            fn eq(&self, other: &MeasurePoint<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number> PartialOrd<MeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint < MeasurePoint -> bool
            fn partial_cmp(&self, other: &MeasurePoint<Unit, Number>) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number> Clone for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// MeasurePoint.clone() -> Measure
            fn clone(&self) -> Self {
                *self
            }
        }

        /// MeasurePoint = MeasurePoint
        impl<Unit, Number> Copy for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{}", MeasurePoint)
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        impl<Unit, Number> fmt::Debug for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", MeasurePoint)
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
