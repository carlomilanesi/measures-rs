measures::define_measure_types! {
    with_2d exact,
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

measures::measurement_vector_property! { P3 }

measures::measurement_unit! {
    name: U3,
    property: P3,
    suffix: " u3",
}

measures::define_units_relationship! { true false false, U1 2 == U2 1 * U3 2 }

fn main() {
    let u1: Measure2d<U1> = Measure::<U2>::new(6.) * Measure2d::<U3>::new(7., 8.);
    assert_eq!(u1.x, 42.);
    assert_eq!(u1.y, 48.);
}
