#[macro_export] // Don't add nor remove the first three lines and the last two lines of this file.
macro_rules! inner_define_measure {
    { $with_approx:ident } => {
        /// 1D relative measure with generic unit of measurement, generic value type,
        /// and with a dynamic value.
        pub struct Measure<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
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

            /// Measure.convert() -> Measure
            pub fn convert<DestUnit>(self) -> Measure<DestUnit, Number>
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
            pub fn lossy_into<DestNumber>(self) -> Measure<Unit, DestNumber>
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
            /// It returns a value forced to be between the given bounds.
            /// `bound1` is not required to be less then or equal to `bound2`.
            pub fn clamp(self, bound1: Self, bound2: Self) -> Self {
                self.max(bound1.min(bound2)).min(bound1.max(bound2))
            }

            /// Measure.decibels_formatter() -> DecibelsMeasureFormatter
            pub const fn decibels_formatter(self) -> DecibelsMeasureFormatter<Unit, Number> {
                DecibelsMeasureFormatter(self)
            }
        }

        impl<Unit, Number> Default for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure::default() -> Measure
            /// It returns the zero measure.
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
            /// Measure<f64>::from(Measure<f32>) -> Measure<f64>
            fn from(measure: Measure<Unit, f32>) -> Self {
                Self::new(measure.value as f64)
            }
        }

        measures::if_all_true! { { $with_approx }
            impl<Unit, Number> From<ApproxMeasure<Unit, Number>> for Measure<Unit, Number>
            where
                Unit: MeasurementUnit,
                Number: ArithmeticOps,
            {
                /// Measure::from(ApproxMeasure) -> Measure
                fn from(am: ApproxMeasure<Unit, Number>) -> Self {
                    Self::new(am.value)
                }
            }
        }

        impl<Unit, Number> Neg for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// -Measure -> Measure
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        impl<Unit, Number> Add<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure + Measure -> Measure
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure += Measure
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        impl<Unit, Number> Sub<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure - Measure -> Measure
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure -= Measure
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        impl<Unit, Number> Mul<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure * Number -> Measure
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        impl<Unit, Number> Mul<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure * Measure<One> -> Measure
            fn mul(self, other: Measure<One, Number>) -> Self::Output {
                Self::new(self.value * other.value)
            }
        }

        impl<Unit, Number> MulAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure *= Number
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure *= Measure<One>
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.value *= other.value;
            }
        }

        impl<Unit> Mul<Measure<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f64>;

            /// f64 * Measure -> Measure
            fn mul(self, other: Measure<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        impl<Unit> Mul<Measure<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f32>;

            /// f32 * Measure -> Measure
            fn mul(self, other: Measure<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        impl<Unit, Number> Div<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure / Number -> Measure
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.value / n)
            }
        }

        impl<Unit, Number> Div<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<One, Number>;

            /// Measure / Measure -> Measure<One>
            fn div(self, other: Measure<Unit, Number>) -> Self::Output {
                Measure::new(self.value / other.value)
            }
        }

        impl<Unit, Number> DivAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure /= Number
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure /= Measure<One>
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

        impl<Unit, Number> PartialEq<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure == Measure -> bool
            fn eq(&self, other: &Measure<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number> PartialOrd<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure < Measure -> bool
            fn partial_cmp(&self, other: &Measure<Unit, Number>) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number> Clone for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure.clone() -> Measure
            fn clone(&self) -> Self {
                *self
            }
        }

        /// Measure = Measure
        impl<Unit, Number> Copy for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{}", Measure) -> String
            /// Measure.to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        impl<Unit, Number> fmt::Debug for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", Measure)
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        /// Wrapper of a `Measure<Unit, Number>`, printing in decibels.
        pub struct DecibelsMeasureFormatter<Unit, Number>(Measure<Unit, Number>)
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps;

        impl<Unit, Number> fmt::Display for DecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{}", Measure.decibels_formatter()) -> String
            /// Measure.decibels_formatter().to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;
                formatter.write_str(" dB")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        impl<Unit, Number> fmt::Debug for DecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", Measure.decibels_formatter())
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;
                formatter.write_str(" dB")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
