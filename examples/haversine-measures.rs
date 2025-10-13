// To run this, type:
// cargo run --example haversine-measures
// Expected output:
// Distance: 2887.2599506071106 km (1794.0601578078463 mi)

measures::define_measure_types! {
    with_directions exact,
    vector_properties [
        Length [
            KiloMetre {
                suffix: " km",
            }
            Mile {
                suffix: " mi",
                ratio: 1.609344,
            }
        ]
    ]
    angle_measurement_units [
        Degree {
            suffix: " deg",
            cycle_fraction: 360.,
        }
    ]
}

struct Point {
    lat: SignedDirection<Degree>,
    lon: SignedDirection<Degree>,
}

fn haversine(origin: Point, destination: Point) -> Measure<KiloMetre> {
    const R: Measure<KiloMetre> = Measure::<KiloMetre>::new(6372.8);
    let a = ((destination.lat - origin.lat) / 2.0).sin().powi(2)
        + ((destination.lon - origin.lon) / 2.0).sin().powi(2)
            * origin.lat.cos()
            * destination.lat.cos();
    let c = 2.0 * a.sqrt().asin();
    R * c
}

fn main() {
    let origin = Point {
        lat: SignedDirection::new(36.12),
        lon: SignedDirection::new(-86.67),
    };
    let destination = Point {
        lat: SignedDirection::new(33.94),
        lon: SignedDirection::new(-118.4),
    };
    let d = haversine(origin, destination);
    println!("Distance: {} ({})", d, d.convert::<Mile>());
}
