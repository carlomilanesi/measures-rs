//! measures-rs version of the example `si.rs` of the crate `uom` version 0.35.0.
//! Example showing how to use the pre-built SI system.
/*
To run this, type:
cargo run --example si
Expected output:
15 m + 10 cm = 15.1 m
15 m + 10 cm = 0.0151 km
15 m / 50 s = 0.3 m/s
15 m / 50 s = 0.0003 km/s

UOM will print:
15 m + 9.999999 cm = 15.1 m
15 m + 9.999999 cm = 0.0151 km
15 m / 50 s = 0.3 m/s
15 m / 50 s = 0.0003 km/s
*/

mod units;
use units::{Centimetre, Kilometre, KilometrePerSecond, Measure, Metre, Second};

fn main() {
    let mo = Measure::<measures::dimensionless::One>::new(2.3);
    let m = Measure::<Centimetre>::new(3.6);
    let _r1 = m * mo;
    let _r2 = mo * m;

    // Setup length and time quantities using different units.
    let l1 = Measure::<Metre>::new(15.);
    let l2 = Measure::<Centimetre>::new(10.);
    let t1 = Measure::<Second>::new(50.0);
    let v1 = l1 / t1;
    //let error = l1 + t1; // error[E0308]: mismatched types

    // No need to setup format arguments.

    // Print results of simple formulas using different output units.
    println!("{} + {} = {}", l1, l2, l1 + l2.convert());
    println!(
        "{} + {} = {}",
        l1,
        l2,
        (l1 + l2.convert()).convert::<Kilometre>()
    );
    println!("{} / {} = {}", l1, t1, v1);
    println!("{} / {} = {}", l1, t1, v1.convert::<KilometrePerSecond>());
}
