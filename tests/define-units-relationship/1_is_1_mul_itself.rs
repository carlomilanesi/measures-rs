measures::define_measure_types! {
    exact,
    scalar_properties               [ ]
    vector_properties               [ ]
    dimensionless_measurement_units [ ]
    angle_measurement_units         [ ]
    relationships [
    ]
}

measures::measurement_vector_property! { P1 }

measures::measurement_unit! {
    name: U1,
    property: P1,
    suffix: " u1",
}

measures::measurement_vector_property! { P2 }

measures::measurement_unit! {
    name: U2,
    property: P2,
    suffix: " u2",
}

measures::define_units_relationship! { true false false, U1 1 == U2 1 * __ 1 }

fn main() {
    let u1: Measure<U1> = Measure::<U2>::new(6.) * Measure::<U2>::new(4.);
    assert_eq!(u1.value, 24.);
}
