measures::define_measure_types! {
    with_3d exact,
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
        U1 1 == U2 3 * U3 3,
    ]
}

fn main() {
    let u1: Measure<U1> = Measure3d::<U2>::new([6., 4., -5.]) * Measure3d::<U3>::new([7., 8., 4.]);
    assert_eq!(u1.value, 54.);
}
