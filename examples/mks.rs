//! measures-rs version of the example `mks.rs` of the crate `uom` version 0.35.0.
//! Example showing how to create a custom system of quantities.
/*
measures-rs will print:
100 m = 328.08398 ft

UOM will print:
100 m = 328.08398 ft
*/

measures::define_measure_types! {
    exact,
    scalar_properties               [ ]
    vector_properties               [ ]
    dimensionless_measurement_units [ ]
    angle_measurement_units         [ ]
    relationships [ ]
}

fn main() {
    let l1 = Measure::<Metre, f32>::new(100.);
    println!("{} = {}", l1, l1.convert::<Foot>());
}

measures::measurement_vector_property! { Length }

measures::measurement_unit! {
    name: Metre,
    property: Length,
    suffix: " m",
}

measures::measurement_unit! {
    name: Foot,
    property: Length,
    suffix: " ft",
    ratio: 0.3048,
}
