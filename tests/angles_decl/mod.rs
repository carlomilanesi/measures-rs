measures::define_measure_types! {
    exact,
    scalar_properties [ ]
    vector_properties [ ]
    angle_measurement_units [ ]
    relationships [ ]
}

measures::angle_measurement_unit! {
    name: Cycle,
    suffix: " cycles",
    cycle_fraction: 1.,
}

measures::angle_measurement_unit! {
    name: Degree,
    suffix: " deg",
    cycle_fraction: 360.,
}
