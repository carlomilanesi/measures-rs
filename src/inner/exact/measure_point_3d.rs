#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_point_3d {
    {$with_approx:ident} => {
        pub struct MeasurePoint3d<Unit, Number = f64> {
            pub values: [Number; 3],
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// MeasurePoint3d::new([Number; 3]) -> MeasurePoint3d
            pub const fn new(values: [Number; 3]) -> Self {
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

            pub const fn z(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.values[2])
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint3d::<DestUnit, Number>::new([
                    self.values[0] * factor + offset,
                    self.values[1] * factor + offset,
                    self.values[2] * factor + offset,
                ])
            }

            /// MeasurePoint3d.lossless_into() -> MeasurePoint3d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> MeasurePoint3d<Unit, DestNumber> {
                MeasurePoint3d::<Unit, DestNumber>::new([
                    DestNumber::from(self.values[0]),
                    DestNumber::from(self.values[1]),
                    DestNumber::from(self.values[2]),
                ])
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint3d<Unit, DestNumber> {
                MeasurePoint3d::<Unit, DestNumber>::new([
                    DestNumber::lossy_from(self.values[0]),
                    DestNumber::lossy_from(self.values[1]),
                    DestNumber::lossy_from(self.values[2]),
                ])
            }
        }

        impl<Unit, Number> Default for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the origin.
            fn default() -> Self {
                Self::new([Number::ZERO, Number::ZERO, Number::ZERO])
            }
        }

        impl<Unit> From<MeasurePoint3d<Unit, f32>> for MeasurePoint3d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: MeasurePoint3d<Unit, f32>) -> Self {
                Self::new([m.values[0] as f64, m.values[1] as f64, m.values[2] as f64])
            }
        }

        // MeasurePoint3d + Measure3d -> MeasurePoint3d
        impl<Unit, Number> Add<Measure3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] + other.values[0],
                    self.values[1] + other.values[1],
                    self.values[2] + other.values[2],
                ])
            }
        }

        // MeasurePoint3d += Measure3d
        impl<Unit, Number> AddAssign<Measure3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
                self.values[2] += other.values[2];
            }
        }

        // MeasurePoint3d - Measure3d -> MeasurePoint3d
        impl<Unit, Number> Sub<Measure3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                    self.values[2] - other.values[2],
                ])
            }
        }

        // MeasurePoint3d -= Measure3d
        impl<Unit, Number> SubAssign<Measure3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
                self.values[2] -= other.values[2];
            }
        }

        /// measure point 3d - measure point 3d -> measure 3d
        impl<Unit, Number> Sub<MeasurePoint3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, Number>;
            fn sub(self, other: MeasurePoint3d<Unit, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                    self.values[2] - other.values[2],
                ])
            }
        }

        /// weighted_midpoint_3d(measure point 3d, measure point 3d, weight) -> measure point 3d
        pub fn weighted_midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint3d<Unit, Number>,
            p2: MeasurePoint3d<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint3d::<Unit, Number>::new([
                p1.values[0] * weight1 + p2.values[0] * weight2,
                p1.values[1] * weight1 + p2.values[1] * weight2,
                p1.values[2] * weight1 + p2.values[2] * weight2,
            ])
        }

        /// midpoint_3d(measure point 3d, measure point 3d) -> measure point 3d
        pub fn midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint3d<Unit, Number>,
            p2: MeasurePoint3d<Unit, Number>,
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint3d::<Unit, Number>::new([
                (p1.values[0] + p2.values[0]) * Number::HALF,
                (p1.values[1] + p2.values[1]) * Number::HALF,
                (p1.values[2] + p2.values[2]) * Number::HALF,
            ])
        }

        /// barycentric_combination_3d(array of 3d measure points, array of weights) -> 3d measure point
        pub fn barycentric_combination_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint3d<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint3d::<Unit, Number>::new([
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
            ])
        }

        // MeasurePoint3d == MeasurePoint3d -> bool
        impl<Unit, Number> PartialEq<MeasurePoint3d<Unit, Number>> for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &MeasurePoint3d<Unit, Number>) -> bool {
                self.values == other.values
            }
        }

        // MeasurePoint3d.clone() -> MeasurePoint3d
        impl<Unit, Number> Clone for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // MeasurePoint3d = MeasurePoint3d
        impl<Unit, Number> Copy for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", MeasurePoint3d)
        impl<Unit, Number> fmt::Display for MeasurePoint3d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", MeasurePoint3d)
        impl<Unit, Number> fmt::Debug for MeasurePoint3d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
