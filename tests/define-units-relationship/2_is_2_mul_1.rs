measures::define_measure_types! {
    with_2d exact,
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

measures::define_units_relationship! { true false false, U1 2 == U2 2 * U3 1 }

fn main() {
    let u1: Measure2d<U1> = Measure2d::<U2>::new(7., 9.) * Measure::<U3>::new(3.);
    assert_eq!(u1.x, 21.);
    assert_eq!(u1.y, 27.);
}
