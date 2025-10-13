//! measures-rs version of the example `unit.rs` of the crate `uom` version 0.35.0.
//! Example showing how to add new units to existing quantities.
/*
To run this, type:
cargo run --example user_defined_unit
Expected output:
15 m = 8.813161 smoot
1 smoot = 1.702 m

The corresponding version using UOM would print:
15 m = 8.813161 smoot
1 smoot = 1.702 m
*/

mod units;
use units::Length;
use units::Measure;

// This is an internal macro, not meant to be used be application developers.
// The proper way to define units of measurement is in the single call
// to the macro `measures::define_measure_types!`.
// However, if, for some reason, it is needed to define a measurement unit
// without modifying that call, this is the way to do it.
measures::measurement_unit! {
    name: Smoot,
    property: Length,
    suffix: " smoot",
    ratio: 1.702,
    offset: 0.,
    with_2d: false,
    with_3d: false,
    vector: true,
}

fn main() {
    let l1 = Measure::<units::Metre, f32>::new(15.);
    let l2 = Measure::<Smoot, f32>::new(1.);

    println!("{} = {}", l1, l1.convert::<Smoot>());
    println!("{} = {}", l2, l2.convert::<units::Metre>());
}
