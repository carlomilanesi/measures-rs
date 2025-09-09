use rs_measures::traits::CrossProduct;

rs_measures::define_measure_types! {
    with_points: false,
    with_directions: false,
    with_2d: false,
    with_3d: true,
    with_transformations: false,
    exact: true,
    with_approx: false,
    [
        Metre 3 == MetrePerSecond 3 * Second 1,
        Metre 1 == Metre 3 * __ 3,
        Joule 1 == Newton 3 * Metre 3,
        NewtonMetre 3 == Newton 3 X Metre 3,
        Metre 3 == Metre 3 X __ 3,
    ]
}

struct Length;
impl VectorProperty for Length {}

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct Time;

struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Velocity;
impl VectorProperty for Velocity {}

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Force;
impl VectorProperty for Force {}

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

pub struct Torque;
impl VectorProperty for Torque {}

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}

#[test]
fn test_relationship_1_3() {
    let metres_per_second_3d = Measure3d::<MetrePerSecond>::new(5., -3., 6.);
    let seconds = Measure::<Second>::new(7.);
    let metres_3d: Measure3d<Metre> = metres_per_second_3d * seconds;
    assert_eq!(metres_3d.x, 35.);
    assert_eq!(metres_3d.y, -21.);
    assert_eq!(metres_3d.z, 42.);
    let metres_3d: Measure3d<Metre> = seconds * metres_per_second_3d;
    assert_eq!(metres_3d.x, 35.);
    assert_eq!(metres_3d.y, -21.);
    assert_eq!(metres_3d.z, 42.);
    let meters_per_second_3d: Measure3d<MetrePerSecond> = metres_3d / seconds;
    assert_eq!(meters_per_second_3d.x, 5.);
    assert_eq!(meters_per_second_3d.y, -3.);
    assert_eq!(meters_per_second_3d.z, 6.);
}

#[test]
fn test_relationship_3_3() {
    let newtons_3d = Measure3d::<Newton>::new(5., -3., 6.);
    let metres_3d = Measure3d::<Metre>::new(7., 4., -8.);
    let joules: Measure<Joule> = newtons_3d * metres_3d;
    assert_eq!(joules.value, -25.);
    let joules: Measure<Joule> = metres_3d * newtons_3d;
    assert_eq!(joules.value, -25.);
}

#[test]
fn test_relationship_3_3_same() {
    let metres_3d_1 = Measure3d::<Metre>::new(5., -3., 6.);
    let metres_3d_2 = Measure3d::<Metre>::new(7., 4., -8.);
    let metres: Measure<Metre> = metres_3d_1 * metres_3d_2;
    assert_eq!(metres.value, -25.);
    let metres: Measure<Metre> = metres_3d_1.squared();
    assert_eq!(metres.value, 70.);
}

#[test]
fn test_relationship_cross_3() {
    let newton_3d = Measure3d::<Newton>::new(5., -3., 6.);
    let metres_3d = Measure3d::<Metre>::new(7., 4., -8.);
    let newton_metres_3d: Measure3d<NewtonMetre> = newton_3d.cross_product(metres_3d);
    assert_eq!(newton_metres_3d.x, 0.);
    assert_eq!(newton_metres_3d.y, 82.);
    assert_eq!(newton_metres_3d.z, 41.);
    let newton_metres_3d: Measure3d<NewtonMetre> = metres_3d.cross_product(newton_3d);
    assert_eq!(newton_metres_3d.x, 0.);
    assert_eq!(newton_metres_3d.y, -82.);
    assert_eq!(newton_metres_3d.z, -41.);
}

#[test]
fn test_relationship_cross_3_same() {
    let metres_3d_1 = Measure3d::<Metre>::new(5., -3., 6.);
    let metres_3d_2 = Measure3d::<Metre>::new(7., 4., -8.);
    let metres_3d_3: Measure3d<Metre> = metres_3d_1.cross_product(metres_3d_2);
    assert_eq!(metres_3d_3.x, 0.);
    assert_eq!(metres_3d_3.y, 82.);
    assert_eq!(metres_3d_3.z, 41.);
}
