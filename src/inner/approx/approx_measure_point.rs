#[macro_export]
macro_rules! inner_define_approx_measure_point {
    {$exact:tt} => {
        /// Approximate point measurement with static unit of measurement and value type,
        /// and with dynamic value and variance.
        pub struct ApproxMeasurePoint<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub value: Number,
            pub variance: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// ApproxMeasurePoint::new_with_variance(Number, Number) -> ApproxMeasurePoint
            pub const fn new_with_variance(value: Number, variance: Number) -> Self {
                Self {
                    value,
                    variance,
                    phantom: PhantomData,
                }
            }

            rs_measures::if_all_true! { {$exact}
                /// ApproxMeasurePoint::new_with_uncertainty(Number, Measure) -> ApproxMeasurePoint
                pub fn new_with_uncertainty(value: Number, uncertainty: Measure<Unit, Number>) -> Self {
                    Self {
                        value,
                        variance: uncertainty.value * uncertainty.value,
                        phantom: PhantomData,
                    }
                }

                /// ApproxMeasurePoint.uncertainty() -> Measure
                pub fn uncertainty(self) -> MeasurePoint<Unit, Number> {
                    MeasurePoint::<Unit, Number>::new(self.variance.sqrt())
                }

                /// ApproxMeasurePoint::from_measure_with_variance(Measure, Number) -> ApproxMeasurePoint
                pub fn from_measure_point_with_variance(measure_point: MeasurePoint<Unit, Number>, variance: Number) -> Self {
                    Self::new_with_variance(measure_point.value, variance)
                }

                /// ApproxMeasurePoint::from_measure_with_uncertainty(Measure, Measure) -> ApproxMeasurePoint
                pub fn from_measure_point_with_uncertainty(measure_point: MeasurePoint<Unit, Number>, uncertainty: Measure<Unit, Number>) -> Self {
                    Self::new_with_uncertainty(measure_point.value, uncertainty)
                }

                /// ApproxMeasurePoint.to_measure_point() -> MeasurePoint
                pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                    MeasurePoint::<Unit, Number>::new(self.value)
                }
            }

            /// ApproxMeasurePoint.convert() -> ApproxMeasurePoint
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                self,
            ) -> ApproxMeasurePoint<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                ApproxMeasurePoint::<DestUnit, Number>::new_with_variance(
                    self.value * ratio
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    self.variance * ratio * ratio,
                )
            }

            /// ApproxMeasurePoint.lossless_into() -> ApproxMeasurePoint
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> ApproxMeasurePoint<Unit, DestNumber> {
                ApproxMeasurePoint::<Unit, DestNumber>::new_with_variance(
                    DestNumber::from(self.value),
                    DestNumber::from(self.variance),
                )
            }

            /// ApproxMeasurePoint.lossy_into() -> ApproxMeasurePoint
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                self,
            ) -> ApproxMeasurePoint<Unit, DestNumber> {
                ApproxMeasurePoint::<Unit, DestNumber>::new_with_variance(
                    DestNumber::lossy_from(self.value),
                    DestNumber::lossy_from(self.variance),
                )
            }
        }

        impl<Unit, Number> Default for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// It returns the origin.
            fn default() -> Self {
                Self::new_with_variance(Number::ZERO, Number::ZERO)
            }
        }

        // ApproxMeasurePoint + ApproxMeasure -> ApproxMeasurePoint
        /// Assuming statistical independence,
        /// the uncertainty is summed in quadrature,
        /// and so the variance is the sum of the variances.
        impl<Unit, Number> Add<ApproxMeasure<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                Self::new_with_variance(self.value + other.value, self.variance + other.variance)
            }
        }

        // ApproxMeasurePoint.add_with_correlation(ApproxMeasure, Number) -> ApproxMeasurePoint
        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_with_correlation(self, other: ApproxMeasure<Unit, Number>, correlation: Number) -> Self {
                Self::new_with_variance(
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

        // ApproxMeasurePoint += ApproxMeasure
        impl<Unit, Number> AddAssign<ApproxMeasure<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: ApproxMeasure<Unit, Number>) {
                self.value += other.value;
                self.variance += other.variance;
            }
        }

        // ApproxMeasurePoint.add_assign_with_correlation(ApproxMeasure, Number)
        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
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

        // ApproxMeasurePoint - ApproxMeasure -> ApproxMeasurePoint
        impl<Unit, Number> Sub<ApproxMeasure<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: ApproxMeasure<Unit, Number>) -> Self::Output {
                Self::new_with_variance(self.value - other.value, self.variance + other.variance)
            }
        }

        // ApproxMeasurePoint.sub_with_correlation(ApproxMeasure, Number) -> ApproxMeasurePoint
        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_with_correlation(self, other: ApproxMeasure<Unit, Number>, correlation: Number) -> Self {
                Self::new_with_variance(
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

        // ApproxMeasurePoint -= ApproxMeasure
        impl<Unit, Number> SubAssign<ApproxMeasure<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: ApproxMeasure<Unit, Number>) {
                self.value -= other.value;
                self.variance += other.variance;
            }
        }

        // ApproxMeasurePoint.sub_assign_with_correlation(ApproxMeasure, Number)
        // For subtractions, the variances are handled as for additions.
        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
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

        // ApproxMeasurePoint - ApproxMeasurePoint -> ApproxMeasure
        impl<Unit, Number> Sub<ApproxMeasurePoint<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = ApproxMeasure<Unit, Number>;
            fn sub(self, other: ApproxMeasurePoint<Unit, Number>) -> Self::Output {
                Self::Output::new_with_variance(self.value - other.value, self.variance + other.variance)
            }
        }

        // ApproxMeasurePoint.difference_with_correlation(ApproxMeasurePoint, Number) -> ApproxMeasure
        impl<Unit, Number> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn difference_with_correlation(
                self,
                other: ApproxMeasurePoint<Unit, Number>,
                correlation: Number,
            ) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::new_with_variance(
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

        // approx_weighted_midpoint(ApproxMeasurePoint, ApproxMeasurePoint, weight) -> ApproxMeasurePoint
        pub fn approx_weighted_midpoint<Unit, Number>(
            p1: ApproxMeasurePoint<Unit, Number>,
            p2: ApproxMeasurePoint<Unit, Number>,
            weight1: Number,
        ) -> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                p1.value * weight1 + p2.value * (Number::ONE - weight1),
                p1.variance * weight1 + p2.variance * (Number::ONE - weight1),
            )
        }

        // approx_midpoint(ApproxMeasurePoint, ApproxMeasurePoint) -> ApproxMeasurePoint
        pub fn approx_midpoint<Unit, Number>(
            p1: ApproxMeasurePoint<Unit, Number>,
            p2: ApproxMeasurePoint<Unit, Number>,
        ) -> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                (p1.value + p2.value) * Number::HALF,
                (p1.variance + p2.variance) * Number::HALF,
            )
        }

        // approx_barycentric_combination([ApproxMeasurePoint], [Number]) -> ApproxMeasurePoint
        pub fn approx_barycentric_combination<Unit, Number>(
            points: &[ApproxMeasurePoint<Unit, Number>],
            weights: &[Number],
        ) -> ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                points.iter().zip(weights).map(|(p, &w)| p.value * w).sum(),
                points
                    .iter()
                    .zip(weights)
                    .map(|(p, &w)| p.variance * w)
                    .sum(),
            )
        }

        //...
        impl<Unit, Number> PartialEq<ApproxMeasurePoint<Unit, Number>> for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &ApproxMeasurePoint<Unit, Number>) -> bool {
                self.value == other.value && self.variance == other.variance
            }
        }

        impl<Unit, Number> Clone for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit, Number> Copy for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        // format!("{}", ApproxMeasurePoint)
        impl<Unit, Number> fmt::Display for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", ApproxMeasurePoint)
        impl<Unit, Number> fmt::Debug for ApproxMeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str("\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
