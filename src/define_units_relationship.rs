#[macro_export]
macro_rules! define_units_relationship {
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 1 * __ 1 } => {
        measures::expand_1_1_same! {$exact $with_approx $with_correlation, $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 1 * $unit3:ident 1 } => {
        measures::expand_1_1! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 2 == $unit2:ident 1 * $unit3:ident 2 } => {
        measures::expand_1_2! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 2 == $unit2:ident 2 * $unit3:ident 1 } => {
        measures::expand_1_2! {$exact $with_approx $with_correlation, $unit3 $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 3 == $unit2:ident 1 * $unit3:ident 3 } => {
        measures::expand_1_3! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 3 == $unit2:ident 3 * $unit3:ident 1 } => {
        measures::expand_1_3! {$exact $with_approx $with_correlation, $unit3 $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 2 * __ 2} => {
        measures::expand_2_2_same! {$exact $with_approx $with_correlation, $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 2 * $unit3:ident 2 } => {
        measures::expand_2_2! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 3 * __ 3} => {
        measures::expand_3_3_same! {$exact $with_approx $with_correlation, $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 3 * $unit3:ident 3 } => {
        measures::expand_3_3! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 2 X __ 2 } => {
        measures::expand_cross_2_same! {$exact $with_approx $with_correlation, $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 1 == $unit2:ident 2 X $unit3:ident 2 } => {
        measures::expand_cross_2! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 3 == $unit2:ident 3 X __ 3 } => {
        measures::expand_cross_3_same! {$exact $with_approx $with_correlation, $unit2 $unit1}
    };
    { $exact:tt $with_approx:tt $with_correlation:tt, $unit1:ident 3 == $unit2:ident 3 X $unit3:ident 3 } => {
        measures::expand_cross_3! {$exact $with_approx $with_correlation, $unit2 $unit3 $unit1}
    };
}

// Generates the operator overloads to multiply and divide two 1-D measures having different units.
#[macro_export]
macro_rules! expand_1_1 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Operations for exact measures.
        measures::if_all_true! { {$exact}
            // Measure<U1> * Measure<U2> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<$unit2, Number>> for Measure<$unit1, Number> {
                type Output = Measure<$unit3, Number>;
                fn mul(self, other: Measure<$unit2, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U2> * Measure<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure<$unit2, Number> {
                type Output = Measure<$unit3, Number>;
                fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U3> / Measure<U1> -> Measure<U2>
            impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure<$unit3, Number> {
                type Output = Measure<$unit2, Number>;
                fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }

            // Measure<U3> / Measure<U2> -> Measure<U1>
            impl<Number: ArithmeticOps> Div<Measure<$unit2, Number>> for Measure<$unit3, Number> {
                type Output = Measure<$unit1, Number>;
                fn div(self, other: Measure<$unit2, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }
        }

        // Operations for measures with a variance.
        measures::if_all_true! { {$with_approx}
            // ApproxMeasure<U1> * ApproxMeasure<U2> -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure<$unit2, Number>> for ApproxMeasure<$unit1, Number> {
                type Output = ApproxMeasure<$unit3, Number>;
                fn mul(self, other: ApproxMeasure<$unit2, Number>) -> Self::Output {
                    Self::Output::with_variance(
                        self.value * other.value,
                        other.value * other.value * self.variance + self.value * self.value * other.variance
                    )
                }
            }

            // ApproxMeasure<U2> * ApproxMeasure<U1> -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure<$unit1, Number>> for ApproxMeasure<$unit2, Number> {
                type Output = ApproxMeasure<$unit3, Number>;
                fn mul(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    let value_product = self.value * other.value;
                    Self::Output::with_variance(
                        value_product,
                        value_product *
                            (other.value * self.variance / self.value +
                            self.value * other.variance / other.value),
                    )
                }
            }

            // ApproxMeasure<U3> / ApproxMeasure<U1> -> ApproxMeasure<U2>
            impl<Number: ArithmeticOps> Div<ApproxMeasure<$unit1, Number>> for ApproxMeasure<$unit3, Number> {
                type Output = ApproxMeasure<$unit2, Number>;
                fn div(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    let self_ratio = self.variance / (self.value * self.value);
                    let other_ratio = other.variance / (other.value * other.value);
                    let value_ratio = self.value / other.value;
                    Self::Output::with_variance(
                        value_ratio,
                        value_ratio * value_ratio * (self_ratio + other_ratio),
                    )
                }
            }

            // ApproxMeasure<U3> / ApproxMeasure<U2> -> ApproxMeasure<U1>
            impl<Number: ArithmeticOps> Div<ApproxMeasure<$unit2, Number>> for ApproxMeasure<$unit3, Number> {
                type Output = ApproxMeasure<$unit1, Number>;
                fn div(self, other: ApproxMeasure<$unit2, Number>) -> Self::Output {
                    let self_ratio = self.variance / (self.value * self.value);
                    let other_ratio = other.variance / (other.value * other.value);
                    let value_ratio = self.value / other.value;
                    Self::Output::with_variance(
                        value_ratio,
                        value_ratio * value_ratio * (self_ratio + other_ratio),
                    )
                }
            }
        }

        // Operations with correlation, for measures with a variance.
        measures::if_all_true! { {$with_correlation}
            // ApproxMeasure<U1>.multiply_with_correlation(ApproxMeasure<U2>, Number) -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> ApproxMeasure<$unit1, Number> {
                fn multiply_with_correlation(self, other: ApproxMeasure<$unit2, Number>, correlation: Number) -> ApproxMeasure<$unit3, Number> {
                    ApproxMeasure::<$unit3, Number>::with_variance(
                        self.value * other.value,
                        other.value * other.value * self.variance + self.value * self.value * other.variance + (Number::ONE + Number::ONE) * self.value * other.value * correlation * self.variance.sqrt() * other.variance.sqrt(),
                    )
                }
            }

            // ApproxMeasure<U2>.multiply_with_correlation(ApproxMeasure<U1>, Number) -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> ApproxMeasure<$unit1, Number> {
                fn multiply_with_correlation(self, other: ApproxMeasure<$unit2, Number>, correlation: Number) -> ApproxMeasure<$unit3, Number> {
                    other.multiply_with_correlation(self, correlation)
                }
            }

            // ApproxMeasure<U3>.divide_with_correlation(ApproxMeasure<U1>, Number) -> ApproxMeasure<U2>
            impl<Number: ArithmeticOps> ApproxMeasure<$unit1, Number> {
                fn divide_with_correlation(self, other: ApproxMeasure<$unit2, Number>, correlation: Number) -> ApproxMeasure<$unit3, Number> {
                    //TODO: Verify this formula.
                    let value_product = self.value * other.value;
                    ApproxMeasure::<$unit3, Number>::with_variance(
                        value_product,
                        other.value * other.value * self.variance + self.value * self.value * other.variance + (Number::ONE + Number::ONE) * self.value * other.value * correlation * self.variance.sqrt() * other.variance.sqrt(),
                    )
                }
            }

            // ApproxMeasure<U3>.divide_with_correlation(ApproxMeasure<U2>, Number) -> ApproxMeasure<U1>
            impl<Number: ArithmeticOps> ApproxMeasure<$unit1, Number> {
                fn divide_with_correlation(self, other: ApproxMeasure<$unit2, Number>, correlation: Number) -> ApproxMeasure<$unit3, Number> {
                    //TODO: Verify this formula.
                    let value_product = self.value * other.value;
                    ApproxMeasure::<$unit3, Number>::with_variance(
                        value_product,
                        other.value * other.value * self.variance + self.value * self.value * other.variance + (Number::ONE + Number::ONE) * self.value * other.value * correlation * self.variance.sqrt() * other.variance.sqrt(),
                    )
                }
            }
        }
    };
}

// Generates the operator overloads to multiply and divide two 1-D measures having the same unit.
#[macro_export]
macro_rules! expand_1_1_same {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit3:ident
    } => {
        measures::if_all_true! { {$exact}
            // Measure<U1> * Measure<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure<$unit1, Number> {
                type Output = Measure<$unit3, Number>;
                fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U3> / Measure<U1> -> Measure<U1>
            impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure<$unit3, Number> {
                type Output = Measure<$unit1, Number>;
                fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }

            // Measure<U1>.squared() -> Measure<U3>
            impl<Number: ArithmeticOps> Measure<$unit1, Number> {
                fn squared(self) -> Measure<$unit3, Number> {
                    Measure::<$unit3, Number>::new(self.value * self.value)
                }
            }

            // Measure<U3>.sqrt() -> Measure<U1>
            // This generic implementation causes performance issues is some cases,
            // and so it is replaced by the following implementations, specific,
            // respectively, for f64 and f32.
            // impl<Number: ArithmeticOps> Sqrt for Measure<$unit3, Number> {
            //     type Output = Measure<$unit1, Number>;
            //     fn sqrt(self) -> Self::Output {
            //         Self::Output::new(self.value.sqrt())
            //     }
            // }

            impl Sqrt for Measure<$unit3, f64> {
                type Output = Measure<$unit1, f64>;
                fn sqrt(self) -> Self::Output {
                    Self::Output::new(self.value.sqrt())
                }
            }

            impl Sqrt for Measure<$unit3, f32> {
                type Output = Measure<$unit1, f32>;
                fn sqrt(self) -> Self::Output {
                    Self::Output::new(self.value.sqrt())
                }
            }
        }

        measures::if_all_true! { {$with_approx}
            // ApproxMeasure<U1> * ApproxMeasure<U1> -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure<$unit1, Number>> for ApproxMeasure<$unit1, Number> {
                type Output = ApproxMeasure<$unit3, Number>;
                fn mul(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    let value_product = self.value * other.value;
                    Self::Output::with_variance(
                        value_product,
                        value_product *
                            (other.value * self.variance / self.value +
                            self.value * other.variance / other.value),
                    )
                }
            }

            // ApproxMeasure<U3> / ApproxMeasure<U1> -> ApproxMeasure<U1>
            impl<Number: ArithmeticOps> Div<ApproxMeasure<$unit1, Number>> for ApproxMeasure<$unit3, Number> {
                type Output = ApproxMeasure<$unit1, Number>;
                fn div(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    let self_ratio = self.variance / (self.value * self.value);
                    let other_ratio = other.variance / (other.value * other.value);
                    let value_ratio = self.value / other.value;
                    Self::Output::with_variance(
                        value_ratio,
                        value_ratio * value_ratio * (self_ratio + other_ratio),
                    )
                }
            }

            // ApproxMeasure<U1>.squared() -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> ApproxMeasure<$unit1, Number> {
                fn squared(self) -> ApproxMeasure<$unit3, Number> {
                    let value_product = self.value * self.value;
                    ApproxMeasure::<$unit3, Number>::with_variance(
                        value_product,
                        value_product * ((self.variance + self.variance) + (self.variance + self.variance)),
                    )
                }
            }

            // ApproxMeasure<U3>.sqrt() -> ApproxMeasure<U1>
            impl<Number: ArithmeticOps> Sqrt for ApproxMeasure<$unit3, Number> {
                type Output = ApproxMeasure<$unit1, Number>;
                fn sqrt(self) -> Self::Output {
                    Self::Output::with_variance(
                        self.value.sqrt(),
                        self.variance / self.value / ((Number::ONE + Number::ONE) + (Number::ONE + Number::ONE)),
                    )
                }
            }
        }
    };
}

