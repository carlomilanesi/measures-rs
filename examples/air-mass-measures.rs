// Run with: cargo run --example air-mass-measures
// Expected output:
// Angle                          0 m              13700 m
// ---------------------------------------------------------
// at  0 deg (in 0°..360°)       1.00000000       1.00000000
// at  5 deg (in 0°..360°)       1.00380963       1.00380965
// at 10 deg (in 0°..360°)       1.01538466       1.01538475
// at 15 deg (in 0°..360°)       1.03517744       1.03517765
// at 20 deg (in 0°..360°)       1.06399053       1.06399093
// at 25 deg (in 0°..360°)       1.10305937       1.10306005
// at 30 deg (in 0°..360°)       1.15418974       1.15419083
// at 35 deg (in 0°..360°)       1.21998076       1.21998246
// at 40 deg (in 0°..360°)       1.30418931       1.30419190
// at 45 deg (in 0°..360°)       1.41234169       1.41234567
// at 50 deg (in 0°..360°)       1.55280404       1.55281025
// at 55 deg (in 0°..360°)       1.73875921       1.73876915
// at 60 deg (in 0°..360°)       1.99212000       1.99213665
// at 65 deg (in 0°..360°)       2.35199740       2.35202722
// at 70 deg (in 0°..360°)       2.89531368       2.89537287
// at 75 deg (in 0°..360°)       3.79582352       3.79596149
// at 80 deg (in 0°..360°)       5.53885809       5.53928113
// at 85 deg (in 0°..360°)      10.07896219      10.08115981
// at 90 deg (in 0°..360°)      34.32981136      34.36666557

measures::define_measure_types! {
    with_directions exact,
    vector_properties [
        Length [
            Meter {
                suffix: " m",
            }
        ]
        Area [
            SquareMeter  {
                suffix: " m\u{b2}",
            }
        ]
    ]
    angle_measurement_units [
        Degree {
            suffix: " deg",
            cycle_fraction: 360.,
        }
    ]
    relationships [
        SquareMeter 1 == Meter 1 * __ 1,
    ]
}

const RE: Measure<Meter> = Measure::<Meter>::new(6371000.0); // Earth radius in meters
const DD: f64 = 0.001; // integrate in this fraction of the distance already covered
const FIN: Measure<Meter> = Measure::<Meter>::new(10000000.0); // integrate only to a height of 10000km, effectively infinity

fn rho(a: Measure<Meter>) -> f64 {
    // the density of air as a function of height above sea level
    (-a / Measure::<Meter>::new(8500.0)).value.exp()
}

fn height(a: Measure<Meter>, z: UnsignedDirection<Degree>, d: Measure<Meter>) -> Measure<Meter> {
    // a = altitude of observer
    // z = zenith angle (in degrees)
    // d = distance along line of sight
    let aa = RE + a;
    let hh = (aa * aa + d * d + 2.0 * d * aa * z.cos()).sqrt();
    hh - RE
}

fn column_density(a: Measure<Meter>, z: UnsignedDirection<Degree>) -> f64 {
    // integrates density along the line of sight
    let mut sum = 0.0;
    let mut d = Measure::<Meter>::new(0.);
    while d < FIN {
        // adaptive step size to avoid it taking forever
        let mut delta = DD * d;
        if delta < Measure::<Meter>::new(DD) {
            delta = Measure::<Meter>::new(DD);
        }
        sum += rho(height(a, z, d + 0.5 * delta)) * delta.value;
        d += delta;
    }
    sum
}

fn airmass(a: Measure<Meter>, z: UnsignedDirection<Degree>) -> f64 {
    column_density(a, z) / column_density(a, UnsignedDirection::<Degree>::new(0.))
}

fn main() {
    println!("Angle                          0 m              13700 m");
    println!("---------------------------------------------------------");
    for a in (0..=90).step_by(5) {
        let z = UnsignedDirection::<Degree>::new(a as f64);
        println!(
            "{:2}      {:11.8}      {:11.8}",
            z,
            airmass(Measure::<Meter>::new(0.), z),
            airmass(Measure::<Meter>::new(13700.), z)
        );
    }
}
