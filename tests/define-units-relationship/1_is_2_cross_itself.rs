measures::define_measure_types! {
    with_2d exact,
    scalar_properties [ ]
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
    ]
    dimensionless_measurement_units [ ]
    angle_measurement_units [ ]
    relationships [
        U1 1 == U2 2 X __ 2,
    ]
}

fn main() {
    use measures::traits::CrossProduct;
    let u1: Measure<U1> =
        Measure2d::<U2>::new([6., 5.]).cross_product(Measure2d::<U2>::new([7., 4.]));
    assert_eq!(u1.value, -11.);
}