// Generates the operator overloads to multiply and divide a 1-D measure and a 2-D measure.
#[macro_export]
macro_rules! expand_1_2 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Measure2d<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new([self.value * other.values[0], self.value * other.values[1]])
            }
        }

        // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure2d<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new([self.values[0] * other.value, self.values[1] * other.value])
            }
        }

        // Measure2d<U3> / Measure<U1> -> Measure2d<U2>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure2d<$unit3, Number> {
            type Output = Measure2d<$unit2, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new([self.values[0] / other.value, self.values[1] / other.value])
            }
        }
    };
}

// Generates the operator overloads to multiply and divide a 1-D measure and a 3-D measure.
#[macro_export]
macro_rules! expand_1_3 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new([
                    self.value * other.values[0],
                    self.value * other.values[1],
                    self.value * other.values[2],
                ])
            }
        }

        // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                other * self
            }
        }

        // Measure3d<U3> / Measure<U1> -> Measure3d<U2>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure3d<$unit3, Number> {
            type Output = Measure3d<$unit2, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[0] / other.value,
                    self.values[1] / other.value,
                    self.values[2] / other.value,
                ])
            }
        }

        measures::if_all_true! { {$with_approx}
            // ApproxMeasure<U1> * ApproxMeasure3d<U2> -> ApproxMeasure3d<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure3d<$unit2, Number>> for ApproxMeasure<$unit1, Number> {
                type Output = ApproxMeasure3d<$unit3, Number>;
                fn mul(self, other: ApproxMeasure3d<$unit2, Number>) -> Self::Output {
                    let value_product_x = self.value * other.values[0];
                    let value_product_y = self.value * other.values[1];
                    let value_product_z = self.value * other.values[2];
                    Self::Output::with_variance(
                        [
                            value_product_x,
                            value_product_y,
                            value_product_z,
                        ],
                        value_product_x *
                            (other.values[0] * self.variance / self.value +
                            self.value * other.variance / other.values[0]) +
                        value_product_y *
                            (other.values[1] * self.variance / self.value +
                            self.value * other.variance / other.values[1]) +
                        value_product_z *
                            (other.values[2] * self.variance / self.value +
                            self.value * other.variance / other.values[2]),
                    )
                }
            }

            // ApproxMeasure3d<U2> * ApproxMeasure<U1> -> ApproxMeasure3d<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure<$unit1, Number>> for ApproxMeasure3d<$unit2, Number> {
                type Output = ApproxMeasure3d<$unit3, Number>;
                fn mul(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    other * self
                }
            }

            // ApproxMeasure3d<U3> / ApproxMeasure<U1> -> ApproxMeasure3d<U2>
            impl<Number: ArithmeticOps> Div<ApproxMeasure<$unit1, Number>> for ApproxMeasure3d<$unit3, Number> {
                type Output = ApproxMeasure3d<$unit2, Number>;
                fn div(self, other: ApproxMeasure<$unit1, Number>) -> Self::Output {
                    let value_ratio_x = self.values[0] / other.value;
                    let value_ratio_y = self.values[1] / other.value;
                    let value_ratio_z = self.values[2] / other.value;
                    let self_ratio_x = self.variance / (self.values[0] * self.values[0]);
                    let self_ratio_y = self.variance / (self.values[1] * self.values[1]);
                    let self_ratio_z = self.variance / (self.values[2] * self.values[2]);
                    let other_ratio = other.variance / (other.value * other.value);
                    Self::Output::with_variance(
                        [
                            value_ratio_x,
                            value_ratio_y,
                            value_ratio_z,
                        ],
                        value_ratio_x * value_ratio_x * (self_ratio_x + other_ratio) +
                        value_ratio_y * value_ratio_y * (self_ratio_y + other_ratio) +
                        value_ratio_z * value_ratio_z * (self_ratio_z + other_ratio)
                    )
                }
            }
        }
    };
}

