use trybuild::TestCases;

fn compile_fail(tester: &TestCases, case: &str) {
    tester.compile_fail(format!("tests/disallowed_operations/{}.rs", case));
}

#[test]
fn tests() {
    let t = TestCases::new();
    compile_fail(&t, "assignment_with_different_number_type");
    compile_fail(&t, "assignment_with_different_unit_type");
    compile_fail(&t, "measure_with_number_type_i32");
    compile_fail(&t, "sum_with_different_number_type");
    compile_fail(&t, "sum_with_different_unit_type");
    compile_fail(&t, "increment_with_different_number_type");
    compile_fail(&t, "increment_with_different_unit_type");
    compile_fail(&t, "difference_with_different_number_type");
    compile_fail(&t, "difference_with_different_unit_type");
    compile_fail(&t, "decrement_with_different_number_type");
    compile_fail(&t, "decrement_with_different_unit_type");

    /*TODO: Add tests for the following disallowed operations:
        - Conversion between types of different property (kilogram and second)
        - Multiplication of a Measure by a factor of different number type (f32 and f64)
        - Multiplication of a factor by a Measure of different number type (f32 and f64)
        - Division by a factor of different number type (f32 and f64)
        - Upscaling by a factor of different number type (f32 and f64)
        - Downscaling by a factor of different number type (f32 and f64)
        - Multiplication of a Measure by a Measure<One> of different number type (f32 and f64)
        - Multiplication of a Measure<One> by a Measure of different number type (f32 and f64)
        - Division by a Measure<One> of different number type (f32 and f64)
        - Upscaling by a Measure<One> of different number type (f32 and f64)
        - Downscaling by a Measure<One> of different number type (f32 and f64)
        - Multiplication between incompatible types (`Measure<Second> * Measure<KiloGram>`)
        - Division between incompatible types (`Measure<Second> / Measure<KiloGram>`)
        - Sum between MeasurePoints
        - Increment a MeasurePoint by a MeasurePoint
        - Decrement a MeasurePoint by a MeasurePoint
        - Negating a MeasurePoint
        - Multiplication between MeasurePoint and number
        - Multiplication between MeasurePoint and MeasurePoint<One>
        - Multiplication between number and MeasurePoint
        - Multiplication between MeasurePoint<One> and MeasurePoint
        - Conversion between Measures of different properties
        - Conversion between MeasurePoints of different properties

        - The same tests as above, but for 2D and 3D vector units.
    */
}
