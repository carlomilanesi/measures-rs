#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure {
    {$exact:tt} => {
        /// Approximate measurement with static unit of measurement and value type,
        /// and with dynamic value and variance.
        pub struct ApproxMeasure<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub value: Number,
            pub variance: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// ApproxMeasure::with_variance(Number, Number) -> ApproxMeasure
            pub const fn with_variance(value: Number, variance: Number) -> Self {
                Self {
                    value,
                    variance,
                    phantom: PhantomData,
                }
            }

            measures::if_all_true! { {$exact}
                /// ApproxMeasure::with_uncertainty(Number, Measure) -> ApproxMeasure
                pub fn with_uncertainty(value: Number, uncertainty: Measure<Unit, Number>) -> Self {
                    Self {
                        value,
                        variance: uncertainty.value * uncertainty.value,
                        phantom: PhantomData,
                    }
                }

                /// ApproxMeasure.uncertainty() -> Measure
                pub fn uncertainty(self) -> Measure<Unit, Number> {
                    Measure::<Unit, Number>::new(self.variance.sqrt())
                }

                /// ApproxMeasure::from_measure_with_variance(Measure, Number) -> ApproxMeasure
                pub fn from_measure_with_variance(measure: Measure<Unit, Number>, variance: Number) -> Self {
                    Self::with_variance(measure.value, variance)
                }

                /// ApproxMeasure::from_measure_with_uncertainty(Measure, Measure) -> ApproxMeasure
                pub fn from_measure_with_uncertainty(measure: Measure<Unit, Number>, uncertainty: Measure<Unit, Number>) -> Self {
                    Self::with_uncertainty(measure.value, uncertainty)
                }

                /// ApproxMeasure.to_measure() -> Measure
                pub const fn to_measure(self) -> Measure<Unit, Number> {
                    Measure::<Unit, Number>::new(self.value)
                }
            }

            /// ApproxMeasure.convert() -> ApproxMeasure
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                self,
            ) -> ApproxMeasure<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                ApproxMeasure::<DestUnit, Number>::with_variance(
                    self.value * ratio,
                    self.variance * (ratio * ratio),
                )
            }

            /// ApproxMeasure.lossless_into() -> ApproxMeasure
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> ApproxMeasure<Unit, DestNumber> {
                ApproxMeasure::<Unit, DestNumber>::with_variance(
                    DestNumber::from(self.value),
                    DestNumber::from(self.variance),
                )
            }

            /// ApproxMeasure.lossy_into() -> ApproxMeasure
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                self,
            ) -> ApproxMeasure<Unit, DestNumber> {
                ApproxMeasure::<Unit, DestNumber>::with_variance(
                    DestNumber::lossy_from(self.value),
                    DestNumber::lossy_from(self.variance),
                )
            }

            /// ApproxMeasure.norm() -> ApproxMeasure
            /// The norm of a scalar is the absolute value of its value.
            /// The variance remains the same.
            pub fn norm(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(self.value.abs(), self.variance)
            }

            /// ApproxMeasure.squared_norm() -> ApproxMeasure<One>
            /// The squared norm of a scalar is the square of its value.
            /// For the variance, the correlation term is one, because
            /// the factors of the product are the same.
            pub fn squared_norm(self) -> ApproxMeasure<One, Number> {
                let value_product = self.value * self.value;
                ApproxMeasure::<One, Number>::with_variance(
                    value_product,
                    (Number::ONE + Number::ONE + Number::ONE + Number::ONE) * value_product * self.variance,
                )
            }

            /// ApproxMeasure.normalized() -> ApproxMeasure
            pub fn normalized(self) -> Self {
                Self::with_variance(
                    self.value.signum(),
                    self.variance / (self.value * self.value),
                )
            }

            pub fn decibels_formatter(self) -> ApproxDecibelsMeasureFormatter<Unit, Number> {
                ApproxDecibelsMeasureFormatter(self)
            }
        }

        impl<Unit, Number> Default for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// It returns the zero measure.
            fn default() -> Self {
                Self::with_variance(Number::ZERO, Number::ZERO)
            }
        }

        impl<Unit> From<ApproxMeasure<Unit, f32>> for ApproxMeasure<Unit, f64>
        where
            Unit: MeasurementUnit,
        {
            fn from(m: ApproxMeasure<Unit, f32>) -> Self {
                Self::with_variance(m.value as f64, m.variance as f64)
            }
        }

        /// -ApproxMeasure -> ApproxMeasure
        /// The negation does not change the variance.
        impl<Unit, Number> Neg for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::with_variance(-self.value, self.variance)
            }
        }

        /// ApproxMeasure + ApproxMeasure -> ApproxMeasure
        /// Assuming statistical independence,
        /// the uncertainty is summed in quadrature,
        /// and so the variance is the sum of the variances.
        impl<Unit, Number> Add<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                Self::with_variance(self.value + other.value, self.variance + other.variance)
            }
        }

        // ApproxMeasure.add_with_correlation(ApproxMeasure, Number) -> ApproxMeasure
        // The general case has correlation between -1 and 1:
        //     Given e = 1 - correlation^2 / 2;
        //     variance is (v1^e + v2^e)^(1 / e)
        // In particular:
        // Case of independence: correlation = 0
        //     And the formulas are:
        //     Given e = 1 - 0^2 / 2 = 1;
        //     variance is (v1^1 + v2^1)^(1 / 1) = (v1 + v2)^1 = v1 + v2
        //     i.e.: variance = self.variance + other.variance
        // Case of full correlation: correlation = 1
        //     And the formulas are:
        //     Given e = 1 - 1^2 / 2 = 0.5;
        //     variance is (v1^0.5 + v2^0.5)^(1 / 0.5) = (v1.sqrt() + v2.sqrt())^2
        //     i.e.: variance = (self.variance.sqrt() + other.variance.sqrt()).squared()
        //     i.e.: uncertainty = variance.sqrt() = (self.uncertainty() + other.uncertainty()).squared().sqrt()
        //     = self.uncertainty() + other.uncertainty()
        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_with_correlation(self, other: ApproxMeasure<Unit, Number>, correlation: Number) -> Self {
                Self::with_variance(
                    self.value + other.value,
                    self.variance
                        + other.variance
                        + (Number::ONE + Number::ONE)
                            * correlation
                            * self.variance.sqrt()
                            * other.variance.sqrt(),
                )
            }
        }

        // ApproxMeasure += ApproxMeasure
        impl<Unit, Number> AddAssign<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: ApproxMeasure<Unit, Number>) {
                self.value += other.value;
                self.variance += other.variance;
            }
        }

        // ApproxMeasure.add_assign_with_correlation(ApproxMeasure, Number)
        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign_with_correlation(
                &mut self,
                other: ApproxMeasure<Unit, Number>,
                correlation: Number,
            ) {
                self.value += other.value;
                self.variance += other.variance
                    + (Number::ONE + Number::ONE)
                        * correlation
                        * self.variance.sqrt()
                        * other.variance.sqrt();
            }
        }

        // ApproxMeasure - ApproxMeasure -> ApproxMeasure
        // For subtractions, the variances are handled as for additions.
        impl<Unit, Number> Sub<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                Self::with_variance(self.value - other.value, self.variance + other.variance)
            }
        }

        // ApproxMeasure.sub_with_correlation(ApproxMeasure, Number) -> ApproxMeasure
        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_with_correlation(self, other: ApproxMeasure<Unit, Number>, correlation: Number) -> Self {
                Self::with_variance(
                    self.value - other.value,
                    self.variance
                        + other.variance
                        + (Number::ONE + Number::ONE)
                            * correlation
                            * self.variance.sqrt()
                            * other.variance.sqrt(),
                )
            }
        }

        // ApproxMeasure -= ApproxMeasure
        impl<Unit, Number> SubAssign<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: ApproxMeasure<Unit, Number>) {
                self.value -= other.value;
                self.variance += other.variance;
            }
        }

        // ApproxMeasure.sub_assign_with_correlation(ApproxMeasure, Number)
        // For subtractions, the variances are handled as for additions.
        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign_with_correlation(
                &mut self,
                other: ApproxMeasure<Unit, Number>,
                correlation: Number,
            ) {
                self.value -= other.value;
                self.variance -= other.variance
                    + (Number::ONE + Number::ONE)
                        * correlation
                        * self.variance.sqrt()
                        * other.variance.sqrt();
            }
        }

        // ApproxMeasure * Number -> ApproxMeasure
        impl<Unit, Number> Mul<Number> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::with_variance(self.value * n, self.variance * (n * n))
            }
        }

        /// ApproxMeasure * ApproxMeasure<One> -> ApproxMeasure
        impl<Unit, Number> Mul<ApproxMeasure<One, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, other: ApproxMeasure<One, Number>) -> Self::Output {
                Self::with_variance(
                    self.value * other.value,
                    self.value * self.value * other.variance + other.value * other.value * self.variance,
                )
            }
        }

        // ApproxMeasure *= Number
        impl<Unit, Number> MulAssign<Number> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
                self.variance *= n * n;
            }
        }

        /// ApproxMeasure *= ApproxMeasure<One>
        impl<Unit, Number> MulAssign<ApproxMeasure<One, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, other: ApproxMeasure<One, Number>) {
                self.value *= other.value;
                self.variance =
                    self.value * self.value * other.variance + other.value * other.value * self.variance;
            }
        }

        // f64 * ApproxMeasure -> ApproxMeasure
        impl<Unit> Mul<ApproxMeasure<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
        {
            type Output = ApproxMeasure<Unit, f64>;
            fn mul(self, other: ApproxMeasure<Unit, f64>) -> Self::Output {
                Self::Output::with_variance(self * other.value, self * self * other.variance)
            }
        }

        // f32 * ApproxMeasure -> ApproxMeasure
        impl<Unit> Mul<ApproxMeasure<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
        {
            type Output = ApproxMeasure<Unit, f32>;
            fn mul(self, other: ApproxMeasure<Unit, f32>) -> Self::Output {
                Self::Output::with_variance(self * other.value, self * self * other.variance)
            }
        }

        // ApproxMeasure / Number -> ApproxMeasure
        impl<Unit, Number> Div<Number> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::with_variance(self.value / n, self.variance / (n * n))
            }
        }

        // ApproxMeasure / ApproxMeasure -> ApproxMeasure<One>
        impl<Unit, Number> Div<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure<One, Number>;
            fn div(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                let self_ratio = self.variance / (self.value * self.value);
                let other_ratio = other.variance / (other.value * other.value);
                let value_ratio = self.value / other.value;
                ApproxMeasure::<One, Number>::with_variance(
                    value_ratio,
                    value_ratio * value_ratio * (self_ratio + other_ratio),
                )
            }
        }

        // ApproxMeasure.div_with_correlation(ApproxMeasure, Number) -> ApproxMeasure<One>
        impl<Unit, Number> ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_with_correlation(
                self,
                other: ApproxMeasure<Unit, Number>,
                correlation: Number,
            ) -> ApproxMeasure<One, Number> {
                ApproxMeasure::<One, Number>::with_variance(
                    self.value / other.value,
                    self.variance / (other.value * other.value)
                        + (self.value * self.value)
                            / ((other.value * other.value) * (other.value * other.value))
                            * other.variance
                        - (Number::ONE + Number::ONE) * self.value
                            / (other.value * other.value * other.value)
                            * correlation
                            * self.variance.sqrt()
                            * other.variance.sqrt(),
                )
            }
        }

        // ApproxMeasure /= Number
        impl<Unit, Number> DivAssign<Number> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
                self.variance /= n * n;
            }
        }

        /// Measure /= Measure<One>
        impl<Unit, Number> DivAssign<ApproxMeasure<One, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: ApproxMeasure<One, Number>) {
                let self_ratio = self.variance / (self.value * self.value);
                let other_ratio = other.variance / (other.value * other.value);
                let value_ratio = self.value / other.value;
                self.value = value_ratio;
                self.variance = value_ratio * value_ratio * (self_ratio + other_ratio);
            }
        }

        // ApproxMeasure == ApproxMeasure -> bool
        impl<Unit, Number> PartialEq<ApproxMeasure<Unit, Number>> for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &ApproxMeasure<Unit, Number>) -> bool {
                self.value == other.value && self.variance == other.variance
            }
        }

        // ApproxMeasure.clone() -> ApproxMeasure
        impl<Unit, Number> Clone for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // ApproxMeasure = ApproxMeasure
        impl<Unit, Number> Copy for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        // format!("{}", ApproxMeasure)
        impl<Unit, Number> fmt::Display for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", ApproxMeasure)
        impl<Unit, Number> fmt::Debug for ApproxMeasure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("[\u{b5}=")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(" \u{3c3}\u{b2}=")?;
                fmt::Display::fmt(&self.variance, formatter)?;
                formatter.write_str("]")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        pub struct ApproxDecibelsMeasureFormatter<Unit, Number>(ApproxMeasure<Unit, Number>)
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps;

        // format!("{}", ApproxMeasure.decibels_formatter())
        impl<Unit, Number> fmt::Display for ApproxDecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;

                // It is plus or minus the square root of the variance, in dB,
                // that is the variance in dB divided by two.
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&(self.0.variance.to_decibels() * Number::HALF), formatter)?;
                formatter.write_str(" dB")?;

                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", ApproxMeasure.decibels_formatter())
        impl<Unit, Number> fmt::Debug for ApproxDecibelsMeasureFormatter<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0.value.to_decibels(), formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&(self.0.variance.to_decibels() * Number::HALF), formatter)?;
                formatter.write_str(" dB")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
