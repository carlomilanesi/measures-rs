#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure_point_3d {
    { $with_approx:ident } => {
        /// Approximate 3d absolute measure, with generic unit of measurement and value type,
        /// and with dynamic values, variances, and covariances.
        pub struct ApproxMeasurePoint3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub values: [Number; 3],
            pub covariances: [[Number; 3]; 3],
            phantom: core::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            pub const fn with_covariances(values: [Number; 3], covariances: [[Number; 3]; 3]) -> Self {
                Self {
                    values,
                    covariances,
                    phantom: PhantomData,
                }
            }

            /// ApproxMeasurePoint3d::with_uncertainty(Number, Number, Number, Measure) -> ApproxMeasurePoint
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

            pub fn x(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(self.values[0], self.covariances[0][0])
            }

            pub fn y(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(self.values[1], self.covariances[1][1])
            }

            pub fn z(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(self.values[02], self.covariances[2][2])
            }

            pub fn convert<DestUnit>(self) -> ApproxMeasurePoint3d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                let r2 = ratio * ratio;
                ApproxMeasurePoint3d::<DestUnit, Number>::with_covariances(
                    [
                        self.values[0] * ratio + offset,
                        self.values[1] * ratio + offset,
                        self.values[2] * ratio + offset,
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

            pub fn lossless_into<DestNumber>(&self) -> ApproxMeasurePoint3d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                ApproxMeasurePoint3d::<Unit, DestNumber>::with_covariances(
                    [
                        DestNumber::from(self.values[0]),
                        DestNumber::from(self.values[1]),
                        DestNumber::from(self.values[2]),
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

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasurePoint3d<Unit, DestNumber> {
                ApproxMeasurePoint3d::<Unit, DestNumber>::with_covariances(
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
        }

        impl<Unit, Number> Default for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// ApproxMeasurePoint3d::default() -> ApproxMeasurePoint3d
            /// It returns the origin, with no uncertainty.
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

        impl<Unit> From<ApproxMeasurePoint3d<Unit, f32>> for ApproxMeasurePoint3d<Unit, f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            fn from(m: ApproxMeasurePoint3d<Unit, f32>) -> Self {
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

        // ApproxMeasurePoint3d + ApproxMeasure3d -> ApproxMeasurePoint3d
        impl<Unit, Number> Add<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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
                    self.covariances,
                )
            }
        }

        // ApproxMeasurePoint3d += ApproxMeasure3d
        impl<Unit, Number> AddAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
                self.values[2] += other.values[2];
            }
        }

        // ApproxMeasurePoint3d - ApproxMeasure3d -> ApproxMeasurePoint3d
        impl<Unit, Number> Sub<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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
                    self.covariances,
                )
            }
        }

        // ApproxMeasurePoint3d -= ApproxMeasure3d
        impl<Unit, Number> SubAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: ApproxMeasure3d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
                self.values[2] -= other.values[2];
            }
        }

        /// measure point 3d - measure point 3d -> measure 3d
        impl<Unit, Number> Sub<ApproxMeasurePoint3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure3d<Unit, Number>;
            fn sub(self, other: ApproxMeasurePoint3d<Unit, Number>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                        self.values[2] - other.values[2],
                    ],
                    self.covariances,
                )
            }
        }

        /// weighted_midpoint_3d(measure point 3d, measure point 3d, weight) -> measure point 3d
        pub fn approx_weighted_midpoint_3d<Unit, Number>(
            p1: ApproxMeasurePoint3d<Unit, Number>,
            p2: ApproxMeasurePoint3d<Unit, Number>,
            weight1: Number,
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            let weight2 = Number::ONE - weight1;
            ApproxMeasurePoint3d::<Unit, Number>::with_covariances(
                [
                    p1.values[0] * weight1 + p2.values[0] * weight2,
                    p1.values[1] * weight1 + p2.values[1] * weight2,
                    p1.values[2] * weight1 + p2.values[2] * weight2,
                ],
                p1.covariances,
            )
        }

        /// midpoint_3d(measure point 3d, measure point 3d) -> measure point 3d
        pub fn approx_midpoint_3d<Unit, Number>(
            p1: ApproxMeasurePoint3d<Unit, Number>,
            p2: ApproxMeasurePoint3d<Unit, Number>,
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint3d::<Unit, Number>::with_covariances(
                [
                    (p1.values[0] + p2.values[0]) * Number::HALF,
                    (p1.values[1] + p2.values[1]) * Number::HALF,
                    (p1.values[2] + p2.values[2]) * Number::HALF,
                ],
                p1.covariances,
            )
        }

        /// barycentric_combination_3d(array of 3d measure points, array of weights) -> 3d measure point
        pub fn approx_barycentric_combination_3d<Unit, Number>(
            points: &[ApproxMeasurePoint3d<Unit, Number>],
            weights: &[Number],
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint3d::<Unit, Number>::with_covariances(
                [
                    points
                        .iter()
                        .zip(weights)
                        .map(|(p, &w)| p.values[0] * w)
                        .sum(),
                    points
                        .iter()
                        .zip(weights)
                        .map(|(p, &w)| p.values[1] * w)
                        .sum(),
                    points
                        .iter()
                        .zip(weights)
                        .map(|(p, &w)| p.values[2] * w)
                        .sum(),
                ],
                points[0].covariances,
            )
        }

        // ApproxMeasurePoint3d == ApproxMeasurePoint3d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasurePoint3d<Unit, Number>>
            for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &ApproxMeasurePoint3d<Unit, Number>) -> bool {
                self.values == other.values && self.covariances == other.covariances
            }
        }

        // ApproxMeasurePoint3d.clone() -> ApproxMeasurePoint3d
        impl<Unit, Number> Clone for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // ApproxMeasurePoint3d = ApproxMeasurePoint3d
        impl<Unit, Number> Copy for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
        }

        /// format!("{}", ApproxMeasurePoint3d) -> String
        /// ApproxMeasurePoint3d.to_string() -> String
        impl<Unit, Number> fmt::Display for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
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

        // format!("{:?}", ApproxMeasurePoint3d)
        impl<Unit, Number> fmt::Debug for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
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
