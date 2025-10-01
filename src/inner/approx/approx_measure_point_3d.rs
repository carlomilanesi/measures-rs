#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_approx_measure_point_3d {
    {$with_approx:ident} => {
        pub struct ApproxMeasurePoint3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub values: [Number; 3],
            pub variance: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            pub const fn with_variance(values: [Number; 3], variance: Number) -> Self {
                Self {
                    values,
                    variance,
                    phantom: PhantomData,
                }
            }

            /// ApproxMeasurePoint3d::with_uncertainty(Number, Number, Number, Measure) -> ApproxMeasurePoint
            pub fn with_uncertainty(values: [Number; 3], uncertainty: Measure<Unit, Number>) -> Self {
                Self::with_variance(values, uncertainty.value * uncertainty.value)
            }

            pub fn uncertainty(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.variance.sqrt())
            }

            pub fn x(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(
                    self.values[0],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn y(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(
                    self.values[1],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn z(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::with_variance(
                    self.values[02],
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> ApproxMeasurePoint3d<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                ApproxMeasurePoint3d::<DestUnit, Number>::with_variance(
                    [
                        self.values[0] * ratio + offset,
                        self.values[1] * ratio + offset,
                        self.values[2] * ratio + offset,
                    ],
                    self.variance * ratio * ratio,
                )
            }

            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> ApproxMeasurePoint3d<Unit, DestNumber> {
                ApproxMeasurePoint3d::<Unit, DestNumber>::with_variance(
                    [
                        DestNumber::from(self.values[0]),
                        DestNumber::from(self.values[1]),
                        DestNumber::from(self.values[2]),
                    ],
                    DestNumber::from(self.variance),
                )
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasurePoint3d<Unit, DestNumber> {
                ApproxMeasurePoint3d::<Unit, DestNumber>::with_variance(
                    [
                        DestNumber::lossy_from(self.values[0]),
                        DestNumber::lossy_from(self.values[1]),
                        DestNumber::lossy_from(self.values[2]),
                    ],
                    DestNumber::lossy_from(self.variance),
                )
            }
        }

        impl<Unit, Number> Default for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the origin.
            fn default() -> Self {
                Self::with_variance([Number::ZERO, Number::ZERO, Number::ZERO], Number::ZERO)
            }
        }

        impl<Unit> From<ApproxMeasurePoint3d<Unit, f32>> for ApproxMeasurePoint3d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: ApproxMeasurePoint3d<Unit, f32>) -> Self {
                Self::with_variance(
                    [m.values[0] as f64, m.values[1] as f64, m.values[2] as f64],
                    m.variance as f64,
                )
            }
        }

        // ApproxMeasurePoint3d + ApproxMeasure3d -> ApproxMeasurePoint3d
        impl<Unit, Number> Add<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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

        // ApproxMeasurePoint3d += ApproxMeasure3d
        impl<Unit, Number> AddAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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

        // ApproxMeasurePoint3d - ApproxMeasure3d -> ApproxMeasurePoint3d
        impl<Unit, Number> Sub<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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

        // ApproxMeasurePoint3d -= ApproxMeasure3d
        impl<Unit, Number> SubAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
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

        /// measure point 3d - measure point 3d -> measure 3d
        impl<Unit, Number> Sub<ApproxMeasurePoint3d<Unit, Number>> for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, Number>;
            fn sub(self, other: ApproxMeasurePoint3d<Unit, Number>) -> Self::Output {
                Self::Output::with_variance(
                    [
                        self.values[0] - other.values[0],
                        self.values[1] - other.values[1],
                        self.values[2] - other.values[2],
                    ],
                    self.variance + other.variance,
                )
            }
        }

        /// weighted_midpoint_3d(measure point 3d, measure point 3d, weight) -> measure point 3d
        pub fn approx_weighted_midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: ApproxMeasurePoint3d<Unit, Number>,
            p2: ApproxMeasurePoint3d<Unit, Number>,
            weight1: Number,
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            let weight2 = Number::ONE - weight1;
            ApproxMeasurePoint3d::<Unit, Number>::with_variance(
                [
                    p1.values[0] * weight1 + p2.values[0] * weight2,
                    p1.values[1] * weight1 + p2.values[1] * weight2,
                    p1.values[2] * weight1 + p2.values[2] * weight2,
                ],
                p1.variance * weight1 + p2.variance * weight2,
            )
        }

        /// midpoint_3d(measure point 3d, measure point 3d) -> measure point 3d
        pub fn approx_midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: ApproxMeasurePoint3d<Unit, Number>,
            p2: ApproxMeasurePoint3d<Unit, Number>,
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            ApproxMeasurePoint3d::<Unit, Number>::with_variance(
                [
                    (p1.values[0] + p2.values[0]) * Number::HALF,
                    (p1.values[1] + p2.values[1]) * Number::HALF,
                    (p1.values[2] + p2.values[2]) * Number::HALF,
                ],
                (p1.variance + p2.variance) * Number::HALF,
            )
        }

        /// barycentric_combination_3d(array of 3d measure points, array of weights) -> 3d measure point
        pub fn approx_barycentric_combination_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[ApproxMeasurePoint3d<Unit, Number>],
            weights: &[Number],
        ) -> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            ApproxMeasurePoint3d::<Unit, Number>::with_variance(
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
                points
                    .iter()
                    .zip(weights)
                    .map(|(p, &w)| p.variance * w)
                    .sum(),
            )
        }

        // ApproxMeasurePoint3d == ApproxMeasurePoint3d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasurePoint3d<Unit, Number>>
            for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &ApproxMeasurePoint3d<Unit, Number>) -> bool {
                self.values == other.values && self.variance == other.variance
            }
        }

        // ApproxMeasurePoint3d.clone() -> ApproxMeasurePoint3d
        impl<Unit, Number> Clone for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // ApproxMeasurePoint3d = ApproxMeasurePoint3d
        impl<Unit, Number> Copy for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", ApproxMeasurePoint3d)
        impl<Unit, Number> fmt::Display for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
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

        // format!("{:?}", ApproxMeasurePoint3d)
        impl<Unit, Number> fmt::Debug for ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
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
