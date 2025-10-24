#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure_3d {
    { $with_approx:ident } => {
        /// Approximate 3d relative measure, with generic unit of measurement and value type,
        /// and with dynamic values, variances, and covariances.
        pub struct ApproxMeasure3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub values: [Number; 3],
            pub covariances: [[Number; 3]; 3],
            phantom: core::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d::with_covariances([number; 3], [[number; 3]; 3]) -> Measure3d
            pub const fn with_covariances(values: [Number; 3], covariances: [[Number; 3]; 3]) -> Self {
                Self {
                    values,
                    covariances,
                    phantom: PhantomData,
                }
            }

            /// Measure3d::with_uncertainty([number; 3], Measure) -> Measure3d
            pub fn with_uncertainty(values: [Number; 3], uncertainty: Measure<Unit, Number>) -> Self {
                let v = uncertainty.value * uncertainty.value;
                Self::with_covariances(
                    values,
                    [
                        [v, Number::ZERO, Number::ZERO],
                        [Number::ZERO, v, Number::ZERO],
                        [Number::ZERO, Number::ZERO, v],
                    ],
                )
            }

            /// Measure3d.x() -> Measure
            pub fn x(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(self.values[0], self.covariances[0][0])
            }

            /// Measure3d.y() -> Measure
            pub fn y(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(self.values[1], self.covariances[1][1])
            }

            /// Measure3d.z() -> Measure
            pub fn z(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::with_variance(self.values[2], self.covariances[2][2])
            }

            /// Measure3d.convert() -> Measure3d
            pub fn convert<DestUnit>(self) -> ApproxMeasure3d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                debug_assert!(Unit::OFFSET == 0.);
                debug_assert!(DestUnit::OFFSET == 0.);
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let r2 = ratio * ratio;
                ApproxMeasure3d::<DestUnit, Number>::with_covariances(
                    [
                        self.values[0] * ratio,
                        self.values[1] * ratio,
                        self.values[2] * ratio,
                    ],
                    [
                        [
                            self.covariances[0][0] * r2,
                            self.covariances[0][1] * r2,
                            self.covariances[0][2] * r2,
                        ],
                        [
                            self.covariances[1][0] * r2,
                            self.covariances[1][1] * r2,
                            self.covariances[1][2] * r2,
                        ],
                        [
                            self.covariances[2][0] * r2,
                            self.covariances[2][1] * r2,
                            self.covariances[2][2] * r2,
                        ],
                    ],
                )
            }

            /// ApproxMeasure3d.lossless_into() -> ApproxMeasure3d
            pub fn lossless_into<DestNumber>(self) -> ApproxMeasure3d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                ApproxMeasure3d::<Unit, DestNumber>::with_covariances(
                    [
                        self.values[0].into(),
                        self.values[1].into(),
                        self.values[2].into(),
                    ],
                    [
                        [
                            self.covariances[0][0].into(),
                            self.covariances[0][1].into(),
                            self.covariances[0][2].into(),
                        ],
                        [
                            self.covariances[1][0].into(),
                            self.covariances[1][1].into(),
                            self.covariances[1][2].into(),
                        ],
                        [
                            self.covariances[2][0].into(),
                            self.covariances[2][1].into(),
                            self.covariances[2][2].into(),
                        ],
                    ],
                )
            }

            /// ApproxMeasure3d.lossy_into() -> ApproxMeasure3d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasure3d<Unit, DestNumber> {
                ApproxMeasure3d::<Unit, DestNumber>::with_covariances(
                    [
                        DestNumber::lossy_from(self.values[0]),
                        DestNumber::lossy_from(self.values[1]),
                        DestNumber::lossy_from(self.values[2]),
                    ],
                    [
                        [
                            DestNumber::lossy_from(self.covariances[0][0]),
                            DestNumber::lossy_from(self.covariances[0][1]),
                            DestNumber::lossy_from(self.covariances[0][2]),
                        ],
                        [
                            DestNumber::lossy_from(self.covariances[1][0]),
                            DestNumber::lossy_from(self.covariances[1][1]),
                            DestNumber::lossy_from(self.covariances[1][2]),
                        ],
                        [
                            DestNumber::lossy_from(self.covariances[2][0]),
                            DestNumber::lossy_from(self.covariances[2][1]),
                            DestNumber::lossy_from(self.covariances[2][2]),
                        ],
                    ],
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
                Self::with_covariances(
                    [self.values[0] * k, self.values[1] * k, self.values[2] * k],
                    [
                        [
                            self.covariances[0][0] * k * k,
                            self.covariances[0][1] * k * k,
                            self.covariances[0][2] * k * k,
                        ],
                        [
                            self.covariances[1][0] * k * k,
                            self.covariances[1][1] * k * k,
                            self.covariances[1][2] * k * k,
                        ],
                        [
                            self.covariances[2][0] * k * k,
                            self.covariances[2][1] * k * k,
                            self.covariances[2][2] * k * k,
                        ],
                    ],
                )
            }
        }

        impl<Unit, Number> Default for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// ApproxMeasure3d::default() -> ApproxMeasure3d
            /// It returns the zero vector, with no uncertainty.
            fn default() -> Self {
                Self::with_covariances(
                    [Number::ZERO, Number::ZERO, Number::ZERO],
                    [
                        [Number::ZERO, Number::ZERO, Number::ZERO],
                        [Number::ZERO, Number::ZERO, Number::ZERO],
                        [Number::ZERO, Number::ZERO, Number::ZERO],
                    ],
                )
            }
        }

        impl<Unit> From<ApproxMeasure3d<Unit, f32>> for ApproxMeasure3d<Unit, f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            fn from(m: ApproxMeasure3d<Unit, f32>) -> Self {
                Self::with_covariances(
                    [m.values[0] as f64, m.values[1] as f64, m.values[2] as f64],
                    [
                        [
                            m.covariances[0][0] as f64,
                            m.covariances[0][1] as f64,
                            m.covariances[0][2] as f64,
                        ],
                        [
                            m.covariances[1][0] as f64,
                            m.covariances[1][1] as f64,
                            m.covariances[1][2] as f64,
                        ],
                        [
                            m.covariances[2][0] as f64,
                            m.covariances[2][1] as f64,
                            m.covariances[2][2] as f64,
                        ],
                    ],
                )
            }
        }

        /// -ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Neg for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::with_covariances(
                    [-self.values[0], -self.values[1], -self.values[2]],
                    self.covariances,
                )
            }
        }

        /// ApproxMeasure3d + ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Add<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::with_covariances(
                    [
                        self.values[0] + other.values[0],
                        self.values[1] + other.values[1],
                        self.values[2] + other.values[2],
                    ],
                    [
                        [
                            self.covariances[0][0] + other.covariances[0][0],
                            self.covariances[0][1] + other.covariances[0][1],
                            self.covariances[0][2] + other.covariances[0][2],
                        ],
                        [
                            self.covariances[1][0] + other.covariances[1][0],
                            self.covariances[1][1] + other.covariances[1][1],
                            self.covariances[1][2] + other.covariances[1][2],
                        ],
                        [
                            self.covariances[2][0] + other.covariances[2][0],
                            self.covariances[2][1] + other.covariances[2][1],
                            self.covariances[2][2] + other.covariances[2][2],
                        ],
                    ],
                )
            }
        }

        /// ApproxMeasure3d += ApproxMeasure3d
        impl<Unit, Number> AddAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
                self.values[2] += other.values[2];
                self.covariances[0][0] += other.covariances[0][0];
                self.covariances[0][1] += other.covariances[0][1];
                self.covariances[0][2] += other.covariances[0][2];
                self.covariances[1][0] += other.covariances[1][0];
                self.covariances[1][1] += other.covariances[1][1];
                self.covariances[1][2] += other.covariances[1][2];
                self.covariances[2][0] += other.covariances[2][0];
                self.covariances[2][1] += other.covariances[2][1];
                self.covariances[2][2] += other.covariances[2][2];
            }
        }

        /// ApproxMeasure3d - ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Sub<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::with_covariances(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                        self.values[2] - other.values[2],
                    ],
                    [
                        [
                            self.covariances[0][0] + other.covariances[0][0],
                            self.covariances[0][1] + other.covariances[0][1],
                            self.covariances[0][2] + other.covariances[0][2],
                        ],
                        [
                            self.covariances[1][0] + other.covariances[1][0],
                            self.covariances[1][1] + other.covariances[1][1],
                            self.covariances[1][2] + other.covariances[1][2],
                        ],
                        [
                            self.covariances[2][0] + other.covariances[2][0],
                            self.covariances[2][1] + other.covariances[2][1],
                            self.covariances[2][2] + other.covariances[2][2],
                        ],
                    ],
                )
            }
        }

        /// ApproxMeasure3d -= ApproxMeasure3d
        impl<Unit, Number> SubAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
                self.values[2] -= other.values[2];
                self.covariances[0][0] += other.covariances[0][0];
                self.covariances[0][1] += other.covariances[0][1];
                self.covariances[0][2] += other.covariances[0][2];
                self.covariances[1][0] += other.covariances[1][0];
                self.covariances[1][1] += other.covariances[1][1];
                self.covariances[1][2] += other.covariances[1][2];
                self.covariances[2][0] += other.covariances[2][0];
                self.covariances[2][1] += other.covariances[2][1];
                self.covariances[2][2] += other.covariances[2][2];
            }
        }

        /// ApproxMeasure3d * Number -> ApproxMeasure3d
        impl<Unit, Number> Mul<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                let n2 = n * n;
                Self::with_covariances(
                    [self.values[0] * n, self.values[1] * n, self.values[2] * n],
                    [
                        [
                            self.covariances[0][0] * n2,
                            self.covariances[0][1] * n2,
                            self.covariances[0][2] * n2,
                        ],
                        [
                            self.covariances[1][0] * n2,
                            self.covariances[1][1] * n2,
                            self.covariances[1][2] * n2,
                        ],
                        [
                            self.covariances[2][0] * n2,
                            self.covariances[2][1] * n2,
                            self.covariances[2][2] * n2,
                        ],
                    ],
                )
            }
        }

        /// ApproxMeasure3d * ApproxMeasure<One> -> ApproxMeasure3d
        impl<Unit, Number> Mul<ApproxMeasure<One, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, other: ApproxMeasure<One, Number>) -> Self::Output {
                Self::with_covariances(
                    [
                        self.values[0] * other.value,
                        self.values[1] * other.value,
                        self.values[2] * other.value,
                    ],
                    self.covariances,
                )
            }
        }

        /// ApproxMeasure<One> * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Mul<ApproxMeasure3d<Unit, Number>> for ApproxMeasure<One, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure3d<Unit, Number>;
            fn mul(self, other: ApproxMeasure3d<Unit, Number>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self.value * other.values[0],
                        self.value * other.values[1],
                        self.value * other.values[2],
                    ],
                    other.covariances,
                )
            }
        }

        /// ApproxMeasure3d *= Number
        impl<Unit, Number> MulAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, n: Number) {
                self.values[0] *= n;
                self.values[1] *= n;
                self.values[2] *= n;
            }
        }

        /// ApproxMeasure3d *= ApproxMeasure<One>
        impl<Unit, Number> MulAssign<ApproxMeasure<One, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, other: ApproxMeasure<One, Number>) {
                self.values[0] *= other.value;
                self.values[1] *= other.value;
                self.values[2] *= other.value;
            }
        }

        /// f64 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            type Output = ApproxMeasure3d<Unit, f64>;
            fn mul(self, other: ApproxMeasure3d<Unit, f64>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self * other.values[0],
                        self * other.values[1],
                        self * other.values[2],
                    ],
                    other.covariances,
                )
            }
        }

        /// f32 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            type Output = ApproxMeasure3d<Unit, f32>;
            fn mul(self, other: ApproxMeasure3d<Unit, f32>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self * other.values[0],
                        self * other.values[1],
                        self * other.values[2],
                    ],
                    other.covariances,
                )
            }
        }

        /// ApproxMeasure3d / Number -> ApproxMeasure3d
        impl<Unit, Number> Div<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::with_covariances(
                    [
                        self.values[0] * factor,
                        self.values[1] * factor,
                        self.values[2] * factor,
                    ],
                    self.covariances,
                )
            }
        }

        /// ApproxMeasure3d /= Number
        impl<Unit, Number> DivAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, n: Number) {
                let factor = Number::ONE / n;
                self.values[0] *= factor;
                self.values[1] *= factor;
                self.values[2] *= factor;
            }
        }

        /// ApproxMeasure3d / ApproxMeasure -> ApproxMeasure3d<One>
        impl<Unit, Number> Div<ApproxMeasure<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure3d<One, Number>;
            fn div(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self.values[0] / other.value,
                        self.values[1] / other.value,
                        self.values[2] / other.value,
                    ],
                    self.covariances,
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
            }
        }

        /// ApproxMeasure3d == ApproxMeasure3d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &ApproxMeasure3d<Unit, Number>) -> bool {
                self.values == other.values && self.covariances == other.covariances
            }
        }

        /// ApproxMeasure3d.clone() -> ApproxMeasure3d
        impl<Unit, Number> Clone for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        /// ApproxMeasure3d = ApproxMeasure3d
        impl<Unit, Number> Copy for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
        }

        /// format!("{}", ApproxMeasure3d) -> String
        /// ApproxMeasure3d.to_string() -> String
        impl<Unit, Number> fmt::Display for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.covariances[0][0].sqrt(), formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.covariances[1][1].sqrt(), formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[2], formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.covariances[2][2].sqrt(), formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        /// format!("{:?}", ApproxMeasure3d)
        impl<Unit, Number> fmt::Debug for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("values=(")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[2], formatter)?;
                formatter.write_str("), covariances=")?;
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<3, 3, Number>(
                        &self.covariances,
                        Unit::SUFFIX,
                        1
                    )
                )
            }
        }
    };
}
