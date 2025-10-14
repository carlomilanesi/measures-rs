measures::define_measure_types! {
    with_2d exact,
    vector_properties [
        P1 [ U1 { suffix: " u1" } ]
        P2 [ U2 { suffix: " u2" } ]
    ]
    relationships [ U1 1 == U2 2 * __ 2 ]
}

fn main() {
    let u1: Measure<U1> = Measure2d::<U2>::new([6., 4.]) * Measure2d::<U2>::new([7., 8.]);
    assert_eq!(u1.value, 74.);
}
