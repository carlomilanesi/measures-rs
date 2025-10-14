measures::define_measure_types! {
    with_3d exact,
    vector_properties [
        P1 [ U1 { suffix: " u1" } ]
        P2 [ U2 { suffix: " u2" } ]
    ]
    relationships [ U1 3 == U2 3 X __ 3 ]
}

fn main() {
    use measures::traits::CrossProduct;
    let u1: Measure3d<U1> =
        Measure3d::<U2>::new([6., -3., 5.]).cross_product(Measure3d::<U2>::new([-2., 7., 8.]));
    assert_eq!(u1.values, [-59., -58., 36.]);
}
