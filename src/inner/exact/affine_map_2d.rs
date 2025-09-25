#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_affine_map_2d {
    {} => {
        pub struct AffineMap2d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            c: [[Number; 3]; 2],
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> AffineMap2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            pub const fn new(coefficients: [[Number; 3]; 2]) -> Self {
                Self {
                    c: coefficients,
                    phantom: PhantomData,
                }
            }

            // Unit conversion.
            pub fn convert<DestUnit>(&self) -> AffineMap2d<DestUnit, Number>
            where
                DestUnit: MeasurementUnit<Property = Unit::Property>,
            {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                AffineMap2d::<DestUnit, Number>::new([
                    [self.c[0][0], self.c[0][1], self.c[0][2] * factor],
                    [self.c[1][0], self.c[1][1], self.c[1][2] * factor],
                ])
            }

            // Translation.
            pub fn translation(v: Measure2d<Unit, Number>) -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, v.values[0]],
                    [Number::ZERO, Number::ONE, v.values[1]],
                ])
            }

            // Rotation about a point by an angle measure.
            pub fn rotation<AngleUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                angle: Measure<AngleUnit, Number>,
            ) -> Self
            where
                AngleUnit: AngleMeasurementUnit,
            {
                Self::rotation_by_radians(fixed_point.values, angle.convert::<Radian>().value)
            }

            pub fn rotation_at_right(fixed_point: MeasurePoint2d<Unit, Number>) -> Self {
                Self::right_rotation_by_sin(fixed_point.values, -Number::ONE)
            }

            pub fn rotation_at_left(fixed_point: MeasurePoint2d<Unit, Number>) -> Self {
                Self::right_rotation_by_sin(fixed_point.values, Number::ONE)
            }

            // Projections

            // Projection onto a line identified by a fixed point
            // and a point angle.
            pub fn projection_by_point_angle<AngleUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: MeasurePoint<AngleUnit, Number>,
            ) -> Self
            where
                AngleUnit: MeasurementUnit<Property = Angle>,
            {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a signed direction.
            pub fn projection_by_signed_direction<AngleUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: SignedDirection<AngleUnit, Number>,
            ) -> Self
            where
                AngleUnit: AngleMeasurementUnit,
            {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and an unsigned direction.
            pub fn projection_by_unsigned_direction<AngleUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: UnsignedDirection<AngleUnit, Number>,
            ) -> Self
            where
                AngleUnit: AngleMeasurementUnit,
            {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn projection_by_unit_vector(
                fixed_point: MeasurePoint2d<Unit, Number>,
                uv: Measure2d<Unit, Number>,
            ) -> Self {
                Self::projection_by_cos_sin(fixed_point.values, uv.values[0], uv.values[1])
            }

            // Reflections

            // Reflection over a line identified by a fixed point
            // and a point angle.
            pub fn reflection_by_point_angle<AngleUnit: MeasurementUnit<Property = Angle>>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: MeasurePoint<AngleUnit, Number>,
            ) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: SignedDirection<AngleUnit, Number>,
            ) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit>(
                fixed_point: MeasurePoint2d<Unit, Number>,
                direction: UnsignedDirection<AngleUnit, Number>,
            ) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(fixed_point.values, cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn reflection_by_unit_vector(
                fixed_point: MeasurePoint2d<Unit, Number>,
                uv: Measure2d<Unit, Number>,
            ) -> Self {
                Self::reflection_by_cos_sin(fixed_point.values, uv.values[0], uv.values[1])
            }

            // Scaling by two factors from a fixed point.
            pub fn scaling(fixed_point: MeasurePoint2d<Unit, Number>, kx: Number, ky: Number) -> Self {
                Self::new([
                    [kx, Number::ZERO, fixed_point.values[0] * (Number::ONE - kx)],
                    [Number::ZERO, ky, fixed_point.values[1] * (Number::ONE - ky)],
                ])
            }

            pub fn inverted(&self) -> Self {
                let inv_determinant =
                    Number::ONE / (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]);
                Self::new([
                    [
                        self.c[1][1] * inv_determinant,
                        self.c[0][1] * -inv_determinant,
                        (self.c[0][1] * self.c[1][2] - self.c[0][2] * self.c[1][1]) * inv_determinant,
                    ],
                    [
                        self.c[1][0] * -inv_determinant,
                        self.c[0][0] * inv_determinant,
                        (self.c[0][2] * self.c[1][0] - self.c[0][0] * self.c[1][2]) * inv_determinant,
                    ],
                ])
            }

            // Composition of two plane affine transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &AffineMap2d<Unit, Number>) -> Self {
                Self::new([
                    [
                        other.c[0][0] * self.c[0][0] + other.c[0][1] * self.c[1][0],
                        other.c[0][0] * self.c[0][1] + other.c[0][1] * self.c[1][1],
                        other.c[0][0] * self.c[0][2] + other.c[0][1] * self.c[1][2] + other.c[0][2],
                    ],
                    [
                        other.c[1][0] * self.c[0][0] + other.c[1][1] * self.c[1][0],
                        other.c[1][0] * self.c[0][1] + other.c[1][1] * self.c[1][1],
                        other.c[1][0] * self.c[0][2] + other.c[1][1] * self.c[1][2] + other.c[1][2],
                    ],
                ])
            }

            pub fn apply_to(&self, m: MeasurePoint2d<Unit, Number>) -> MeasurePoint2d<Unit, Number> {
                MeasurePoint2d::<Unit, Number>::new([
                    self.c[0][0] * m.values[0] + self.c[0][1] * m.values[1] + self.c[0][2],
                    self.c[1][0] * m.values[0] + self.c[1][1] * m.values[1] + self.c[1][2],
                ])
            }

            fn rotation_by_radians(fp: [Number; 2], radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::new([
                    [cos_a, -sin_a, fp[0] - cos_a * fp[0] + sin_a * fp[1]],
                    [sin_a, cos_a, fp[1] - sin_a * fp[0] - cos_a * fp[1]],
                ])
            }

            fn right_rotation_by_sin(fp: [Number; 2], sine: Number) -> Self {
                Self::new([
                    [Number::ZERO, -sine, fp[0] + sine * fp[1]],
                    [sine, Number::ZERO, fp[1] - sine * fp[0]],
                ])
            }

            fn projection_by_cos_sin(fp: [Number; 2], cos_a: Number, sin_a: Number) -> Self {
                let cc = cos_a * cos_a;
                let cs = cos_a * sin_a;
                let ss = sin_a * sin_a;
                let sxmcy = sin_a * fp[0] - cos_a * fp[1];
                Self::new([[cc, cs, sin_a * sxmcy], [cs, ss, -cos_a * sxmcy]])
            }

            fn reflection_by_cos_sin(fp: [Number; 2], cos_a: Number, sin_a: Number) -> Self {
                let c2ms2 = cos_a * cos_a - sin_a * sin_a;
                let two = Number::ONE + Number::ONE;
                let cs_bis = two * cos_a * sin_a;
                let sxmcy_bis = two * (sin_a * fp[0] - cos_a * fp[1]);
                Self::new([
                    [c2ms2, cs_bis, sin_a * sxmcy_bis],
                    [cs_bis, -c2ms2, -cos_a * sxmcy_bis],
                ])
            }
        }

        impl<Unit, Number> Default for AffineMap2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the identity transformation.
            fn default() -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, Number::ZERO],
                    [Number::ZERO, Number::ONE, Number::ZERO],
                ])
            }
        }

        impl<Unit> From<AffineMap2d<Unit, f32>> for AffineMap2d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: AffineMap2d<Unit, f32>) -> Self {
                Self::new([
                    [m.c[0][0] as f64, m.c[0][1] as f64, m.c[0][2] as f64],
                    [m.c[1][0] as f64, m.c[1][1] as f64, m.c[1][2] as f64],
                ])
            }
        }

        // format!("{}", AffineMap2d)
        impl<Unit, Number> fmt::Display for AffineMap2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<2, 3, Number>(&self.c, Unit::SUFFIX, 1)
                )
            }
        }

        // format!("{:?}", AffineMap2d)
        impl<Unit, Number> fmt::Debug for AffineMap2d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<2, 3, Number>(&self.c, Unit::SUFFIX, 1)
                )
            }
        }
    };
}
