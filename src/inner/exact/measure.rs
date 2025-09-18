#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure {
    {$with_approx:ident} => {
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
                    phantom: PhantomData::<Unit>,
                }
            }

            measures::if_all_true! { {$with_approx}
                pub const fn from_approx_measure(approx_measure: ApproxMeasure<Unit, Number>) -> Self {
                    Self::new(approx_measure.value)
                }
            }

            measures::if_all_true! { {$with_approx}
                pub fn to_approx_measure_with_variance(self, variance: Number) -> ApproxMeasure<Unit, Number> {
                    ApproxMeasure::<Unit, Number>::new_with_variance(self.value, variance)
                }

                pub fn to_approx_measure_with_uncertainty(self, uncertainty: Measure<Unit, Number>) -> ApproxMeasure<Unit, Number> {
                    ApproxMeasure::<Unit, Number>::new_with_uncertainty(self.value, uncertainty)
                }
            }

            /// Measure.convert() -> Measure
            pub fn convert<DestUnit>(&self) -> Measure<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                Measure::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            // Measure.lossless_into() -> Measure
            pub fn lossless_into<DestNumber>(&self) -> Measure<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                Measure::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }

            // Measure.lossy_into() -> Measure
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure<Unit, DestNumber> {
                Measure::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }

            // Measure.squared_norm() -> Measure<One> {
            pub fn squared_norm(self) -> Measure<One, Number> {
                Measure::<One, Number>::new(self.value * self.value)
            }

            /// Measure.normalized() -> Number
            pub fn normalized(self) -> Self {
                Self::new(self.value.signum())
            }

            pub fn min(self, other: Self) -> Self {
                if self <= other {
                    self
                } else {
                    other
                }
            }

            pub fn max(self, other: Self) -> Self {
                if self >= other {
                    self
                } else {
                    other
                }
            }

            pub fn clamp(self, lower_bound: Self, upper_bound: Self) -> Self {
                self.max(lower_bound).min(upper_bound)
            }

            pub fn format_in_decibels(self) -> DecibelFormattedMeasure<Unit, Number> {
                DecibelFormattedMeasure(self)
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

        // -Measure -> Measure
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

        // Measure + Measure -> Measure
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

        // Measure += Measure
        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // Measure - Measure -> Measure
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

        // Measure -= Measure
        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // Measure * Number -> Measure
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

        // Measure * Measure<One> -> Measure
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

        // Measure *= Number
        impl<Unit, Number> MulAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        // Measure *= Measure<One, Number>
        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.value *= other.value;
            }
        }

        // f64 * Measure -> Measure
        impl<Unit> Mul<Measure<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f64>;
            fn mul(self, other: Measure<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // f32 * Measure -> Measure
        impl<Unit> Mul<Measure<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
        {
            type Output = Measure<Unit, f32>;
            fn mul(self, other: Measure<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // Measure / Number -> Measure
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

        // Measure / Measure -> Measure<One>
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

        // Measure /= Number
        impl<Unit, Number> DivAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        // Measure /= Measure<One>
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
            fn cos(self) -> Self::Output {
                self.convert::<Radian>().value.cos()
            }
            fn sin(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }
            fn tan(self) -> Self::Output {
                self.convert::<Radian>().value.sin()
            }
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                self.convert::<Radian>().value.sin_cos()
            }
        }

        // Measure == Measure -> bool
        impl<Unit, Number> PartialEq<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &Measure<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        // Measure < Measure -> bool
        impl<Unit, Number> PartialOrd<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn partial_cmp(&self, other: &Measure<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        // Measure.clone() -> Measure
        impl<Unit, Number> Clone for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure = Measure
        impl<Unit, Number> Copy for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        // format!("{}", Measure)
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

        // format!("{:?}", Measure)
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

        pub struct DecibelFormattedMeasure<Unit: MeasurementUnit, Number: ArithmeticOps>(
            Measure<Unit, Number>,
        );

        // format!("{}", Measure.format_in_decibels())
        impl<Unit, Number> fmt::Display for DecibelFormattedMeasure<Unit, Number>
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

        // format!("{:?}", Measure.format_in_decibels())
        impl<Unit, Number> fmt::Debug for DecibelFormattedMeasure<Unit, Number>
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
