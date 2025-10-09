use measures::traits::CrossProduct;

measures::define_measure_types! {
    with_3d exact,
    scalar_properties [
        Time [
            Second {
                suffix: " s",
            }
        ]
        Energy [
            Joule {
                suffix: " J",
            }
        ]
    ]
    vector_properties [
        Length [
            Metre {
                suffix: " m",
            }
        ]
        Velocity [
            MetrePerSecond {
                suffix: " m/s",
            }
        ]
        Force [
            Newton {
                suffix: " N",
            }
        ]
        Torque [
            NewtonMetre {
                suffix: " N\u{b7}m",
            }
        ]
    ]
    dimensionless_measurement_units [ ]
    angle_measurement_units [ ]
    relationships [
        Metre 3 == MetrePerSecond 3 * Second 1,
        Metre 1 == Metre 3 * __ 3,
        Joule 1 == Newton 3 * Metre 3,
        NewtonMetre 3 == Newton 3 X Metre 3,
        Metre 3 == Metre 3 X __ 3,
    ]
}

#[test]
fn test_relationship_1_3() {
    let metres_per_second_3d = Measure3d::<MetrePerSecond>::new([5., -3., 6.]);
    let seconds = Measure::<Second>::new(7.);
    let metres_3d: Measure3d<Metre> = metres_per_second_3d * seconds;
    assert_eq!(metres_3d.values, [35., -21., 42.]);
    let metres_3d: Measure3d<Metre> = seconds * metres_per_second_3d;
    assert_eq!(metres_3d.values, [35., -21., 42.]);
    let meters_per_second_3d: Measure3d<MetrePerSecond> = metres_3d / seconds;
    assert_eq!(meters_per_second_3d.values, [5., -3., 6.]);
}

#[test]
fn test_relationship_3_3() {
    let newtons_3d = Measure3d::<Newton>::new([5., -3., 6.]);
    let metres_3d = Measure3d::<Metre>::new([7., 4., -8.]);
    let joules: Measure<Joule> = newtons_3d * metres_3d;
    assert_eq!(joules.value, -25.);
    let joules: Measure<Joule> = metres_3d * newtons_3d;
    assert_eq!(joules.value, -25.);
}

#[test]
fn test_relationship_3_3_same() {
    let metres_3d_1 = Measure3d::<Metre>::new([5., -3., 6.]);
    let metres_3d_2 = Measure3d::<Metre>::new([7., 4., -8.]);
    let metres: Measure<Metre> = metres_3d_1 * metres_3d_2;
    assert_eq!(metres.value, -25.);
    let metres: Measure<Metre> = metres_3d_1.squared();
    assert_eq!(metres.value, 70.);
}

#[test]
fn test_relationship_cross_3() {
    let newton_3d = Measure3d::<Newton>::new([5., -3., 6.]);
    let metres_3d = Measure3d::<Metre>::new([7., 4., -8.]);
    let newton_metres_3d: Measure3d<NewtonMetre> = newton_3d.cross_product(metres_3d);
    assert_eq!(newton_metres_3d.values, [0., 82., 41.]);
    let newton_metres_3d: Measure3d<NewtonMetre> = metres_3d.cross_product(newton_3d);
    assert_eq!(newton_metres_3d.values, [0., -82., -41.]);
}

#[test]
fn test_relationship_cross_3_same() {
    let metres_3d_1 = Measure3d::<Metre>::new([5., -3., 6.]);
    let metres_3d_2 = Measure3d::<Metre>::new([7., 4., -8.]);
    let metres_3d_3: Measure3d<Metre> = metres_3d_1.cross_product(metres_3d_2);
    assert_eq!(metres_3d_3.values, [0., 82., 41.]);
}
