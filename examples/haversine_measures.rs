// Run with: cargo run --example haversine_formula-measures
// Expected output: Distance: 2887.2599506071106 km (1794.0601578078463 mi)

rs_measures::define_measure_types! {
    with_points: false,
    with_directions: true,
    with_2d: false,
    with_3d: false,
    with_transformations: false,
    exact: true,
    with_approx: false,
    []
}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

pub struct Length;

pub struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}

pub struct Mile;
impl MeasurementUnit for Mile {
    type Property = Length;
    const RATIO: f64 = 1.609344;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi";
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
