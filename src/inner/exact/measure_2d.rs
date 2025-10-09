#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_measure_2d {
    { $with_points:tt $with_directions:tt $with_approx:ident } => {
        pub struct Measure2d<Unit, Number = f64> {
            pub values: [Number; 2],
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            /// Measure2d::new([Number; 2]) -> Measure2d
            pub const fn new(values: [Number; 2]) -> Self {
                Self {
                    values,
                    phantom: PhantomData,
                }
            }

            /// Measure2d.x() -> Measure
            pub const fn x(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.values[0])
            }

            /// Measure2d.y() -> Measure
            pub const fn y(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.values[1])
            }

            measures::if_all_true! { {$with_approx}
                /*TODO
                /// Measure2d::from_approx_measure(ApproxMeasure2d) -> Measure2d
                pub const fn from_approx_measure(approx_measure: ApproxMeasure2d<Unit, Number>) -> Self {
                    Self::new(approx_measure.values)
                }

                /// Measure2d.to_approx_measure_with_variance(Number) -> ApproxMeasure2d
                pub fn to_approx_measure_with_variance(self, variance: Number) -> ApproxMeasure2d<Unit, Number> {
                    ApproxMeasure2d::<Unit, Number>::with_variance(self.value, variance)
                }

                /// Measure2d.to_approx_measure_with_uncertainty(Measure) -> ApproxMeasure2d
                pub fn to_approx_measure_with_uncertainty(self, uncertainty: Measure<Unit, Number>) -> ApproxMeasure2d<Unit, Number> {
                    ApproxMeasure2d::<Unit, Number>::with_uncertainty(self.value, uncertainty)
                }
                */
            }

            /// Measure2d.convert() -> Measure2d
            pub fn convert<DestUnit>(&self) -> Measure2d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure2d::<DestUnit, Number>::new([self.values[0] * factor, self.values[1] * factor])
            }

            /// Measure2d.lossless_into() -> Measure2d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber>::new([
                    DestNumber::from(self.values[0]),
                    DestNumber::from(self.values[1]),
                ])
            }

            /// Measure2d.lossy_into() -> Measure2d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber>::new([
                    DestNumber::lossy_from(self.values[0]),
                    DestNumber::lossy_from(self.values[1]),
                ])
            }

            /// Measure2d.norm() -> Measure
            pub fn norm(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(
                    (self.values[0] * self.values[0] + self.values[1] * self.values[1]).sqrt(),
                )
            }

            /// Measure2d.squared_norm() -> Number
            pub fn squared_norm(self) -> Number {
                self.values[0] * self.values[0] + self.values[1] * self.values[1]
            }

            /// Measure2d.normalized() -> Measure2d
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new([self.values[0] * k, self.values[1] * k])
            }

            measures::if_all_true! { {$with_points}
                /// Measure2d::from_direction_measure(AnglePoint) -> Measure2d
                pub fn from_direction<AngleUnit>(
                    direction: MeasurePoint<AngleUnit, Number>,
                ) -> Self
                where
                AngleUnit: AngleMeasurementUnit,
                {
                    let (y, x) = direction.convert::<Radian>().sin_cos();
                    Self::new([x, y])
                }

                /// Measure2d.direction_measure() -> MeasurePoint
                pub fn direction_measure<AngleUnit: MeasurementUnit<Property = Angle>>(
                    self,
                ) -> MeasurePoint<AngleUnit, Number> {
                    MeasurePoint::<Radian, Number>::new(self.values[1].atan2(self.values[0])).convert::<AngleUnit>()
                }
            }

            measures::if_all_true! { {$with_directions}
                /// Measure2d::from_signed_direction(SignedDirection) -> Measure2d
                pub fn from_signed_direction<AngleUnit>(
                    direction: SignedDirection<AngleUnit, Number>,
                ) -> Self
                where
                AngleUnit: AngleMeasurementUnit,
                {
                    let (y, x) = direction.convert::<Radian>().sin_cos();
                    Self::new([x, y])
                }

                /// Measure2d::from_unsigned_direction(UnsignedDirection) -> Measure2d
                pub fn from_unsigned_direction<AngleUnit>(
                    direction: UnsignedDirection<AngleUnit, Number>,
                ) -> Self
                where
                AngleUnit: AngleMeasurementUnit,
                {
                    let (y, x) = direction.convert::<Radian>().sin_cos();
                    Self::new([x, y])
                }

                /// Measure2d.signed_direction() -> SignedDirection
                pub fn signed_direction<AngleUnit: MeasurementUnit<Property = Angle>>(
                    self,
                ) -> SignedDirection<AngleUnit, Number> {
                    SignedDirection::<Radian, Number>::new(self.values[1].atan2(self.values[0])).convert::<AngleUnit>()
                }

                /// Measure2d.unsigned_direction() -> UnsignedDirection
                pub fn unsigned_direction<AngleUnit: MeasurementUnit<Property = Angle>>(
                    self,
                ) -> UnsignedDirection<AngleUnit, Number> {
                    UnsignedDirection::<Radian, Number>::new(self.values[1].atan2(self.values[0])).convert::<AngleUnit>()
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
                Self::new([Number::ZERO, Number::ZERO])
            }
        }

        impl<Unit> From<Measure2d<Unit, f32>> for Measure2d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(measure: Measure2d<Unit, f32>) -> Self {
                Self::new([measure.values[0] as f64, measure.values[1] as f64])
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
                Self::new([-self.values[0], -self.values[1]])
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
                Self::new([
                    self.values[0] + other.values[0],
                    self.values[1] + other.values[1],
                ])
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
                self.values[0] += other.values[0];
                self.values[1] += other.values[1];
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
                Self::new([
                    self.values[0] - other.values[0],
                    self.values[1] - other.values[1],
                ])
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
                self.values[0] -= other.values[0];
                self.values[1] -= other.values[1];
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
                Self::new([self.values[0] * n, self.values[1] * n])
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
                Self::new([self.values[0] * other.value, self.values[1] * other.value])
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
                self.values[0] *= n;
                self.values[1] *= n;
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
                self.values[0] *= other.value;
                self.values[1] *= other.value;
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
                Self::Output::new([self * other.values[0], self * other.values[1]])
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
                Self::Output::new([self * other.values[0], self * other.values[1]])
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
                Self::new([self.values[0] / n, self.values[1] / n])
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
                self.values[0] /= n;
                self.values[1] /= n;
            }
        }

        // Measure2d /= Measure<One>
        impl<Unit, Number> DivAssign<Measure<One, Number>> for Measure2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn div_assign(&mut self, other: Measure<One, Number>) {
                self.values[0] /= other.value;
                self.values[1] /= other.value;
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
                self.values == other.values
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
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
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
                fmt::Display::fmt(&self.values[0], formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.values[1], formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
