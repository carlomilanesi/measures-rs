//! measures-rs version of the example `unit.rs` of the crate `uom` version 0.35.0.
//! Example showing how to add new units to existing quantities.
mod units;
use units::Length;
use units::Measure;

measures::measurement_unit! {
    name: Smoot,
    property: Length,
    ratio: 1.702,
    offset: 0.,
    suffix: " smoot",
}

fn main() {
    let l1 = Measure::<units::Metre, f32>::new(15.);
    let l2 = Measure::<Smoot, f32>::new(1.);

    println!("{} = {}", l1, l1.convert::<Smoot>());
    println!("{} = {}", l2, l2.convert::<units::Metre>());
}
/*
measures-rs will print:
15 m = 8.813161 smoot
1 smoot = 1.702 m

UOM will print:
15 m = 8.813161 smoot
1 smoot = 1.702 m
*/
