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
        U1 2 == U2 2 * U3 1,
    ]
}

fn main() {
    let u1: Measure2d<U1> = Measure2d::<U2>::new([7., 9.]) * Measure::<U3>::new(3.);
    assert_eq!(u1.values, [21., 27.]);
}
