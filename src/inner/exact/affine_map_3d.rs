#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_affine_map_3d {
    {} => {
        /// Affine transformation of `MeasurePoint3d` objects.
        pub struct AffineMap3d<Unit, Number = f64>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            c: [[Number; 4]; 3],
            phantom: core::marker::PhantomData<Unit>,
        }

        impl<Unit, Number> AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            /// Create an AffineMap3d from its 12 coefficients.
            pub const fn new(coefficients: [[Number; 4]; 3]) -> Self {
                Self {
                    c: coefficients,
                    phantom: PhantomData,
                }
            }

            // Unit conversion.
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> AffineMap3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                AffineMap3d::<DestUnit, Number>::new([
                    [
                        self.c[0][0],
                        self.c[0][1],
                        self.c[0][2],
                        self.c[0][3] * factor,
                    ],
                    [
                        self.c[1][0],
                        self.c[1][1],
                        self.c[1][2],
                        self.c[1][3] * factor,
                    ],
                    [
                        self.c[2][0],
                        self.c[2][1],
                        self.c[2][2],
                        self.c[2][3] * factor,
                    ],
                ])
            }

            // Translation

            pub fn translation(v: Measure3d<Unit, Number>) -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, Number::ZERO, v.values[0]],
                    [Number::ZERO, Number::ONE, Number::ZERO, v.values[1]],
                    [Number::ZERO, Number::ZERO, Number::ONE, v.values[2]],
                ])
            }

            // Rotations

            // Rotation by an angle measure around a unit vector
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn rotation<AngleUnit: AngleMeasurementUnit, AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
                angle: Measure<AngleUnit, Number>,
            ) -> Self
            where
                AxisUnit::Property: VectorProperty,
            {
                let fpx = fixed_point.values[0];
                let fpy = fixed_point.values[1];
                let fpz = fixed_point.values[2];
                let ux = unit_vector.values[0];
                let uy = unit_vector.values[1];
                let uz = unit_vector.values[2];
                let a = angle.convert::<Radian>().value;
                let (sin_a, cos_a) = a.sin_cos();
                let one_minus_cos_a = Number::ONE - cos_a;
                let c00 = cos_a + ux * ux * one_minus_cos_a;
                let c01 = ux * uy * one_minus_cos_a - uz * sin_a;
                let c02 = ux * uz * one_minus_cos_a + uy * sin_a;
                let c10 = uy * ux * one_minus_cos_a + uz * sin_a;
                let c11 = cos_a + uy * uy * one_minus_cos_a;
                let c12 = uy * uz * one_minus_cos_a - ux * sin_a;
                let c20 = uz * ux * one_minus_cos_a - uy * sin_a;
                let c21 = uz * uy * one_minus_cos_a + ux * sin_a;
                let c22 = cos_a + uz * uz * one_minus_cos_a;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Projections

            // Projection onto a line identified by a unit vector
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_line<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let fpx = fixed_point.values[0];
                let fpy = fixed_point.values[1];
                let fpz = fixed_point.values[2];
                let ux = unit_vector.values[0];
                let uy = unit_vector.values[1];
                let uz = unit_vector.values[2];
                Self::new([
                    [
                        ux * ux,
                        uy * ux,
                        uz * ux,
                        fpx - fpx * ux * ux - fpy * uy * ux - fpz * uz * ux,
                    ],
                    [
                        ux * uy,
                        uy * uy,
                        uz * uy,
                        fpy - fpx * ux * uy - fpy * uy * uy - fpz * uz * uy,
                    ],
                    [
                        ux * uz,
                        uy * uz,
                        uz * uz,
                        fpz - fpx * ux * uz - fpy * uy * uz - fpz * uz * uz,
                    ],
                ])
            }

            // Projection onto a plane whose normal is identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_plane<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let fpx = fixed_point.values[0];
                let fpy = fixed_point.values[1];
                let fpz = fixed_point.values[2];
                let ux = unit_vector.values[0];
                let uy = unit_vector.values[1];
                let uz = unit_vector.values[2];
                let c00 = Number::ONE - unit_vector.values[0] * unit_vector.values[0];
                let c01 = -unit_vector.values[1] * unit_vector.values[0];
                let c02 = -unit_vector.values[2] * unit_vector.values[0];
                let c10 = -unit_vector.values[0] * unit_vector.values[1];
                let c11 = Number::ONE - unit_vector.values[1] * unit_vector.values[1];
                let c12 = -unit_vector.values[2] * unit_vector.values[1];
                let c20 = -unit_vector.values[0] * unit_vector.values[2];
                let c21 = -unit_vector.values[1] * unit_vector.values[2];
                let c22 = Number::ONE - unit_vector.values[2] * unit_vector.values[2];
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Reflections

            // Reflection over a line identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_line<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let two = Number::ONE + Number::ONE;
                let fpx = fixed_point.values[0];
                let fpy = fixed_point.values[1];
                let fpz = fixed_point.values[2];
                let ux = unit_vector.values[0];
                let uy = unit_vector.values[1];
                let uz = unit_vector.values[2];
                let c00 = two * ux * ux - Number::ONE;
                let c01 = two * uy * ux;
                let c02 = two * uz * ux;
                let c10 = two * ux * uy;
                let c11 = two * uy * uy - Number::ONE;
                let c12 = two * uz * uy;
                let c20 = two * ux * uz;
                let c21 = two * uy * uz;
                let c22 = two * uz * uz - Number::ONE;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Reflection over a plane whose normal is identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_plane<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let minus_two = -(Number::ONE + Number::ONE);
                let fpx = fixed_point.values[0];
                let fpy = fixed_point.values[1];
                let fpz = fixed_point.values[2];
                let ux = unit_vector.values[0];
                let uy = unit_vector.values[1];
                let uz = unit_vector.values[2];
                let c00 = minus_two * unit_vector.values[0] * unit_vector.values[0] + Number::ONE;
                let c01 = minus_two * unit_vector.values[1] * unit_vector.values[0];
                let c02 = minus_two * unit_vector.values[2] * unit_vector.values[0];
                let c10 = minus_two * unit_vector.values[0] * unit_vector.values[1];
                let c11 = minus_two * unit_vector.values[1] * unit_vector.values[1] + Number::ONE;
                let c12 = minus_two * unit_vector.values[2] * unit_vector.values[1];
                let c20 = minus_two * unit_vector.values[0] * unit_vector.values[2];
                let c21 = minus_two * unit_vector.values[1] * unit_vector.values[2];
                let c22 = minus_two * unit_vector.values[2] * unit_vector.values[2] + Number::ONE;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Scaling by three factors from a point.

            pub fn scaling(fixed_point: MeasurePoint3d<Unit, Number>, factors: [Number; 3]) -> Self {
                Self::new([
                    [
                        factors[0],
                        Number::ZERO,
                        Number::ZERO,
                        fixed_point.values[0] * (Number::ONE - factors[0]),
                    ],
                    [
                        Number::ZERO,
                        factors[1],
                        Number::ZERO,
                        fixed_point.values[1] * (Number::ONE - factors[1]),
                    ],
                    [
                        Number::ZERO,
                        Number::ZERO,
                        factors[2],
                        fixed_point.values[2] * (Number::ONE - factors[2]),
                    ],
                ])
            }

            // Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE
                    / (self.c[0][0] * (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1])
                        - self.c[0][1] * (self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0])
                        + self.c[0][2] * (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]));
                let c00 = (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1]) * inv_determinant;
                let c01 = -(self.c[0][1] * self.c[2][2] - self.c[0][2] * self.c[2][1]) * inv_determinant;
                let c02 = (self.c[0][1] * self.c[1][2] - self.c[0][2] * self.c[1][1]) * inv_determinant;
                let c10 = -(self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0]) * inv_determinant;
                let c11 = (self.c[0][0] * self.c[2][2] - self.c[0][2] * self.c[2][0]) * inv_determinant;
                let c12 = -(self.c[0][0] * self.c[1][2] - self.c[0][2] * self.c[1][0]) * inv_determinant;
                let c20 = (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]) * inv_determinant;
                let c21 = -(self.c[0][0] * self.c[2][1] - self.c[0][1] * self.c[2][0]) * inv_determinant;
                let c22 = (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]) * inv_determinant;
                Self::new([
                    [
                        c00,
                        c01,
                        c02,
                        -c00 * self.c[0][3] - c01 * self.c[1][3] - c02 * self.c[2][3],
                    ],
                    [
                        c10,
                        c11,
                        c12,
                        -c10 * self.c[0][3] - c11 * self.c[1][3] - c12 * self.c[2][3],
                    ],
                    [
                        c20,
                        c21,
                        c22,
                        -c20 * self.c[0][3] - c21 * self.c[1][3] - c22 * self.c[2][3],
                    ],
                ])
            }

            // Composition of spacial affine transformations.
            // To apply the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &AffineMap3d<Unit, Number>) -> Self {
                Self::new([
                    [
                        other.c[0][0] * self.c[0][0]
                            + other.c[0][1] * self.c[1][0]
                            + other.c[0][2] * self.c[2][0],
                        other.c[0][0] * self.c[0][1]
                            + other.c[0][1] * self.c[1][1]
                            + other.c[0][2] * self.c[2][1],
                        other.c[0][0] * self.c[0][2]
                            + other.c[0][1] * self.c[1][2]
                            + other.c[0][2] * self.c[2][2],
                        other.c[0][0] * self.c[0][3]
                            + other.c[0][1] * self.c[1][3]
                            + other.c[0][2] * self.c[2][3]
                            + other.c[0][3],
                    ],
                    [
                        other.c[1][0] * self.c[0][0]
                            + other.c[1][1] * self.c[1][0]
                            + other.c[1][2] * self.c[2][0],
                        other.c[1][0] * self.c[0][1]
                            + other.c[1][1] * self.c[1][1]
                            + other.c[1][2] * self.c[2][1],
                        other.c[1][0] * self.c[0][2]
                            + other.c[1][1] * self.c[1][2]
                            + other.c[1][2] * self.c[2][2],
                        other.c[1][0] * self.c[0][3]
                            + other.c[1][1] * self.c[1][3]
                            + other.c[1][2] * self.c[2][3]
                            + other.c[1][3],
                    ],
                    [
                        other.c[2][0] * self.c[0][0]
                            + other.c[2][1] * self.c[1][0]
                            + other.c[2][2] * self.c[2][0],
                        other.c[2][0] * self.c[0][1]
                            + other.c[2][1] * self.c[1][1]
                            + other.c[2][2] * self.c[2][1],
                        other.c[2][0] * self.c[0][2]
                            + other.c[2][1] * self.c[1][2]
                            + other.c[2][2] * self.c[2][2],
                        other.c[2][0] * self.c[0][3]
                            + other.c[2][1] * self.c[1][3]
                            + other.c[2][2] * self.c[2][3]
                            + other.c[2][3],
                    ],
                ])
            }

            pub fn apply_to(&self, m: MeasurePoint3d<Unit, Number>) -> MeasurePoint3d<Unit, Number> {
                MeasurePoint3d::<Unit, Number>::new([
                    self.c[0][0] * m.values[0]
                        + self.c[0][1] * m.values[1]
                        + self.c[0][2] * m.values[2]
                        + self.c[0][3],
                    self.c[1][0] * m.values[0]
                        + self.c[1][1] * m.values[1]
                        + self.c[1][2] * m.values[2]
                        + self.c[1][3],
                    self.c[2][0] * m.values[0]
                        + self.c[2][1] * m.values[1]
                        + self.c[2][2] * m.values[2]
                        + self.c[2][3],
                ])
            }
        }

        impl<Unit, Number> Default for AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the identity transformation.
            fn default() -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, Number::ZERO, Number::ZERO],
                    [Number::ZERO, Number::ONE, Number::ZERO, Number::ZERO],
                    [Number::ZERO, Number::ZERO, Number::ONE, Number::ZERO],
                ])
            }
        }

        // AffineMap3d == AffineMap3d -> bool
        impl<Unit, Number> PartialEq<AffineMap3d<Unit, Number>> for AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &AffineMap3d<Unit, Number>) -> bool {
                self.c == other.c
            }
        }

        // AffineMap3d.clone() -> AffineMap3d
        impl<Unit, Number> Clone for AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                Self::new(self.c.clone())
            }
        }

        impl<Unit> From<AffineMap3d<Unit, f32>> for AffineMap3d<Unit, f64>
        where
            Unit: MeasurementUnit,
            Unit::Property: VectorProperty,
        {
            fn from(m: AffineMap3d<Unit, f32>) -> Self {
                Self::new([
                    [
                        m.c[0][0] as f64,
                        m.c[0][1] as f64,
                        m.c[0][2] as f64,
                        m.c[0][3] as f64,
                    ],
                    [
                        m.c[1][0] as f64,
                        m.c[1][1] as f64,
                        m.c[1][2] as f64,
                        m.c[1][3] as f64,
                    ],
                    [
                        m.c[2][0] as f64,
                        m.c[2][1] as f64,
                        m.c[2][2] as f64,
                        m.c[2][3] as f64,
                    ],
                ])
            }
        }

        // format!("{}", AffineMap3d)
        impl<Unit, Number> fmt::Display for AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<3, 4, Number>(&self.c, Unit::SUFFIX, 1)
                )
            }
        }

        // format!("{:?}", AffineMap3d)
        impl<Unit, Number> fmt::Debug for AffineMap3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<3, 4, Number>(&self.c, Unit::SUFFIX, 1)
                )
            }
        }
    };
}
