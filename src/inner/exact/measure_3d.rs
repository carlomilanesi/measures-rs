#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_3d {
    {$with_approx:ident} => {
        pub struct Measure3d<Unit, Number: ArithmeticOps = f64> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Unit, Number> Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// measure 3d :: new(number, number, number) -> measure 3d
            pub const fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }

            /// measure 3d .x() -> measure
            pub const fn x(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.x)
            }

            /// measure 3d .y() -> measure
            pub const fn y(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.y)
            }

            /// measure 3d .z() -> measure
            pub const fn z(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.z)
            }

            /// measure 3d .convert() -> measure 3d
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure3d::<DestUnit, Number> {
                    x: self.x * factor,
                    y: self.y * factor,
                    z: self.z * factor,
                    phantom: PhantomData,
                }
            }

            /// measure 3d .lossy_into() -> measure 3d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure3d<Unit, DestNumber> {
                Measure3d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    z: DestNumber::lossy_from(self.z),
                    phantom: PhantomData,
                }
            }

            /// Measure3d.norm() -> Measure
            pub fn norm(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new((self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
            }

            /// Measure3d.squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            /// Measure3d.normalized() -> Measure3d
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new(self.x * k, self.y * k, self.z * k)
            }
        }

        impl<Unit, Number> Default for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::new(Number::ZERO, Number::ZERO, Number::ZERO)
            }
        }

        impl<Unit> From<Measure3d<Unit, f32>> for Measure3d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: Measure3d<Unit, f32>) -> Self {
                Measure3d::<Unit, f64>::new(m.x as f64, m.y as f64, m.z as f64)
            }
        }

        // -Measure3d -> Measure3d
        impl<Unit, Number> Neg for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }

        // Measure3d + Measure3d -> Measure3d
        impl<Unit, Number> Add<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // Measure3d += Measure3d
        impl<Unit, Number> AddAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // Measure3d - Measure3d -> Measure3d
        impl<Unit, Number> Sub<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // Measure3d -= Measure3d
        impl<Unit, Number> SubAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        // Measure3d * Number -> Measure3d
        impl<Unit, Number> Mul<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::Output::new(self.x * n, self.y * n, self.z * n)
            }
        }

        // Measure3d * Measure<One> -> Measure3d
        impl<Unit, Number> Mul<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, other: Measure<One, Number>) -> Self::Output {
                Self::Output::new(
                    self.x * other.value,
                    self.y * other.value,
                    self.z * other.value,
                )
            }
        }

        // Measure3d *= Number
        impl<Unit, Number> MulAssign<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
                self.z *= n;
            }
        }

        // Measure3d *= Measure<One, Number>
        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.x *= other.value;
                self.y *= other.value;
                self.z *= other.value;
            }
        }

        // f64 * Measure3d -> Measure3d
        impl<Unit> Mul<Measure3d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, f64>;
            fn mul(self, other: Measure3d<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // f32 * Measure3d -> Measure3d
        impl<Unit> Mul<Measure3d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, f32>;
            fn mul(self, other: Measure3d<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // Measure3d / Number -> Measure3d
        impl<Unit, Number> Div<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::new(self.x * factor, self.y * factor, self.z * factor)
            }
        }

        // Measure3d /= Number
        impl<Unit, Number> DivAssign<Number> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
                self.z /= n;
            }
        }

        // Measure3d /= Measure<One>
        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: Measure<One, Number>) {
                self.x /= other.value;
                self.y /= other.value;
                self.z /= other.value;
            }
        }

        // Measure3d == Measure3d -> bool
        impl<Unit, Number> PartialEq<Measure3d<Unit, Number>> for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &Measure3d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        // Measure3d.clone() -> Measure3d
        impl<Unit, Number> Clone for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure3d = Measure3d
        impl<Unit, Number> Copy for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", Measure3d)
        impl<Unit, Number> fmt::Display for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", Measure3d)
        impl<Unit, Number> fmt::Debug for Measure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
