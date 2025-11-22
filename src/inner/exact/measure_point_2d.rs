#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_point_2d {
    { $with_approx:ident $with_serde:ident } => {
        /// 2D absolute measure with generic unit of measurement, generic value type,
        /// and with 2 dynamic components.
        /// This type does not make sense for Unit::OFFSET != 0.
        /// Though, the language does not allow such constraint yet.
        pub struct MeasurePoint2d<Unit, Number = f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            pub values: [Number; 2],
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d::new([Number; 2]) -> MeasurePoint2d
            pub const fn new(values: [Number; 2]) -> Self {
                Self {
                    values,
                    phantom: PhantomData,
                }
            }

            /// MeasurePoint2d.x() -> MeasurePoint
            pub const fn x(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.values[0])
            }

            /// MeasurePoint2d.y() -> MeasurePoint
            pub const fn y(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.values[1])
            }

            /// MeasurePoint2d.convert() -> MeasurePoint2d
            pub fn convert<DestUnit>(self) -> MeasurePoint2d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                debug_assert!(Unit::OFFSET == 0.);
                debug_assert!(DestUnit::OFFSET == 0.);
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                MeasurePoint2d::<DestUnit, Number>::new([self.values[0] * factor, self.values[1] * factor])
            }

            /// MeasurePoint2d.lossless_into() -> MeasurePoint2d
            pub fn lossless_into<DestNumber>(self) -> MeasurePoint2d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                MeasurePoint2d::<Unit, DestNumber>::new([
                    DestNumber::from(self.values[0]),
                    DestNumber::from(self.values[1]),
                ])
            }

            /// MeasurePoint2d.lossy_into() -> MeasurePoint2d
            pub fn lossy_into<DestNumber>(self) -> MeasurePoint2d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                MeasurePoint2d::<Unit, DestNumber>::new([
                    DestNumber::lossy_from(self.values[0]),
                    DestNumber::lossy_from(self.values[1]),
                ])
            }
        }

        impl<Unit, Number> Default for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d::default() -> MeasurePoint2d
            /// It returns the origin.
            fn default() -> Self {
                Self::new([Number::ZERO, Number::ZERO])
            }
        }

        impl<Unit> From<MeasurePoint2d<Unit, f32>> for MeasurePoint2d<Unit, f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            /// MeasurePoint2d<f64>::from(MeasurePoint2d<f32>) -> MeasurePoint2d<f64>
            fn from(m: MeasurePoint2d<Unit, f32>) -> Self {
                Self::new([m.values[0] as f64, m.values[1] as f64])
            }
        }

        measures::if_all_true! { { $with_approx }
            impl<Unit, Number> From<ApproxMeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
            where
                Unit: MeasurementUnit<Property: VectorProperty>,
                Number: ArithmeticOps,
            {
                /// MeasurePoint2d::from(ApproxMeasurePoint2d) -> MeasurePoint2d
                fn from(am: ApproxMeasurePoint2d<Unit, Number>) -> Self {
                    Self::new(am.values)
                }
            }
        }

        measures::if_all_true! { { $with_serde }
            impl<Unit, Number> serde::Serialize for MeasurePoint2d<Unit, Number>
            where
                Unit: MeasurementUnit<Property: VectorProperty>,
                Number: ArithmeticOps + serde::Serialize,
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer
                {
                    self.values.serialize(serializer)
                }
            }

            impl<'de, Unit, Number> serde::Deserialize<'de> for MeasurePoint2d<Unit, Number>
            where
                Unit: MeasurementUnit<Property: VectorProperty>,
                Number: ArithmeticOps + serde::Deserialize<'de>,
            {
                fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
                where
                    De: serde::Deserializer<'de>,
                {
                    Ok(Self::new(serde::Deserialize::deserialize(deserializer)?))
                }
            }
        }

        impl<Unit, Number> Add<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// MeasurePoint2d + Measure2d -> MeasurePoint2d
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] + other.values[0],
                    self.values[1] + other.values[1],
                ])
            }
        }

        impl<Unit, Number> AddAssign<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d += Measure2d
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
            }
        }

        impl<Unit, Number> Sub<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// MeasurePoint2d - Measure2d -> MeasurePoint2d
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                ])
            }
        }

        impl<Unit, Number> SubAssign<Measure2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d -= Measure2d
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
            }
        }

        impl<Unit, Number> Sub<MeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Measure2d<Unit, Number>;

            /// MeasurePoint2d - MeasurePoint2d -> Measure2d
            fn sub(self, other: MeasurePoint2d<Unit, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                ])
            }
        }

        /// weighted_midpoint(MeasurePoint2d, MeasurePoint2d, Number) -> MeasurePoint2d
        pub fn weighted_midpoint_2d<Unit, Number>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint2d::<Unit, Number>::new([
                p1.values[0] * weight1 + p2.values[0] * weight2,
                p1.values[1] * weight1 + p2.values[1] * weight2,
            ])
        }

        /// midpoint(MeasurePoint2d, MeasurePoint2d) -> MeasurePoint2d
        pub fn midpoint_2d<Unit, Number>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            MeasurePoint2d::<Unit, Number>::new([
                (p1.values[0] + p2.values[0]) * Number::HALF,
                (p1.values[1] + p2.values[1]) * Number::HALF,
            ])
        }

        /// barycentric_combination([MeasurePoint2d], [Number2d]) -> MeasurePoint2d
        pub fn barycentric_combination_2d<Unit, Number>(
            points: &[MeasurePoint2d<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
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

        impl<Unit, Number> PartialEq<MeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d == MeasurePoint2d -> bool
            fn eq(&self, other: &MeasurePoint2d<Unit, Number>) -> bool {
                self.values[0] == other.values[0] && self.values[1] == other.values[1]
            }
        }

        impl<Unit, Number> Clone for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// MeasurePoint2d.clone() -> MeasurePoint2d
            fn clone(&self) -> Self {
                *self
            }
        }

        /// MeasurePoint2d = MeasurePoint2d
        impl<Unit, Number> Copy for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// format!("{}", MeasurePoint2d) -> String
            /// MeasurePoint2d.to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        impl<Unit, Number> fmt::Debug for MeasurePoint2d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", MeasurePoint2d)
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