// Generates the operator overloads to compute the dot product (`*`) of two 2-D measures having different units.
#[macro_export]
macro_rules! expand_2_2 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit2, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1])
            }
        }

        // Measure2d<U2> * Measure2d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1])
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}

// Generates the operator overloads to compute the dot product (`*`) of two 2-D measures having the same unit.
#[macro_export]
macro_rules! expand_2_2_same {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident
    } => {
        // Measure2d<U1> * Measure2d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit1, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn mul(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1])
            }
        }

        // Measure2d<U1>.squared() -> Measure<U3>
        impl<Number: ArithmeticOps> Measure2d<$unit1, Number> {
            fn squared(self) -> Measure<$unit2, Number> {
                Measure::<$unit2, Number>::new(self.values[0] * self.values[0] + self.values[1] * self.values[1])
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}

// Generates the operator overloads to compute the dot product (`*`) of two 3-D measures having the same unit.
#[macro_export]
macro_rules! expand_3_3_same {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident
    } => {
        // Measure3d<U1> * Measure3d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit1, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn mul(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1] + self.values[2] * other.values[2])
            }
        }

        // Measure3d<U1>.squared() -> Measure<U3>
        impl<Number: ArithmeticOps> Measure3d<$unit1, Number> {
            fn squared(self) -> Measure<$unit2, Number> {
                Measure::<$unit2, Number>::new(self.values[0] * self.values[0] + self.values[1] * self.values[1] + self.values[2] * self.values[2])
            }
        }

        measures::if_all_true! { {$with_approx}
            // ApproxMeasure3d<U1> * ApproxMeasure3d<U1> -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure3d<$unit1, Number>> for ApproxMeasure3d<$unit1, Number> {
                type Output = ApproxMeasure<$unit2, Number>;
                fn mul(self, other: ApproxMeasure3d<$unit1, Number>) -> Self::Output {
                    let value_product_x = self.values[0] * other.values[0];
                    let value_product_y = self.values[1] * other.values[1];
                    let value_product_z = self.values[2] * other.values[2];
                    Self::Output::with_variance(
                        value_product_x +
                        value_product_y +
                        value_product_z,
                        value_product_x *
                            (other.values[0] * self.variance / self.values[0] +
                            self.values[0] * other.variance / other.values[0]) +
                        value_product_y *
                            (other.values[1] * self.variance / self.values[1] +
                            self.values[1] * other.variance / other.values[1]) +
                        value_product_z *
                            (other.values[2] * self.variance / self.values[2] +
                            self.values[2] * other.variance / other.values[2]),
                    )
                }
            }

            // ApproxMeasure3d<U1>.squared() -> ApproxMeasure<U3>
            impl<Number: ArithmeticOps> ApproxMeasure3d<$unit1, Number> {
                fn squared(self) -> ApproxMeasure<$unit2, Number> {
                    let value = self.values[0] * self.values[0] + self.values[1] * self.values[1] + self.values[2] * self.values[2];
                    ApproxMeasure::<$unit2, Number>::with_variance(
                        value,
                        value * (self.variance + self.variance),
                    )
                }
            }
        }
    };
}

