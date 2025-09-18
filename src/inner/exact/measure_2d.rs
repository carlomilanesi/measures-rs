#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_2d {
    { $with_points:tt $with_directions:tt } => {
        pub struct Measure2d<Unit, Number = f64> {
            pub x: Number,
            pub y: Number,
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            /// measure 2d :: new(number, number) -> measure 2d
            pub const fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            /// measure 2d .x() -> measure
            pub const fn x(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.x)
            }

            /// measure 2d .x() -> measure
            pub const fn y(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.y)
            }

            /// measure 2d .convert() -> measure 2d
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure2d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure2d::<DestUnit, Number> {
                    x: self.x * factor,
                    y: self.y * factor,
                    phantom: PhantomData,
                }
            }

            /// measure 2d .lossy_into() -> measure 2d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    phantom: PhantomData,
                }
            }

            /// measure 2d .lossy_into() -> measure 2d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    phantom: PhantomData,
                }
            }

            /// measure 2d .squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y
            }

            /// measure 2d .normalized() -> number
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new(self.x * k, self.y * k)
            }

            measures::if_all_true! { {$with_points}
                /// Measure2d::from_direction(AnglePoint) -> Measure2d
                pub fn from_direction<AngleUnit: AngleMeasurementUnit>(
                    direction: MeasurePoint<AngleUnit, Number>,
                ) -> Self {
                    let (y, x) = direction.convert::<Radian>().value.sin_cos();
                    Self::new(x, y)
                }
            }

            measures::if_all_true! { {$with_directions}
                /// Measure2d.signed_direction() -> SignedDirection
                pub fn signed_direction<AngleUnit: MeasurementUnit<Property = Angle>>(
                    self,
                ) -> SignedDirection<AngleUnit, Number> {
                    SignedDirection::<Radian, Number>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
                }
            }

            measures::if_all_true! { {$with_directions}
                /// Measure2d.unsigned_direction() -> UnsignedDirection
                pub fn unsigned_direction<AngleUnit: MeasurementUnit<Property = Angle>>(
                    self,
                ) -> UnsignedDirection<AngleUnit, Number> {
                    UnsignedDirection::<Radian, Number>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
                }
            }
        }

        impl<Unit, Number> Default for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::new(Number::ZERO, Number::ZERO)
            }
        }

        // -Measure2d -> Measure2d
        impl<Unit, Number> Neg for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y)
            }
        }

        // Measure2d + Measure2d -> Measure2d
        impl<Unit, Number> Add<Measure2d<Unit, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // Measure2d += Measure2d
        impl<Unit, Number> AddAssign<Measure2d<Unit, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // Measure2d - Measure2d -> Measure2d
        impl<Unit, Number> Sub<Measure2d<Unit, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // Measure2d -= Measure2d
        impl<Unit, Number> SubAssign<Measure2d<Unit, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        // Measure2d * Number -> Measure2d
        impl<Unit, Number> Mul<Number> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n)
            }
        }

        // Measure2d * Measure<One> -> Measure2d
        impl<Unit, Number> Mul<Measure<One, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, other: Measure<One, Number>) -> Self::Output {
                Self::new(self.x * other.value, self.y * other.value)
            }
        }

        // Measure2d *= Number
        impl<Unit, Number> MulAssign<Number> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
            }
        }

        // Measure2d *= Measure<One, Number>
        impl<Unit, Number> MulAssign<Measure<One, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, other: Measure<One, Number>) {
                self.x *= other.value;
                self.y *= other.value;
            }
        }

        // f64 * Measure2d -> Measure2d
        impl<Unit> Mul<Measure2d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = Measure2d<Unit, f64>;
            fn mul(self, other: Measure2d<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y)
            }
        }

        // f32 * Measure2d -> Measure2d
        impl<Unit> Mul<Measure2d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = Measure2d<Unit, f32>;
            fn mul(self, other: Measure2d<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y)
            }
        }

        // Measure2d / Number -> Measure2d
        impl<Unit, Number> Div<Number> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n)
            }
        }

        // Measure2d /= Number
        impl<Unit, Number> DivAssign<Number> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
            }
        }

        // Measure2d /= Measure<One>
        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: Measure<One, Number>) {
                self.x /= other.value;
                self.y /= other.value;
            }
        }

        // Measure2d == Measure2d -> bool
        impl<Unit, Number> PartialEq<Measure2d<Unit, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &Measure2d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        // Measure2d.clone() -> Measure2d
        impl<Unit, Number> Clone for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure2d = Measure2d
        impl<Unit, Number> Copy for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", Measure2d)
        impl<Unit, Number> fmt::Display for Measure2d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", Measure2d)
        impl<Unit, Number> fmt::Debug for Measure2d<Unit, Number>
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
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
