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
        P3 [
            U3 {
                suffix: " u3",
            }
        ]
    ]
    dimensionless_measurement_units [ ]
    angle_measurement_units [ ]
    relationships [
        U1 2 == U2 1 * U3 2,
    ]
}

fn main() {
    let u1: Measure2d<U1> = Measure::<U2>::new(6.) * Measure2d::<U3>::new([7., 8.]);
    assert_eq!(u1.values, [42., 48.]);
}
