#[macro_export] // Don't add nor remove the first three lines and the last two lines.
macro_rules! inner_define_linear_map_2d {
    {} => {
        /// Linear transformation of `Measure2d` objects.
        pub struct LinearMap2d<Number = f64>
        where
            Number: ArithmeticOps,
        {
            c: [[Number; 2]; 2],
        }

        impl<Number: ArithmeticOps> LinearMap2d<Number> {
            /// Create a LinearMap2d from its 4 coefficients.
            pub const fn new(coefficients: [[Number; 2]; 2]) -> Self {
                Self { c: coefficients }
            }

            // Linear maps have no translations.

            //// Rotations

            /// Create a rotation by an angle measure.
            pub fn rotation<AngleUnit>(angle: Measure<AngleUnit, Number>) -> Self
            where
                AngleUnit: AngleMeasurementUnit,
            {
                Self::rotation_by_radians(angle.convert::<Radian>().value)
            }

            /// Create a rotation to the right.
            pub fn right_rotation() -> Self {
                Self {
                    c: [[Number::ZERO, Number::ONE], [-Number::ONE, Number::ZERO]],
                }
            }

            /// Create a rotation to the left.
            pub fn left_rotation() -> Self {
                Self {
                    c: [[Number::ZERO, -Number::ONE], [Number::ONE, Number::ZERO]],
                }
            }

            //// Projections

            // Projection onto a line identified by a measure point angle.
            pub fn projection_by_angle<Unit: AngleMeasurementUnit>(
                angle: impl Into<MeasurePoint<Unit, Number>>,
            ) -> Self {
                Self::projection_by_radians(angle.into().convert::<Radian>().value)
            }

            // Projection onto a line identified by a unit plane vector.
            // Precondition: unit_v.squared_norm().value == 1
            pub fn projection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::projection_by_cos_sin(v.values[0], v.values[1])
            }

            //// Reflections

            // Reflection over a line identified by a point angle.
            pub fn reflection_by_angle<AngleUnit: AngleMeasurementUnit>(
                angle: impl Into<MeasurePoint<AngleUnit, Number>>,
            ) -> Self {
                Self::reflection_by_radians(angle.into().convert::<Radian>().value)
            }

            // Reflection over a line identified by a unit plane vector.
            // Precondition: v.squared_norm() == 1
            pub fn reflection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::reflection_by_cos_sin(v.values[0], v.values[1])
            }

            //// Scaling by two factors.

            pub fn scaling(factors: [Number; 2]) -> Self {
                Self {
                    c: [[factors[0], Number::ZERO], [Number::ZERO, factors[1]]],
                }
            }

            //// Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant =
                    Number::ONE / (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]);
                Self {
                    c: [
                        [
                            self.c[1][1] * inv_determinant,
                            self.c[0][1] * -inv_determinant,
                        ],
                        [
                            self.c[1][0] * -inv_determinant,
                            self.c[0][0] * inv_determinant,
                        ],
                    ],
                }
            }

            // Composition of two plane linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &LinearMap2d<Number>) -> Self {
                Self {
                    c: [
                        [
                            other.c[0][0] * self.c[0][0] + other.c[0][1] * self.c[1][0],
                            other.c[0][0] * self.c[0][1] + other.c[0][1] * self.c[1][1],
                        ],
                        [
                            other.c[1][0] * self.c[0][0] + other.c[1][1] * self.c[1][0],
                            other.c[1][0] * self.c[0][1] + other.c[1][1] * self.c[1][1],
                        ],
                    ],
                }
            }

            pub fn apply_to<Unit: MeasurementUnit>(
                &self,
                m: Measure2d<Unit, Number>,
            ) -> Measure2d<Unit, Number>
            where
                Unit::Property: VectorProperty,
            {
                Measure2d::<Unit, Number>::new([
                    self.c[0][0] * m.values[0] + self.c[0][1] * m.values[1],
                    self.c[1][0] * m.values[0] + self.c[1][1] * m.values[1],
                ])
            }

            fn rotation_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self {
                    c: [[cos_a, -sin_a], [sin_a, cos_a]],
                }
            }

            fn projection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                Self {
                    c: [
                        [cos_a * cos_a, cos_a * sin_a],
                        [sin_a * cos_a, sin_a * sin_a],
                    ],
                }
            }

            fn projection_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self::projection_by_cos_sin(cos_a, sin_a)
            }

            fn reflection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                let one = Number::ONE;
                let two = Number::ONE + Number::ONE;
                Self {
                    c: [
                        [two * cos_a * cos_a - one, two * cos_a * sin_a],
                        [two * cos_a * sin_a, two * sin_a * sin_a - one],
                    ],
                }
            }

            fn reflection_by_radians(radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::reflection_by_cos_sin(cos_a, sin_a)
            }
        }

        impl<Number> Default for LinearMap2d<Number>
        where
            Number: ArithmeticOps,
        {
            // It returns the identity transformation.
            fn default() -> Self {
                Self::new([[Number::ONE, Number::ZERO], [Number::ZERO, Number::ONE]])
            }
        }

        // LinearMap2d == LinearMap2d -> bool
        impl<Number> PartialEq<LinearMap2d<Number>> for LinearMap2d<Number>
        where
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &LinearMap2d<Number>) -> bool {
                self.c == other.c
            }
        }

        // LinearMap2d.clone() -> LinearMap2d
        impl<Number> Clone for LinearMap2d<Number>
        where
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                Self { c: self.c.clone() }
            }
        }

        impl From<LinearMap2d<f32>> for LinearMap2d<f64> {
            fn from(m: LinearMap2d<f32>) -> Self {
                Self::new([
                    [m.c[0][0] as f64, m.c[0][1] as f64],
                    [m.c[1][0] as f64, m.c[1][1] as f64],
                ])
            }
        }

        // format!("{}", LinearMap2d)
        impl<Number: ArithmeticOps> fmt::Display for LinearMap2d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<2, 2, Number>(&self.c, "", 1)
                )
            }
        }

        // format!("{:?}", LinearMap2d)
        impl<Number: ArithmeticOps> fmt::Debug for LinearMap2d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    measures::matrix_utils::format_matrix::<2, 2, Number>(&self.c, "", 1)
                )
            }
        }
    };
}
