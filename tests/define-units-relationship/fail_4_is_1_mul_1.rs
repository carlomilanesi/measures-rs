measures::define_measure_types! {
    exact,
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
        U1 4 == U2 1 * U3 1,
    ]
}

fn main() {}
