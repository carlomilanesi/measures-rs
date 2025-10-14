measures::define_measure_types! {
    exact,
    vector_properties [
        P1 [ U1 { suffix: " u1" } ]
        P2 [ U2 { suffix: " u2" } ]
    ]
    relationships [ U1 1 == U2 1 * __ 1 ]
}

fn main() {
    let u1: Measure<U1> = Measure::<U2>::new(6.) * Measure::<U2>::new(4.);
    assert_eq!(u1.value, 24.);
}
