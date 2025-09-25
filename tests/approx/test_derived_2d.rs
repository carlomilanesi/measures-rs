use measures::traits::CrossProduct;

measures::define_measure_types! {
    with_2d exact,
    [
        Metre 2 == MetrePerSecond 2 * Second 1,
        Metre 1 == Metre 2 * __ 2,
        Joule 1 == Newton 2 * Metre 2,
        NewtonMetre 1 == Newton 2 X Metre 2,
        Metre 1 == Metre 2 X __ 2,
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
fn test_relationship_1_2() {
    let metres_per_second_2d = Measure2d::<MetrePerSecond>::new([5., -3.]);
    let seconds = Measure::<Second>::new(7.);
    let metres_2d: Measure2d<Metre> = metres_per_second_2d * seconds;
    assert_eq!(metres_2d.values, [35., -21.]);
    let metres_2d: Measure2d<Metre> = seconds * metres_per_second_2d;
    assert_eq!(metres_2d.values, [35., -21.]);
    let meters_per_second_2d: Measure2d<MetrePerSecond> = metres_2d / seconds;
    assert_eq!(meters_per_second_2d.values, [5., -3.]);
}

#[test]
fn test_relationship_2_2() {
    let newtons_2d = Measure2d::<Newton>::new([5., -3.]);
    let metres_2d = Measure2d::<Metre>::new([7., 4.]);
    let joules: Measure<Joule> = newtons_2d * metres_2d;
    assert_eq!(joules.value, 23.);
    let joules: Measure<Joule> = metres_2d * newtons_2d;
    assert_eq!(joules.value, 23.);
}

#[test]
fn test_relationship_2_2_same() {
    let metres_2d_1 = Measure2d::<Metre>::new([5., -3.]);
    let metres_2d_2 = Measure2d::<Metre>::new([7., 4.]);
    let metres: Measure<Metre> = metres_2d_1 * metres_2d_2;
    assert_eq!(metres.value, 23.);
    let metres: Measure<Metre> = metres_2d_1.squared();
    assert_eq!(metres.value, 34.);
}

#[test]
fn test_relationship_cross_2() {
    let newton_2d = Measure2d::<Newton>::new([5., -3.]);
    let metres_2d = Measure2d::<Metre>::new([7., 4.]);
    let newton_metres: Measure<NewtonMetre> = newton_2d.cross_product(metres_2d);
    assert_eq!(newton_metres.value, 41.);
    let newton_metres: Measure<NewtonMetre> = metres_2d.cross_product(newton_2d);
    assert_eq!(newton_metres.value, -41.);
}

#[test]
fn test_relationship_cross_2_same() {
    let metres_2d_1 = Measure2d::<Metre>::new([5., -3.]);
    let metres_2d_2 = Measure2d::<Metre>::new([7., 4.]);
    let metres: Measure<Metre> = metres_2d_1.cross_product(metres_2d_2);
    assert_eq!(metres.value, 41.);
}
