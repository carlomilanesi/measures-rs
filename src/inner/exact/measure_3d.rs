#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_3d {
    { $with_approx:ident } => {
        /// 3D relative measure with generic unit of measurement, generic value type,
        /// and with 3 dynamic components.
        pub struct Measure3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            pub values: [Number; 3],
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d::new([Number; 3]) -> Measure3d
            pub const fn new(values: [Number; 3]) -> Self {
                Self {
                    values,
                    phantom: PhantomData,
                }
            }

            /// Measure3d.x() -> Measure
            pub const fn x(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.values[0])
            }

            /// Measure3d.y() -> Measure
            pub const fn y(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.values[1])
            }

            /// Measure3d.z() -> Measure
            pub const fn z(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.values[2])
            }

            /// Measure3d.convert() -> Measure3d
            pub fn convert<DestUnit>(self) -> Measure3d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure3d::<DestUnit, Number>::new([
                    self.values[0] * factor,
                    self.values[1] * factor,
                    self.values[2] * factor,
                ])
            }

            /// Measure3d.lossless_into() -> Measure3d
            pub fn lossless_into<DestNumber>(self) -> Measure3d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + From<Number>,
            {
                Measure3d::<Unit, DestNumber>::new([
                    DestNumber::from(self.values[0]),
                    DestNumber::from(self.values[1]),
                    DestNumber::from(self.values[2]),
                ])
            }

            /// Measure3d.lossy_into() -> Measure3d
            pub fn lossy_into<DestNumber>(&self) -> Measure3d<Unit, DestNumber>
            where
                DestNumber: ArithmeticOps + LossyFrom<Number>,
            {
                Measure3d::<Unit, DestNumber>::new([
                    DestNumber::lossy_from(self.values[0]),
                    DestNumber::lossy_from(self.values[1]),
                    DestNumber::lossy_from(self.values[2]),
                ])
            }

            /// Measure3d.norm() -> Measure
            pub fn norm(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(
                    (self.values[0] * self.values[0]
                        + self.values[1] * self.values[1]
                        + self.values[2] * self.values[2])
                        .sqrt(),
                )
            }

            /// Measure3d.squared_norm() -> Number
            pub fn squared_norm(self) -> Number {
                self.values[0] * self.values[0]
                    + self.values[1] * self.values[1]
                    + self.values[2] * self.values[2]
            }

            /// Measure3d.normalized() -> Measure3d
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new([self.values[0] * k, self.values[1] * k, self.values[2] * k])
            }
        }

        impl<Unit, Number> Default for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d::default() -> Measure3d
            /// It returns the zero vector.
            fn default() -> Self {
                Self::new([Number::ZERO, Number::ZERO, Number::ZERO])
            }
        }

        impl<Unit> From<Measure3d<Unit, f32>> for Measure3d<Unit, f64>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            /// Measure3d<f64>::from(Measure3d<f32>) -> Measure3d<f64>
            fn from(m: Measure3d<Unit, f32>) -> Self {
                Self::new([m.values[0] as f64, m.values[1] as f64, m.values[2] as f64])
            }
        }

        impl<Unit, Number> Neg for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// -Measure3d -> Measure3d
            fn neg(self) -> Self::Output {
                Self::new([-self.values[0], -self.values[1], -self.values[2]])
            }
        }

        impl<Unit, Number> Add<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure3d + Measure3d -> Measure3d
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] + other.values[0],
                    self.values[1] + other.values[1],
                    self.values[2] + other.values[2],
                ])
            }
        }

        impl<Unit, Number> AddAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d += Measure3d
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
                self.values[2] += other.values[2];
            }
        }

        impl<Unit, Number> Sub<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure3d - Measure3d -> Measure3d
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                    self.values[2] - other.values[2],
                ])
            }
        }

        impl<Unit, Number> SubAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d -= Measure3d
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
                self.values[2] -= other.values[2];
            }
        }

        impl<Unit, Number> Mul<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure3d * Number -> Measure3d
            fn mul(self, n: Number) -> Self::Output {
                Self::Output::new([self.values[0] * n, self.values[1] * n, self.values[2] * n])
            }
        }

        impl<Unit, Number> Mul<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure3d * Measure<One> -> Measure3d
            fn mul(self, other: Measure<One, Number>) -> Self::Output {
                Self::new([
                    self.values[0] * other.value,
                    self.values[1] * other.value,
                    self.values[2] * other.value,
                ])
            }
        }

        impl<Unit, Number> Mul<Measure3d<Unit, Number>> for Measure<One, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Measure3d<Unit, Number>;

            /// Measure<One> * Measure3d -> Measure3d
            fn mul(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::Output::new([
                    self.value * other.values[0],
                    self.value * other.values[1],
                    self.value * other.values[2],
                ])
            }
        }

        impl<Unit, Number> MulAssign<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d *= Number
            fn mul_assign(&mut self, n: Number) {
                self.values[0] *= n;
                self.values[1] *= n;
                self.values[2] *= n;
            }
        }

        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d *= Measure<One>
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.values[0] *= other.value;
                self.values[1] *= other.value;
                self.values[2] *= other.value;
            }
        }

        impl<Unit> Mul<Measure3d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            type Output = Measure3d<Unit, f64>;

            /// f64 * Measure3d -> Measure3d
            fn mul(self, other: Measure3d<Unit, f64>) -> Self::Output {
                Self::Output::new([
                    self * other.values[0],
                    self * other.values[1],
                    self * other.values[2],
                ])
            }
        }

        impl<Unit> Mul<Measure3d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
        {
            type Output = Measure3d<Unit, f32>;

            /// f32 * Measure3d -> Measure3d
            fn mul(self, other: Measure3d<Unit, f32>) -> Self::Output {
                Self::Output::new([
                    self * other.values[0],
                    self * other.values[1],
                    self * other.values[2],
                ])
            }
        }

        impl<Unit, Number> Div<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Self;

            /// Measure3d / Number -> Measure3d
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::new([
                    self.values[0] * factor,
                    self.values[1] * factor,
                    self.values[2] * factor,
                ])
            }
        }

        impl<Unit, Number> DivAssign<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d /= Number
            fn div_assign(&mut self, n: Number) {
                let factor = Number::ONE / n;
                self.values[0] *= factor;
                self.values[1] *= factor;
                self.values[2] *= factor;
            }
        }

        impl<Unit, Number> Div<Measure<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            type Output = Measure3d<One, Number>;

            /// Measure3d / Measure -> Measure3d<One>
            fn div(self, other: Measure<Unit, Number>) -> Self::Output {
                Measure3d::new([
                    self.values[0] / other.value,
                    self.values[1] / other.value,
                    self.values[2] / other.value,
                ])
            }
        }

        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d /= Measure<One>
            fn div_assign(&mut self, other: Measure<One, Number>) {
                self.values[0] /= other.value;
                self.values[1] /= other.value;
                self.values[2] /= other.value;
            }
        }

        impl<Unit, Number> PartialEq<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d == Measure3d -> bool
            fn eq(&self, other: &Measure3d<Unit, Number>) -> bool {
                self.values == other.values
            }
        }

        impl<Unit, Number> Clone for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// Measure3d.clone() -> Measure3d
            fn clone(&self) -> Self {
                *self
            }
        }

        /// Measure3d = Measure3d
        impl<Unit, Number> Copy for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
        }

        impl<Unit, Number> fmt::Display for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// format!("{}", Measure3d) -> String
            /// Measure3d.to_string() -> String
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[2], formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        impl<Unit, Number> fmt::Debug for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit<Property: VectorProperty>,
            Number: ArithmeticOps,
        {
            /// format!("{:?}", Measure3d)
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
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
