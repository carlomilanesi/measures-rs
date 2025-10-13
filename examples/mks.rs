//! measures-rs version of the example `mks.rs` of the crate `uom` version 0.35.0.
//! Example showing how to create a custom system of quantities.
/*
To run this, type:
cargo run --example mks
Expected output:
100 m = 328.08398 ft

The corresponding version using UOM would print:
100 m = 328.08398 ft
*/

measures::define_measure_types! {
    exact,
    vector_properties [
        Length [
            Metre {
                suffix: " m",
            }
            Foot {
                suffix: " ft",
                ratio: 0.3048,
            }
        ]
    ]
}

fn main() {
    let l1 = Measure::<Metre, f32>::new(100.);
    println!("{} = {}", l1, l1.convert::<Foot>());
}
