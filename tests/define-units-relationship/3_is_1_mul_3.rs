measures::define_measure_types! {
    with_3d exact,
    []
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

measures::define_units_relationship! { true false false, U1 3 == U2 1 * U3 3 }

fn main() {
    let u1: Measure3d<U1> = Measure::<U2>::new(3.) * Measure3d::<U3>::new(6., -2., 7.);
    assert_eq!(u1.x, 18.);
    assert_eq!(u1.y, -6.);
    assert_eq!(u1.z, 21.);
}
