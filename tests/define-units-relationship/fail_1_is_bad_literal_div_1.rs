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
    ]
    angle_measurement_units [ ]
    relationships [
        U1 1 == U2 1 / __ 1,
    ]
}

fn main() {}
