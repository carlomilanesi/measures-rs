measures::define_measure_types! {
    exact,
    scalar_properties               [ ]
    vector_properties               [ ]
    dimensionless_measurement_units [ ]
    angle_measurement_units         [ ]
    relationships [
        SquareMetre 1 == Metre 1 * __ 1,
        One 1 == Siemens 1 * Ohm 1,
    ]
}
/*
1
    measures::expand_1_1_same! {$exact $with_approx, $unit2 $unit1}
    measures::expand_1_1! {$exact $with_approx, $unit2 $unit3 $unit1}
2
    measures::expand_1_2! {$exact $with_approx, $unit2 $unit3 $unit1}
    measures::expand_2_2_same! {$exact $with_approx, $unit2 $unit1}
    measures::expand_2_2! {$exact $with_approx, $unit2 $unit3 $unit1}
    measures::expand_cross_2_same! {$exact $with_approx, $unit2 $unit1}
    measures::expand_cross_2! {$exact $with_approx, $unit2 $unit3 $unit1}
3
    measures::expand_1_3! {$exact $with_approx, $unit2 $unit3 $unit1}
    measures::expand_3_3_same! {$exact $with_approx, $unit2 $unit1}
    measures::expand_3_3! {$exact $with_approx, $unit2 $unit3 $unit1}
    measures::expand_cross_3_same! {$exact $with_approx, $unit2 $unit1}
    measures::expand_cross_3! {$exact $with_approx, $unit2 $unit3 $unit1}
*/

measures::measurement_vector_property! { Length }

measures::measurement_unit! {
    name: Metre,
    property: Length,
    suffix: " m",
}

measures::measurement_scalar_property! { Area }

measures::measurement_unit! {
    name: SquareMetre,
    property: Area,
    suffix: " m\u{b2}",
}

measures::measurement_scalar_property! { ElectricalConductance }

measures::measurement_unit! {
    name: Siemens,
    property: ElectricalConductance,
    suffix: " S",
}

measures::measurement_scalar_property! { ElectricalResistance }

measures::measurement_unit! {
    name: Ohm,
    property: ElectricalResistance,
    suffix: " \u{3a9}",
}

#[test]
fn test_relationship_1_1() {
    let siemens = Measure::<Siemens>::new(5.);
    let ohm = Measure::<Ohm>::new(7.);
    let one: Measure<One> = siemens * ohm;
    assert_eq!(one.value, 35.);
    let one: Measure<One> = ohm * siemens;
    assert_eq!(one.value, 35.);
    let ohm: Measure<Ohm> = one / siemens;
    assert_eq!(ohm.value, 7.);
    let siemens: Measure<Siemens> = one / ohm;
    assert_eq!(siemens.value, 5.);
}

#[test]
fn test_relationship_1_1_same() {
    let metre = Measure::<Metre>::new(5.);
    let square_metre: Measure<SquareMetre> = metre * metre;
    assert_eq!(square_metre.value, 25.);
    let metre: Measure<Metre> = square_metre / metre;
    assert_eq!(metre.value, 5.);
    let metre: Measure<Metre> = square_metre.sqrt();
    assert_eq!(metre.value, 5.);
    let square_metre: Measure<SquareMetre> = metre.squared();
    assert_eq!(square_metre.value, 25.);
}

/*
use measures::define_units_relationship;

#[test]
fn test_u1_mul_u1_equals_u3() {
    //expand_1_1 ==
    // Measure<U1> * Measure<U1> -> Measure<U2>
    // Measure<U2> / Measure<U1> -> Measure<U1>
    // Measure<U1>.squared() -> Measure<U2>
    // Measure<U2>.sqrt() -> Measure<U1>

    define_units_relation! { Metre * Metre == SquareMetre }

    let distance1 = Measure::<Metre>::new(2.);
    assert_eq!(distance1.to_string(), "2 m");
    let distance2 = Measure::<Metre>::new(3.);
    assert_eq!(distance2.to_string(), "3 m");
    let area1: Measure<SquareMetre> = distance1 * distance2;
    assert_eq!(area1.to_string(), "6 m\u{b2}");
    let distance3: Measure<Metre> = area1 / distance1;
    assert_eq!(distance3.to_string(), "3 m");
    let area2: Measure<SquareMetre> = distance1.squared();
    assert_eq!(area2.to_string(), "4 m\u{b2}");
    let distance4: Measure<Metre> = area2.sqrt();
    assert_eq!(distance4.to_string(), "2 m");
}

#[test]
fn test_u1_mul_u2_equals_u3() {
    //expand_1_1 !=
    // Measure<U1> * Measure<U2> -> Measure<U3>
    // Measure<U2> * Measure<U1> -> Measure<U3>
    // Measure<U3> / Measure<U1> -> Measure<U2>
    // Measure<U3> / Measure<U2> -> Measure<U1>

    define_units_relation! { MetrePerSecond * Second == Metre }

    let velocity1 = Measure::<MetrePerSecond>::new(2.);
    assert_eq!(velocity1.to_string(), "2 m/s");
    let interval1 = Measure::<Second>::new(5.);
    assert_eq!(interval1.to_string(), "5 s");
    let distance1: Measure<Metre> = velocity1 * interval1;
    assert_eq!(distance1.to_string(), "10 m");
    let distance2: Measure<Metre> = interval1 * velocity1;
    assert_eq!(distance2.to_string(), "10 m");
    let interval2: Measure<Second> = distance1 / velocity1;
    assert_eq!(interval2.to_string(), "5 s");
    let interval3: Measure<Second> = distance1 / velocity1;
    assert_eq!(interval3.to_string(), "5 s");
}
*/

/*
//expand_1_1 ==
    // Measure<U1> * Measure<U1> -> Measure<U2>
    // Measure<U2> / Measure<U1> -> Measure<U1>
    // Measure<U1>.squared() -> Measure<U2>
    // Measure<U2>.sqrt() -> Measure<U1>
//!=
    // Measure<U1> * Measure<U2> -> Measure<U3>
    // Measure<U2> * Measure<U1> -> Measure<U3>
    // Measure<U3> / Measure<U1> -> Measure<U2>
    // Measure<U3> / Measure<U2> -> Measure<U1>

//expand_1_2 ==
    // Measure2d<U1> * Measure2d<U1> -> Measure<U2>
    // Measure2d<U2>.sqr() -> Measure<U1>
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
    // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
    // Measure2d<U3> / Measure<U1> -> Measure2d<U2>

//expand_1_3 ==
    // Measure3d<U1> * Measure3d<U1> -> Measure<U2>
    // Measure3d<U1>.sqr() -> Measure<U2>
    // Measure3d<U1>.cross_product(Measure2d<U1>) -> Measure3d<U3>
//!=
    // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
    // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
    // Measure3d<U3> / Measure<U1> -> Measure3d<U2>

// expand_2_2 ==
    // Measure2d<U1> * Measure2d<U1> -> Measure<U2>
    // Measure2d<U2>.sqr() -> Measure<U1>
//!=
    // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
    // Measure2d<U2> * Measure2d<U1> -> Measure<U3>

// expand_3_3 ==
    // Measure3d<U1> * Measure3d<U1> -> Measure<U2>
    // Measure3d<U1>.sqr() -> Measure<U2>
//!=
    // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
    // Measure3d<U2> * Measure3d<U1> -> Measure<U3>

// expand_cross_2 ==
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U3>
    // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U3>

// expand_cross_3 ==
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
    // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
*/
