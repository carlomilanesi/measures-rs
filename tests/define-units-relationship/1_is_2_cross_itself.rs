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

measures::define_units_relationship! { true false false, U1 1 == U2 2 X __ 2 }

fn main() {
    use measures::traits::CrossProduct;
    let u1: Measure<U1> = Measure2d::<U2>::new(6., 5.).cross_product(Measure2d::<U2>::new(7., 4.));
    assert_eq!(u1.value, -11.);
}