// Generates the operator overloads to compute the dot product (`*`) of two 3-D measures having different units.
#[macro_export]
macro_rules! expand_3_3 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit2, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1] + self.values[2] * other.values[2])
            }
        }

        // Measure3d<U2> * Measure3d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[0] + self.values[1] * other.values[1] + self.values[2] * other.values[2])
            }
        }

        measures::if_all_true! { {$with_approx}
            // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure3d<$unit2, Number>> for ApproxMeasure3d<$unit1, Number> {
                type Output = ApproxMeasure<$unit3, Number>;
                fn mul(self, other: ApproxMeasure3d<$unit2, Number>) -> Self::Output {
                    let value_product_x = self.values[0] * other.values[0];
                    let value_product_y = self.values[1] * other.values[1];
                    let value_product_z = self.values[2] * other.values[2];
                    Self::Output::with_variance(
                        value_product_x + value_product_y + value_product_z,
                        value_product_x *
                            (other.values[0] * self.variance / self.values[0] +
                            self.values[0] * other.variance / other.values[0]) +
                        value_product_y *
                            (other.values[1] * self.variance / self.values[1] +
                            self.values[1] * other.variance / other.values[1]) +
                        value_product_z *
                            (other.values[2] * self.variance / self.values[2] +
                            self.values[2] * other.variance / other.values[2]),
                    )
                }
            }

            // Measure3d<U2> * Measure3d<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<ApproxMeasure3d<$unit1, Number>> for ApproxMeasure3d<$unit2, Number> {
                type Output = ApproxMeasure<$unit3, Number>;
                fn mul(self, other: ApproxMeasure3d<$unit1, Number>) -> Self::Output {
                    let value_product_x = self.values[0] * other.values[0];
                    let value_product_y = self.values[1] * other.values[1];
                    let value_product_z = self.values[2] * other.values[2];
                    Self::Output::with_variance(
                        value_product_x + value_product_y + value_product_z,
                        value_product_x *
                            (other.values[0] * self.variance / self.values[0] +
                            self.values[0] * other.variance / other.values[0]) +
                        value_product_y *
                            (other.values[1] * self.variance / self.values[1] +
                            self.values[1] * other.variance / other.values[1]) +
                        value_product_z *
                            (other.values[2] * self.variance / self.values[2] +
                            self.values[2] * other.variance / other.values[2]),
                    )
                }
            }
        }
    };
}

