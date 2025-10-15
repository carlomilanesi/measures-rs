#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure_3d {
    {$with_approx:ident} => {
        /// Approximate 3d measurement with static unit of measurement and value type,
        /// and with dynamic values and variance.
        pub struct ApproxMeasure3d<Unit, Number: ArithmeticOps = f64> {
            pub values: [Number; 3],
            pub variance: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Unit, Number> ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// Measure3d::with_variance(number, number, number, number) -> Measure3d
            pub const fn with_variance(values: [Number; 3], variance: Number) -> Self {
                Self {
                    values,
                    variance,
                    phantom: PhantomData,
                }
            }

            /// Measure3d::with_uncertainty(number, number, number, number) -> Measure3d
            pub fn with_uncertainty(values: [Number; 3], uncertainty: Measure<Unit, Number>) -> Self {
                Self::with_variance(values, uncertainty.value * uncertainty.value)
            }

            /// Measure3d.x() -> Measure
            pub fn x(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(
                    self.values[0],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.y() -> Measure
            pub fn y(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(
                    self.values[1],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.z() -> Measure
            pub fn z(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(
                    self.values[2],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.convert() -> Measure3d
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> ApproxMeasure3d<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                ApproxMeasure3d::<DestUnit, Number>::with_variance(
                    [
                        self.values[0] * ratio,
                        self.values[1] * ratio,
                        self.values[2] * ratio,
                    ],
                    self.variance * ratio * ratio,
                )
            }

            /// ApproxMeasure3d.lossless_into() -> ApproxMeasure3d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> ApproxMeasure3d<Unit, DestNumber> {
                ApproxMeasure3d::<Unit, DestNumber>::with_variance(
                    [
                        DestNumber::from(self.values[0]),
                        DestNumber::from(self.values[1]),
                        DestNumber::from(self.values[2]),
                    ],
                    DestNumber::from(self.variance),
                )
            }

            /// ApproxMeasure3d.lossy_into() -> ApproxMeasure3d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasure3d<Unit, DestNumber> {
                ApproxMeasure3d::<Unit, DestNumber>::with_variance(
                    [
                        DestNumber::lossy_from(self.values[0]),
                        DestNumber::lossy_from(self.values[1]),
                        DestNumber::lossy_from(self.values[2]),
                    ],
                    DestNumber::lossy_from(self.variance),
                )
            }

            /// Measure3d.squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.values[0] * self.values[0]
                    + self.values[1] * self.values[1]
                    + self.values[2] * self.values[2]
            }

            /// Measure3d.normalized() -> Measure3d
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::with_variance(
                    [self.values[0] * k, self.values[1] * k, self.values[2] * k],
                    self.variance * k * k,
                )
            }
        }

        impl<Unit, Number> Default for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::with_variance([Number::ZERO, Number::ZERO, Number::ZERO], Number::ZERO)
            }
        }

        impl<Unit> From<ApproxMeasure3d<Unit, f32>> for ApproxMeasure3d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: ApproxMeasure3d<Unit, f32>) -> Self {
                Self::with_variance(
                    [m.values[0] as f64, m.values[1] as f64, m.values[2] as f64],
                    m.variance as f64,
                )
            }
        }

        /// -ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Neg for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::with_variance(
                    [-self.values[0], -self.values[1], -self.values[2]],
                    self.variance,
                )
            }
        }

        /// ApproxMeasure3d + ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Add<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::with_variance(
                    [
                        self.values[0] + other.values[0],
                        self.values[1] + other.values[1],
                        self.values[2] + other.values[2],
                    ],
                    self.variance + other.variance,
                )
            }
        }

        /// ApproxMeasure3d += ApproxMeasure3d
        impl<Unit, Number> AddAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
                self.values[2] += other.values[2];
                self.variance += other.variance;
            }
        }

        /// ApproxMeasure3d - ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Sub<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::with_variance(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                        self.values[2] - other.values[2],
                    ],
                    self.variance + other.variance,
                )
            }
        }

        /// ApproxMeasure3d -= ApproxMeasure3d
        impl<Unit, Number> SubAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
                self.values[2] -= other.values[2];
                self.variance += other.variance;
            }
        }

        /// ApproxMeasure3d * Number -> ApproxMeasure3d
        impl<Unit, Number> Mul<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::with_variance(
                    [self.values[0] * n, self.values[1] * n, self.values[2] * n],
                    self.variance * n * n,
                )
            }
        }

        /// ApproxMeasure3d * ApproxMeasure<One> -> ApproxMeasure3d
        impl<Unit, Number> Mul<ApproxMeasure<One, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, other: ApproxMeasure<One, Number>) -> Self::Output {
                Self::with_variance(
                    [
                        self.values[0] * other.value,
                        self.values[1] * other.value,
                        self.values[2] * other.value,
                    ],
                    self.values[0] * self.values[0] * other.variance
                        + other.value * other.value * self.variance,
                )
            }
        }

        /// ApproxMeasure<One> * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Mul<ApproxMeasure3d<Unit, Number>> for ApproxMeasure<One, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, Number>;
            fn mul(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::Output::with_variance(
                    [
                        self.value * other.values[0],
                        self.value * other.values[1],
                        self.value * other.values[2],
                    ],
                    self.value * self.value * other.variance
                        + other.values[0] * other.values[0] * self.variance,
                )
            }
        }

        /// ApproxMeasure3d *= Number
        impl<Unit, Number> MulAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, n: Number) {
                self.values[0] *= n;
                self.values[1] *= n;
                self.values[2] *= n;
                self.variance *= n * n;
            }
        }

        /// ApproxMeasure3d *= ApproxMeasure<One>
        impl<Unit, Number> MulAssign<ApproxMeasure<One, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, other: ApproxMeasure<One, Number>) {
                self.values[0] *= other.value;
                self.values[1] *= other.value;
                self.values[2] *= other.value;
                self.variance = self.values[0] * self.values[0] * other.variance
                    + other.value * other.value * self.variance;
            }
        }

        /// f64 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, f64>;
            fn mul(self, other: ApproxMeasure3d<Unit, f64>) -> Self::Output {
                Self::Output::with_variance(
                    [
                        self * other.values[0],
                        self * other.values[1],
                        self * other.values[2],
                    ],
                    self * self * other.variance,
                )
            }
        }

        /// f32 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, f32>;
            fn mul(self, other: ApproxMeasure3d<Unit, f32>) -> Self::Output {
                Self::Output::with_variance(
                    [
                        self * other.values[0],
                        self * other.values[1],
                        self * other.values[2],
                    ],
                    self * self * other.variance,
                )
            }
        }

        /// ApproxMeasure3d / Number -> ApproxMeasure3d
        impl<Unit, Number> Div<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::with_variance(
                    [
                        self.values[0] * factor,
                        self.values[1] * factor,
                        self.values[2] * factor,
                    ],
                    self.variance * factor * factor,
                )
            }
        }

        /// ApproxMeasure3d /= Number
        impl<Unit, Number> DivAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn div_assign(&mut self, n: Number) {
                let factor = Number::ONE / n;
                self.values[0] *= factor;
                self.values[1] *= factor;
                self.values[2] *= factor;
                self.variance *= factor * factor;
            }
        }

        /// ApproxMeasure3d / ApproxMeasure -> ApproxMeasure3d<One>
        impl<Unit, Number> Div<ApproxMeasure<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<One, Number>;
            fn div(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                let self_ratio = self.variance / (self.values[0] * self.values[0]);
                let other_ratio = other.variance / (other.value * other.value);
                let value_ratio = self.values[0] / other.value;
                Self::Output::with_variance(
                    [
                        self.values[0] / other.value,
                        self.values[1] / other.value,
                        self.values[2] / other.value,
                    ],
                    value_ratio * value_ratio * (self_ratio + other_ratio),
                )
            }
        }

        /// ApproxMeasure3d /= ApproxMeasure<One>
        impl<Unit, Number> DivAssign<ApproxMeasure<One, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: ApproxMeasure<One, Number>) {
                self.values[0] /= other.value;
                self.values[1] /= other.value;
                self.values[2] /= other.value;
                let self_ratio = self.variance / (self.values[0] * self.values[0]);
                let other_ratio = other.variance / (other.value * other.value);
                let value_ratio = self.values[0] / other.value;
                self.variance = value_ratio * value_ratio * (self_ratio + other_ratio);
            }
        }

        /// ApproxMeasure3d == ApproxMeasure3d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &ApproxMeasure3d<Unit, Number>) -> bool {
                self.values == other.values && self.variance == other.variance
            }
        }

        /// ApproxMeasure3d.clone() -> ApproxMeasure3d
        impl<Unit, Number> Clone for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        /// ApproxMeasure3d = ApproxMeasure3d
        impl<Unit, Number> Copy for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        /// format!("{}", ApproxMeasure3d)
        impl<Unit, Number> fmt::Display for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[2], formatter)?;
                formatter.write_str(")\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        /// format!("{:?}", ApproxMeasure3d)
        impl<Unit, Number> fmt::Debug for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[2], formatter)?;
                formatter.write_str(")\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
