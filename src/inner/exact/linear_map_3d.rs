#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_linear_map_3d {
    {} => {
        /// Linear transformation of `Measure3d` objects.
        pub struct LinearMap3d<Number = f64>
        where
            Number: ArithmeticOps,
        {
            c: [[Number; 3]; 3],
        }

        impl<Number> LinearMap3d<Number>
        where
            Number: ArithmeticOps,
        {
            /// Create a LinearMap3d from its 9 coefficients.
            pub const fn new(coefficients: [[Number; 3]; 3]) -> Self {
                Self { c: coefficients }
            }

            // Linear maps have no translations.

            // Rotations

            // Rotation by an angle measure around a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn rotation<AngleUnit, AxisUnit>(
                angle: Measure<AngleUnit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self
            where
                AngleUnit: AngleMeasurementUnit,
                AxisUnit: MeasurementUnit,
                AxisUnit::Property: VectorProperty,
            {
                Self::rotation_by_radians_around_unit_vector(
                    angle.convert::<Radian>().value,
                    unit_vector.values[0],
                    unit_vector.values[1],
                    unit_vector.values[2],
                )
            }

            // Projections

            // Projection onto a line identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_line<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                Self {
                    c: [
                        [
                            unit_vector.values[0] * unit_vector.values[0],
                            unit_vector.values[1] * unit_vector.values[0],
                            unit_vector.values[2] * unit_vector.values[0],
                        ],
                        [
                            unit_vector.values[0] * unit_vector.values[1],
                            unit_vector.values[1] * unit_vector.values[1],
                            unit_vector.values[2] * unit_vector.values[1],
                        ],
                        [
                            unit_vector.values[0] * unit_vector.values[2],
                            unit_vector.values[1] * unit_vector.values[2],
                            unit_vector.values[2] * unit_vector.values[2],
                        ],
                    ],
                }
            }

            // Projection onto a plane whose normal is identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_plane<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                Self {
                    c: [
                        [
                            Number::ONE - unit_vector.values[0] * unit_vector.values[0],
                            -unit_vector.values[1] * unit_vector.values[0],
                            -unit_vector.values[2] * unit_vector.values[0],
                        ],
                        [
                            -unit_vector.values[0] * unit_vector.values[1],
                            Number::ONE - unit_vector.values[1] * unit_vector.values[1],
                            -unit_vector.values[2] * unit_vector.values[1],
                        ],
                        [
                            -unit_vector.values[0] * unit_vector.values[2],
                            -unit_vector.values[1] * unit_vector.values[2],
                            Number::ONE - unit_vector.values[2] * unit_vector.values[2],
                        ],
                    ],
                }
            }

            // Reflections

            // Reflection over a line identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_line<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                let two = Number::ONE + Number::ONE;
                Self {
                    c: [
                        [
                            two * unit_vector.values[0] * unit_vector.values[0] - Number::ONE,
                            two * unit_vector.values[1] * unit_vector.values[0],
                            two * unit_vector.values[2] * unit_vector.values[0],
                        ],
                        [
                            two * unit_vector.values[0] * unit_vector.values[1],
                            two * unit_vector.values[1] * unit_vector.values[1] - Number::ONE,
                            two * unit_vector.values[2] * unit_vector.values[1],
                        ],
                        [
                            two * unit_vector.values[0] * unit_vector.values[2],
                            two * unit_vector.values[1] * unit_vector.values[2],
                            two * unit_vector.values[2] * unit_vector.values[2] - Number::ONE,
                        ],
                    ],
                }
            }

            // Reflection over a plane whose normal is identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_plane<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                let minus_two = -(Number::ONE + Number::ONE);
                Self {
                    c: [
                        [
                            minus_two * unit_vector.values[0] * unit_vector.values[0] + Number::ONE,
                            minus_two * unit_vector.values[1] * unit_vector.values[0],
                            minus_two * unit_vector.values[2] * unit_vector.values[0],
                        ],
                        [
                            minus_two * unit_vector.values[0] * unit_vector.values[1],
                            minus_two * unit_vector.values[1] * unit_vector.values[1] + Number::ONE,
                            minus_two * unit_vector.values[2] * unit_vector.values[1],
                        ],
                        [
                            minus_two * unit_vector.values[0] * unit_vector.values[2],
                            minus_two * unit_vector.values[1] * unit_vector.values[2],
                            minus_two * unit_vector.values[2] * unit_vector.values[2] + Number::ONE,
                        ],
                    ],
                }
            }

            // Scaling by three factors.

            pub fn scaling(factors: [Number; 3]) -> Self {
                Self {
                    c: [
                        [factors[0], Number::ZERO, Number::ZERO],
                        [Number::ZERO, factors[1], Number::ZERO],
                        [Number::ZERO, Number::ZERO, factors[2]],
                    ],
                }
            }

            // Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE
                    / (self.c[0][0] * (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1])
                        - self.c[0][1] * (self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0])
                        + self.c[0][2] * (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]));
                Self {
                    c: [
                        [
                            (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1]) * inv_determinant,
                            -(self.c[0][1] * self.c[2][2] - self.c[0][2] * self.c[2][1]) * inv_determinant,
                            (self.c[0][1] * self.c[1][2] - self.c[0][2] * self.c[1][1]) * inv_determinant,
                        ],
                        [
                            -(self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0]) * inv_determinant,
                            (self.c[0][0] * self.c[2][2] - self.c[0][2] * self.c[2][0]) * inv_determinant,
                            -(self.c[0][0] * self.c[1][2] - self.c[0][2] * self.c[1][0]) * inv_determinant,
                        ],
                        [
                            (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]) * inv_determinant,
                            -(self.c[0][0] * self.c[2][1] - self.c[0][1] * self.c[2][0]) * inv_determinant,
                            (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]) * inv_determinant,
                        ],
                    ],
                }
            }

            // Composition of spacial linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &LinearMap3d<Number>) -> Self {
                Self {
                    c: [
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
                        ],
                    ],
                }
            }

            pub fn apply_to<Unit: MeasurementUnit>(
                &self,
                m: Measure3d<Unit, Number>,
            ) -> Measure3d<Unit, Number>
            where
                Unit::Property: VectorProperty,
            {
                Measure3d::<Unit, Number>::new([
                    self.c[0][0] * m.values[0] + self.c[0][1] * m.values[1] + self.c[0][2] * m.values[2],
                    self.c[1][0] * m.values[0] + self.c[1][1] * m.values[1] + self.c[1][2] * m.values[2],
                    self.c[2][0] * m.values[0] + self.c[2][1] * m.values[1] + self.c[2][2] * m.values[2],
                ])
            }

            fn rotation_by_radians_around_unit_vector(
                a: Number,
                ux: Number,
                uy: Number,
                uz: Number,
            ) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                let one_minus_cos_a = Number::ONE - cos_a;
                Self {
                    c: [
                        [
                            cos_a + ux * ux * one_minus_cos_a,
                            ux * uy * one_minus_cos_a - uz * sin_a,
                            ux * uz * one_minus_cos_a + uy * sin_a,
                        ],
                        [
                            uy * ux * one_minus_cos_a + uz * sin_a,
                            cos_a + uy * uy * one_minus_cos_a,
                            uy * uz * one_minus_cos_a - ux * sin_a,
                        ],
                        [
                            uz * ux * one_minus_cos_a - uy * sin_a,
                            uz * uy * one_minus_cos_a + ux * sin_a,
                            cos_a + uz * uz * one_minus_cos_a,
                        ],
                    ],
                }
            }
        }

        impl<Number> Default for LinearMap3d<Number>
        where
            Number: ArithmeticOps,
        {
            // It returns the identity transformation.
            fn default() -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, Number::ZERO],
                    [Number::ZERO, Number::ONE, Number::ZERO],
                    [Number::ZERO, Number::ZERO, Number::ONE],
                ])
            }
        }

        // LinearMap3d == LinearMap3d -> bool
        impl<Number> PartialEq<LinearMap3d<Number>> for LinearMap3d<Number>
        where
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &LinearMap3d<Number>) -> bool {
                self.c == other.c
            }
        }

        // LinearMap3d.clone() -> LinearMap3d
        impl<Number> Clone for LinearMap3d<Number>
        where
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                Self { c: self.c.clone() }
            }
        }

        impl From<LinearMap3d<f32>> for LinearMap3d<f64> {
            fn from(m: LinearMap3d<f32>) -> Self {
                Self::new([
                    [m.c[0][0] as f64, m.c[0][1] as f64, m.c[0][2] as f64],
                    [m.c[1][0] as f64, m.c[1][1] as f64, m.c[1][2] as f64],
                    [m.c[2][0] as f64, m.c[2][1] as f64, m.c[2][2] as f64],
                ])
            }
        }

        // format!("{}", LinearMap3d)
        impl<Number: ArithmeticOps> fmt::Display for LinearMap3d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<3, 3, Number>(&self.c, "", 1)
                )
            }
        }

        // format!("{:?}", LinearMap3d)
        impl<Number: ArithmeticOps> fmt::Debug for LinearMap3d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<3, 3, Number>(&self.c, "", 1)
                )
            }
        }
    };
}