// Generates the function to compute the cross product of two 2-D measures having the same unit.
#[macro_export]
macro_rules! expand_cross_2_same {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident
    } => {
        // Measure2d<U1>.cross_product(Measure2d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure2d<$unit1, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn cross_product(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[1] - self.values[1] * other.values[0])
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}

// Generates the functions to compute the cross product of two 2-D measures having different units.
#[macro_export]
macro_rules! expand_cross_2 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U3>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure2d<$unit2, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn cross_product(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[1] - self.values[1] * other.values[0])
            }
        }

        // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure2d<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn cross_product(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.values[0] * other.values[1] - self.values[1] * other.values[0])
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}

// Generates the function to compute the cross product of two 3-D measures having the same unit.
#[macro_export]
macro_rules! expand_cross_3_same {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident
    } => {
        // Measure3d<U1>.cross_product(Measure3d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure3d<$unit1, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure3d<$unit2, Number>;
            fn cross_product(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(
                    [
                        self.values[1] * other.values[2] - self.values[2] * other.values[1],
                        self.values[2] * other.values[0] - self.values[0] * other.values[2],
                        self.values[0] * other.values[1] - self.values[1] * other.values[0],
                    ]
                )
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}

// Generates the functions to compute the cross product of two 3-D measures having different units.
#[macro_export]
macro_rules! expand_cross_3 {
    {
        $exact:ident $with_approx:ident $with_correlation:ident,
        $unit1:ident $unit2:ident $unit3:ident
    } => {
        // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure3d<$unit2, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn cross_product(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[1] * other.values[2] - self.values[2] * other.values[1],
                    self.values[2] * other.values[0] - self.values[0] * other.values[2],
                    self.values[0] * other.values[1] - self.values[1] * other.values[0],
                ])
            }
        }

        // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
        impl<Number: ArithmeticOps> measures::traits::CrossProduct<Measure3d<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn cross_product(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new([
                    self.values[1] * other.values[2] - self.values[2] * other.values[1],
                    self.values[2] * other.values[0] - self.values[0] * other.values[2],
                    self.values[0] * other.values[1] - self.values[1] * other.values[0],
                ])
            }
        }

        measures::if_all_true! { {$with_approx}
        }
    };
}
