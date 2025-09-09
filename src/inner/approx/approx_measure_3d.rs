#[macro_export]
macro_rules! inner_define_approx_measure_3d {
    {$with_approx:ident} => {
        /// Approximate 3d measurement with static unit of measurement and value type,
        /// and with dynamic values and variance.
        pub struct ApproxMeasure3d<Unit, Number: ArithmeticOps = f64> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            pub variance: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Unit, Number> ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// Measure3d::new_with_variance(number, number, number, number) -> Measure3d
            pub const fn new_with_variance(x: Number, y: Number, z: Number, variance: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    variance,
                    phantom: PhantomData,
                }
            }

            /// Measure3d::new_with_uncertainty(number, number, number, number) -> Measure3d
            pub fn new_with_uncertainty(
                x: Number,
                y: Number,
                z: Number,
                uncertainty: Measure<Unit, Number>,
            ) -> Self {
                Self::new_with_variance(x, y, z, uncertainty.value * uncertainty.value)
            }

            /// Measure3d.x() -> Measure
            pub fn x(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::new_with_variance(
                    self.x,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.y() -> Measure
            pub fn y(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::new_with_variance(
                    self.y,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.z() -> Measure
            pub fn z(self) -> ApproxMeasure<Unit, Number> {
                ApproxMeasure::<Unit, Number>::new_with_variance(
                    self.z,
                    self.variance / (Number::ONE + Number::ONE + Number::ONE),
                )
            }

            /// Measure3d.convert() -> Measure3d
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> ApproxMeasure3d<DestUnit, Number> {
                let ratio = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                ApproxMeasure3d::<DestUnit, Number>::new_with_variance(
                    self.x * ratio,
                    self.y * ratio,
                    self.z * ratio,
                    self.variance * ratio * ratio,
                )
            }

            /// Measure3d.lossless_into() -> Measure3d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> ApproxMeasure3d<Unit, DestNumber> {
                ApproxMeasure3d::<Unit, DestNumber>::new_with_variance(
                    DestNumber::from(self.x),
                    DestNumber::from(self.y),
                    DestNumber::from(self.z),
                    DestNumber::from(self.variance),
                )
            }

            /// measure 3d .lossy_into() -> measure 3d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> ApproxMeasure3d<Unit, DestNumber> {
                ApproxMeasure3d::<Unit, DestNumber>::new_with_variance(
                    DestNumber::lossy_from(self.x),
                    DestNumber::lossy_from(self.y),
                    DestNumber::lossy_from(self.z),
                    DestNumber::lossy_from(self.variance),
                )
            }

            /// measure 3d .squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            /// measure 3d .normalized() -> number
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new_with_variance(self.x * k, self.y * k, self.z * k, self.variance * k * k)
            }
        }

        impl<Unit, Number> Default for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::new_with_variance(Number::ZERO, Number::ZERO, Number::ZERO, Number::ZERO)
            }
        }

        // -Measure3d -> Measure3d
        impl<Unit, Number> Neg for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new_with_variance(-self.x, -self.y, -self.z, self.variance)
            }
        }

        // ApproxMeasure3d + ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Add<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
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

        // ApproxMeasure3d += ApproxMeasure3d
        impl<Unit, Number> AddAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
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

        // ApproxMeasure3d - ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit, Number> Sub<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
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

        // ApproxMeasure3d -= ApproxMeasure3d
        impl<Unit, Number> SubAssign<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
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

        // ApproxMeasure3d * Number -> ApproxMeasure3d
        impl<Unit, Number> Mul<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new_with_variance(self.x * n, self.y * n, self.z * n, self.variance * n * n)
            }
        }

        // ApproxMeasure3d *= Number
        impl<Unit, Number> MulAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
                self.z *= n;
                self.variance *= n * n;
            }
        }

        // f64 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, f64>;
            fn mul(self, other: ApproxMeasure3d<Unit, f64>) -> Self::Output {
                Self::Output::new_with_variance(
                    self * other.x,
                    self * other.y,
                    self * other.z,
                    self * self * other.variance,
                )
            }
        }

        // f32 * ApproxMeasure3d -> ApproxMeasure3d
        impl<Unit> Mul<ApproxMeasure3d<Unit, f32>> for f32
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            type Output = ApproxMeasure3d<Unit, f32>;
            fn mul(self, other: ApproxMeasure3d<Unit, f32>) -> Self::Output {
                Self::Output::new_with_variance(
                    self * other.x,
                    self * other.y,
                    self * other.z,
                    self * self * other.variance,
                )
            }
        }

        // ApproxMeasure3d / Number -> ApproxMeasure3d
        impl<Unit, Number> Div<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::new_with_variance(
                    self.x * factor,
                    self.y * factor,
                    self.z * factor,
                    self.variance * factor,
                )
            }
        }

        // ApproxMeasure3d /= Number
        impl<Unit, Number> DivAssign<Number> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
                self.z /= n;
                self.variance /= n;
            }
        }

        // ApproxMeasure3d == ApproxMeasure3d -> bool
        impl<Unit, Number> PartialEq<ApproxMeasure3d<Unit, Number>> for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &ApproxMeasure3d<Unit, Number>) -> bool {
                self.x == other.x
                    && self.y == other.y
                    && self.z == other.z
                    && self.variance == other.variance
            }
        }

        // ApproxMeasure3d.clone() -> ApproxMeasure3d
        impl<Unit, Number> Clone for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // ApproxMeasure3d = ApproxMeasure3d
        impl<Unit, Number> Copy for ApproxMeasure3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
        }

        // format!("{}", ApproxMeasure3d)
        impl<Unit, Number> fmt::Display for ApproxMeasure3d<Unit, Number>
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
                formatter.write_str(")\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", ApproxMeasure3d)
        impl<Unit, Number> fmt::Debug for ApproxMeasure3d<Unit, Number>
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
                formatter.write_str(")\u{b1}")?;
                fmt::Display::fmt(&self.variance.sqrt(), formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
