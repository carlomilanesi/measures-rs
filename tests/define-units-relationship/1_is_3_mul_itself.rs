measures::define_measure_types! {
    with_3d exact,
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

measures::define_units_relationship! { true false false, U1 1 == U2 3 * __ 3 }

fn main() {
    let u1: Measure<U1> = Measure3d::<U2>::new(6., 4., -5.) * Measure3d::<U2>::new(7., 8., 4.);
    assert_eq!(u1.value, 54.);
}
