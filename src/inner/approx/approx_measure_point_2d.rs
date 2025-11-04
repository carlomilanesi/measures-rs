#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure_point_2d {
    { $with_approx:ident } => {
        /// Approximate 2d absolute measure, with generic unit of measurement and value type,
        /// and with dynamic values, variances, and covariances.
        pub struct ApproxMeasurePoint2d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub values: [Number; 2],
            pub covariances: [[Number; 2]; 2],
            phantom: core::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            pub const fn with_covariances(values: [Number; 2], covariances: [[Number; 2]; 2]) -> Self {
                Self {
                    values,
                    covariances,
                    phantom: PhantomData,
                }
            }

            /// ApproxMeasurePoint2d::with_uncertainty(Number, Number, Number, Measure) -> ApproxMeasurePoint
            pub fn with_uncertainty(values: [Number; 2], uncertainty: Measure<Unit, Number>) -> Self {
                let v = uncertainty.value * uncertainty.value;
                Self::with_covariances(values, [[v, Number::ZERO], [Number::ZERO, v]])
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

            pub fn convert<DestUnit>(self) -> ApproxMeasurePoint2d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                debug_assert!(Unit::OFFSET == 0.);
                debug_assert!(DestUnit::OFFSET == 0.);
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let r2 = ratio * ratio;
                ApproxMeasurePoint2d::<DestUnit, Number>::with_covariances(
                    [self.values[0] * ratio, self.values[1] * ratio],
                    [
                        [self.covariances[0][0] * r2, self.covariances[0][1] * r2],
                        [self.covariances[1][0] * r2, self.covariances[1][1] * r2],
                    ],
                )
            }

            pub fn lossless_into<DestNumber>(self) -> ApproxMeasurePoint2d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                ApproxMeasurePoint2d::<Unit, DestNumber>::with_covariances(
                    [
                        DestNumber::from(self.values[0]),
                        DestNumber::from(self.values[1]),
                    ],
                    [
                        [self.covariances[0][0].into(), self.covariances[0][1].into()],
                        [self.covariances[1][0].into(), self.covariances[1][1].into()],
                    ],
                )
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasurePoint2d<Unit, DestNumber> {
                ApproxMeasurePoint2d::<Unit, DestNumber>::with_covariances(
                    [
                        DestNumber::lossy_from(self.values[0]),
                        DestNumber::lossy_from(self.values[1]),
                    ],
                    [
                        [
                            DestNumber::lossy_from(self.covariances[0][0]),
                            DestNumber::lossy_from(self.covariances[0][1]),
                        ],
                        [
                            DestNumber::lossy_from(self.covariances[1][0]),
                            DestNumber::lossy_from(self.covariances[1][1]),
                        ],
                    ],
                )
            }
        }

        impl<Unit, Number> Default for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// ApproxMeasurePoint2d::default() -> ApproxMeasurePoint2d
            /// It returns the origin, with no uncertainty.
            fn default() -> Self {
                Self::with_covariances(
                    [Number::ZERO, Number::ZERO],
                    [[Number::ZERO, Number::ZERO], [Number::ZERO, Number::ZERO]],
                )
            }
        }

        impl<Unit> From<ApproxMeasurePoint2d<Unit, f32>> for ApproxMeasurePoint2d<Unit, f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            fn from(m: ApproxMeasurePoint2d<Unit, f32>) -> Self {
                Self::with_covariances(
                    [m.values[0] as f64, m.values[1] as f64],
                    [
                        [m.covariances[0][0] as f64, m.covariances[0][1] as f64],
                        [m.covariances[1][0] as f64, m.covariances[1][1] as f64],
                    ],
                )
            }
        }

        // ApproxMeasurePoint2d + ApproxMeasure2d -> ApproxMeasurePoint2d
        impl<Unit, Number> Add<ApproxMeasure2d<Unit, Number>> for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: ApproxMeasure2d<Unit, Number>) -> Self::Output {
                Self::with_covariances(
                    [
                        self.values[0] + other.values[0],
                        self.values[1] + other.values[1],
                    ],
                    self.covariances,
                )
            }
        }

        // ApproxMeasurePoint2d += ApproxMeasure2d
        impl<Unit, Number> AddAssign<ApproxMeasure2d<Unit, Number>> for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: ApproxMeasure2d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
            }
        }

        // ApproxMeasurePoint2d - ApproxMeasure2d -> ApproxMeasurePoint2d
        impl<Unit, Number> Sub<ApproxMeasure2d<Unit, Number>> for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: ApproxMeasure2d<Unit, Number>) -> Self::Output {
                Self::with_covariances(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                    ],
                    self.covariances,
                )
            }
        }

        // ApproxMeasurePoint2d -= ApproxMeasure2d
        impl<Unit, Number> SubAssign<ApproxMeasure2d<Unit, Number>> for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: ApproxMeasure2d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
            }
        }

        /// measure point 2d - measure point 2d -> measure 2d
        impl<Unit, Number> Sub<ApproxMeasurePoint2d<Unit, Number>> for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure2d<Unit, Number>;
            fn sub(self, other: ApproxMeasurePoint2d<Unit, Number>) -> Self::Output {
                Self::Output::with_covariances(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                    ],
                    self.covariances,
                )
            }
        }

        /// weighted_midpoint_2d(measure point 2d, measure point 2d, weight) -> measure point 2d
        pub fn approx_weighted_midpoint_2d<Unit, Number>(
            p1: ApproxMeasurePoint2d<Unit, Number>,
            p2: ApproxMeasurePoint2d<Unit, Number>,
            weight1: Number,
        ) -> ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            let weight2 = Number::ONE - weight1;
            ApproxMeasurePoint2d::<Unit, Number>::with_covariances(
                [
                    p1.values[0] * weight1 + p2.values[0] * weight2,
                    p1.values[1] * weight1 + p2.values[1] * weight2,
                ],
                p1.covariances,
            )
        }

        /// midpoint_2d(measure point 2d, measure point 2d) -> measure point 2d
        pub fn approx_midpoint_2d<Unit, Number>(
            p1: ApproxMeasurePoint2d<Unit, Number>,
            p2: ApproxMeasurePoint2d<Unit, Number>,
        ) -> ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint2d::<Unit, Number>::with_covariances(
                [
                    (p1.values[0] + p2.values[0]) * Number::HALF,
                    (p1.values[1] + p2.values[1]) * Number::HALF,
                ],
                p1.covariances,
            )
        }

        /// barycentric_combination_2d(array of 2d measure points, array of weights) -> 2d measure point
        pub fn approx_barycentric_combination_2d<Unit, Number>(
            points: &[ApproxMeasurePoint2d<Unit, Number>],
            weights: &[Number],
        ) -> ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint2d::<Unit, Number>::with_covariances(
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
                ],
                points[0].covariances,
            )
        }

        // ApproxMeasurePoint2d == ApproxMeasurePoint2d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasurePoint2d<Unit, Number>>
            for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &ApproxMeasurePoint2d<Unit, Number>) -> bool {
                self.values == other.values && self.covariances == other.covariances
            }
        }

        // ApproxMeasurePoint2d.clone() -> ApproxMeasurePoint2d
        impl<Unit, Number> Clone for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // ApproxMeasurePoint2d = ApproxMeasurePoint2d
        impl<Unit, Number> Copy for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
        }

        /// format!("{}", ApproxMeasurePoint2d) -> String
        /// ApproxMeasurePoint2d.to_string() -> String
        impl<Unit, Number> fmt::Display for ApproxMeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(" \u{b1} ")?; // ±
                fmt::Display::fmt(&self.covariances[0][0].sqrt(), formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(" \u{b1} ")?; // ±
                fmt::Display::fmt(&self.covariances[1][1].sqrt(), formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", ApproxMeasurePoint2d)
        impl<Unit, Number> fmt::Debug for ApproxMeasurePoint2d<Unit, Number>
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
                formatter.write_str("), covariances=")?;
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<2, 2, Number>(
                        &self.covariances,
                        Unit::SUFFIX,
                        1
                    )
                )
            }
        }
    };
}
