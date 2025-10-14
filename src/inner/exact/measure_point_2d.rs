#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_point_2d {
    {$with_approx:ident} => {
        /// Absolute measure inside a 2D space
        pub struct MeasurePoint2d<Unit, Number = f64> {
            pub values: [Number; 2],
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// Measure2d::new([Number; 2]) -> Measure2d
            pub const fn new(values: [Number; 2]) -> Self {
                Self {
                    values,
                    phantom: PhantomData,
                }
            }

            pub const fn x(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.values[0])
            }

            pub const fn y(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.values[1])
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint2d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint2d::<DestUnit, Number>::new([
                    self.values[0] * factor + offset,
                    self.values[1] * factor + offset,
                ])
            }

            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> MeasurePoint2d<Unit, DestNumber> {
                MeasurePoint2d::<Unit, DestNumber>::new([
                    DestNumber::from(self.values[0]),
                    DestNumber::from(self.values[1]),
                ])
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                self,
            ) -> MeasurePoint2d<Unit, DestNumber> {
                MeasurePoint2d::<Unit, DestNumber>::new([
                    DestNumber::lossy_from(self.values[0]),
                    DestNumber::lossy_from(self.values[1]),
                ])
            }
        }

        impl<Unit, Number> Default for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the origin.
            fn default() -> Self {
                Self::new([Number::ZERO, Number::ZERO])
            }
        }

        impl<Unit> From<MeasurePoint2d<Unit, f32>> for MeasurePoint2d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: MeasurePoint2d<Unit, f32>) -> Self {
                Self::new([m.values[0] as f64, m.values[1] as f64])
            }
        }

        // measure point + measure
        impl<Unit, Number> Add<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] + other.values[0],
                    self.values[1] + other.values[1],
                ])
            }
        }

        // measure point += measure
        impl<Unit, Number> AddAssign<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
            }
        }

        // measure point - measure
        impl<Unit, Number> Sub<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                ])
            }
        }

        // measure point -= measure
        impl<Unit, Number> SubAssign<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
            }
        }

        // measure point 2d - measure point 2d
        impl<Unit, Number> Sub<MeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Measure2d<Unit, Number>;
            fn sub(self, other: MeasurePoint2d<Unit, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                ])
            }
        }

        /// weighted_midpoint_2d(measure point 2d, measure point 2d, weight) -> measure point 2d
        pub fn weighted_midpoint_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint2d::<Unit, Number>::new([
                p1.values[0] * weight1 + p2.values[0] * weight2,
                p1.values[1] * weight1 + p2.values[1] * weight2,
            ])
        }

        /// midpoint_2d(measure point 2d, measure point 2d) -> measure point 2d
        pub fn midpoint_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint2d::<Unit, Number>::new([
                (p1.values[0] + p2.values[0]) * Number::HALF,
                (p1.values[1] + p2.values[1]) * Number::HALF,
            ])
        }

        /// barycentric_combination_2d(array of 2d measure points, array of weights) -> 2d measure point
        pub fn barycentric_combination_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint2d<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint2d::<Unit, Number>::new([
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
            ])
        }

        // MeasurePoint2d == MeasurePoint2d -> bool
        impl<Unit, Number> PartialEq<MeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &MeasurePoint2d<Unit, Number>) -> bool {
                self.values[0] == other.values[0] && self.values[1] == other.values[1]
            }
        }

        // MeasurePoint2d.clone() -> MeasurePoint2d
        impl<Unit, Number> Clone for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // MeasurePoint2d = MeasurePoint2d
        impl<Unit, Number> Copy for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", MeasurePoint2d)
        impl<Unit, Number> fmt::Display for MeasurePoint2d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", MeasurePoint2d)
        impl<Unit, Number> fmt::Debug for MeasurePoint2d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
