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

measures::define_units_relationship! { true false false, U1 3 == U2 3 X __ 3 }

fn main() {
    use measures::traits::CrossProduct;
    let u1: Measure3d<U1> =
        Measure3d::<U2>::new(6., -3., 5.).cross_product(Measure3d::<U2>::new(-2., 7., 8.));
    assert_eq!(u1.x, -59.);
    assert_eq!(u1.y, -58.);
    assert_eq!(u1.z, 36.);
}
