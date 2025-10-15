#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure {
    { $with_approx:ident } => {
        pub struct Measure<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure::new(Number) -> Measure
            pub const fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }

            measures::if_all_true! { {$with_approx}
                /// Measure::from_approx_measure(ApproxMeasure) -> Measure
                pub const fn from_approx_measure(approx_measure: ApproxMeasure<Unit, Number>) -> Self {
                    Self::new(approx_measure.value)
                }

                /// Measure.to_approx_measure_with_variance(Number) -> ApproxMeasure
                pub fn to_approx_measure_with_variance(self, variance: Number) -> ApproxMeasure<Unit, Number> {
                    ApproxMeasure::<Unit, Number>::with_variance(self.value, variance)
                }

                /// Measure.to_approx_measure_with_uncertainty(Measure) -> ApproxMeasure
                pub fn to_approx_measure_with_uncertainty(self, uncertainty: Measure<Unit, Number>) -> ApproxMeasure<Unit, Number> {
                    ApproxMeasure::<Unit, Number>::with_uncertainty(self.value, uncertainty)
                }
            }

            /// Measure.convert() -> Measure
            pub fn convert<DestUnit>(&self) -> Measure<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                Measure::<DestUnit, Number>::new(
                    self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO),
                )
            }

            /// Measure.lossless_into() -> Measure
            pub fn lossless_into<DestNumber>(self) -> Measure<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                Measure::<Unit, DestNumber>::new(DestNumber::from(self.value))
            }

            /// Measure.lossy_into() -> Measure
            pub fn lossy_into<DestNumber>(&self) -> Measure<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                Measure::<Unit, DestNumber>::new(DestNumber::lossy_from(self.value))
            }

            /// Measure.norm() -> Measure
            pub fn norm(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.value.abs())
            }

            /// Measure.squared_norm() -> Number
            pub fn squared_norm(self) -> Number {
                self.value * self.value
            }

            /// Measure.normalized() -> Measure
            pub fn normalized(self) -> Self {
                Self::new(self.value.signum())
            }

            /// Measure.min(Measure) -> Measure
            pub fn min(self, other: Self) -> Self {
                if self <= other {
                    self
                } else {
                    other
                }
            }

            /// Measure.max(Measure) -> Measure
            pub fn max(self, other: Self) -> Self {
                if self >= other {
                    self
                } else {
                    other
                }
            }

            /// Measure.clamp(Measure, Measure) -> Measure
            pub fn clamp(self, lower_bound: Self, upper_bound: Self) -> Self {
                self.max(lower_bound).min(upper_bound)
            }

            /// Measure.decibels_formatter() -> DecibelsMeasureFormatter
            pub fn decibels_formatter(self) -> DecibelsMeasureFormatter<Unit, Number> {
                DecibelsMeasureFormatter(self)
            }
        }

        impl<Unit, Number> Default for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        /// The trivial conversions from `Measure<Unit, f32>` to `Measure<Unit, f32>`
        /// and from `Measure<Unit, f64>` to `Measure<Unit, f64>` are provided by the core library.
        /// The lossy conversion from `Measure<Unit, f64>` to `Measure<Unit, f32>`
        /// shouldn't be provided by the trait `From`. Use `Measure.lossy_into()` instead.
        /// This is the lossless conversion from `Measure<Unit, f32>` to `Measure<Unit, f64>`
        impl<Unit> From<Measure<Unit, f32>> for Measure<Unit, f64>
        where
            Unit: MeasurementUnit,
        {
            fn from(measure: Measure<Unit, f32>) -> Self {
                Self::new(measure.value as f64)
            }
        }

        /// -Measure -> Measure
        impl<Unit, Number> Neg for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        /// Measure + Measure -> Measure
        impl<Unit, Number> Add<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        /// Measure += Measure
        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        /// Measure - Measure -> Measure
        impl<Unit, Number> Sub<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        /// Measure -= Measure
        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        /// Measure * Number -> Measure
        impl<Unit, Number> Mul<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        /// Measure * Measure<One> -> Measure
        impl<Unit, Number> Mul<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, other: Measure<One, Number>) -> Self::Output {
                Self::new(self.value * other.value)
            }
        }

        /// Measure *= Number
        impl<Unit, Number> MulAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        /// Measure *= Measure<One>
        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.value *= other.value;
            }
        }

        /// f64 * Measure -> Measure
        impl<Unit> Mul<Measure<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f64>;
            fn mul(self, other: Measure<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        /// f32 * Measure -> Measure
        impl<Unit> Mul<Measure<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f32>;
            fn mul(self, other: Measure<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        /// Measure / Number -> Measure
        impl<Unit, Number> Div<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.value / n)
            }
        }

        /// Measure / Measure -> Measure<One>
        impl<Unit, Number> Div<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<One, Number>;
            fn div(self, other: Measure<Unit, Number>) -> Self::Output {
                Measure::new(self.value / other.value)
            }
        }

        /// Measure /= Number
        impl<Unit, Number> DivAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        /// Measure /= Measure<One>
        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: Measure<One, Number>) {
                self.value /= other.value;
            }
        }

        impl<Unit, Number> Trigonometry for Measure<Unit, Number>
        where
            Unit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Number;

            /// Measure.cos() -> Number
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }

            /// Measure.sin() -> Number
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }

            /// Measure.tan() -> Number
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.tan()
            }

            /// Measure.sin_cos() -> (Number, Number)
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        /// Measure == Measure -> bool
        impl<Unit, Number> PartialEq<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &Measure<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        /// Measure < Measure -> bool
        impl<Unit, Number> PartialOrd<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Number: ArithmeticOps,
        {
            fn partial_cmp(&self, other: &Measure<Unit, Number>) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        /// Measure.clone() -> Measure
        impl<Unit, Number> Clone for Measure<Unit, Number>
        where
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        /// Measure = Measure
        impl<Unit, Number> Copy for Measure<Unit, Number> where Number: ArithmeticOps {}

        /// format!("{}", Measure)
        impl<Unit, Number> fmt::Display for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        /// format!("{:?}", Measure)
        impl<Unit, Number> fmt::Debug for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        pub struct DecibelsMeasureFormatter<Unit, Number>(Measure<Unit, Number>);

        /// format!("{}", Measure.decibels_formatter())
        impl<Unit, Number> fmt::Display for DecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;
                formatter.write_str(" dB")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        /// format!("{:?}", Measure.decibels_formatter())
        impl<Unit, Number> fmt::Debug for DecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;
                formatter.write_str(" dB")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
