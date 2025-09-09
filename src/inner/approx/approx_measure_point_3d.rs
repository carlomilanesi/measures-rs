#[macro_export]
macro_rules! inner_define_approx_measure_point_3d {
    {$with_approx:ident} => {
        pub struct ApproxMeasurePoint3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            pub variance: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> ApproxMeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            pub const fn new_with_variance(x: Number, y: Number, z: Number, variance: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    variance,
                    phantom: PhantomData,
                }
            }

            /// ApproxMeasurePoint3d::new_with_uncertainty(Number, Number, Number, Measure) -> ApproxMeasurePoint
            pub fn new_with_uncertainty(
                x: Number,
                y: Number,
                z: Number,
                uncertainty: Measure<Unit, Number>,
            ) -> Self {
                Self::new_with_variance(x, y, z, uncertainty.value * uncertainty.value)
            }

            pub fn uncertainty(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.variance.sqrt())
            }

            pub fn x(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                    self.x,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn y(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                    self.y,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn z(self) -> ApproxMeasurePoint<Unit, Number> {
                ApproxMeasurePoint::<Unit, Number>::new_with_variance(
                    self.z,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> ApproxMeasurePoint3d<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                ApproxMeasurePoint3d::<DestUnit, Number>::new_with_variance(
                    self.x * ratio + offset,
                    self.y * ratio + offset,
                    self.z * ratio + offset,
                    self.variance * ratio * ratio,
                )
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> ApproxMeasurePoint3d<Unit, DestNumber> {
                ApproxMeasurePoint3d::<Unit, DestNumber>::new_with_variance(
                    DestNumber::from(self.x),
                    DestNumber::from(self.y),
                    DestNumber::from(self.z),
                    DestNumber::from(self.variance),
                )
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasurePoint3d<Unit, DestNumber> {
                ApproxMeasurePoint3d::<Unit, DestNumber>::new_with_variance(
                    DestNumber::lossy_from(self.x),
                    DestNumber::lossy_from(self.y),
                    DestNumber::lossy_from(self.z),
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
                Self::new_with_variance(Number::ZERO, Number::ZERO, Number::ZERO, Number::ZERO)
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
                Self::new_with_variance(
                    self.x + other.x,
                    self.y + other.y,
                    self.z + other.z,
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
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
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
                Self::new_with_variance(
                    self.x - other.x,
                    self.y - other.y,
                    self.z - other.z,
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
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
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
                Self::Output::new_with_variance(
                    self.x - other.x,
                    self.y - other.y,
                    self.z - other.z,
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
            ApproxMeasurePoint3d::<Unit, Number>::new_with_variance(
                p1.x * weight1 + p2.x * weight2,
                p1.y * weight1 + p2.y * weight2,
                p1.z * weight1 + p2.z * weight2,
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
            ApproxMeasurePoint3d::<Unit, Number>::new_with_variance(
                (p1.x + p2.x) * Number::HALF,
                (p1.y + p2.y) * Number::HALF,
                (p1.z + p2.z) * Number::HALF,
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
            ApproxMeasurePoint3d::<Unit, Number>::new_with_variance(
                points.iter().zip(weights).map(|(p, &w)| p.x * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.y * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.z * w).sum(),
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
                self.x == other.x
                    && self.y == other.y
                    && self.z == other.z
                    && self.variance == other.variance
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
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
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
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
                formatter.write_str(")\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
