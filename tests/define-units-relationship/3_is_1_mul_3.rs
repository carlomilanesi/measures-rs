measures::define_measure_types! {
    with_3d exact,
    vector_properties [
        P1 [
            U1 {
                suffix: " u1",
            }
        ]
        P2 [
            U2 {
                suffix: " u2",
            }
        ]
        P3 [
            U3 {
                suffix: " u3",
            }
        ]
    ]
    relationships [
        U1 3 == U2 1 * U3 3,
    ]
}

fn main() {
    let u1: Measure3d<U1> = Measure::<U2>::new(3.) * Measure3d::<U3>::new([6., -2., 7.]);
    assert_eq!(u1.values, [18., -6., 21.]);
}
