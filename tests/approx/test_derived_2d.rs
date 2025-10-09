use measures::traits::CrossProduct;

measures::define_measure_types! {
    with_2d exact,
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
    ]
    dimensionless_measurement_units [ ]
    angle_measurement_units [ ]
    relationships [
        Metre 2 == MetrePerSecond 2 * Second 1,
        Metre 1 == Metre 2 * __ 2,
        Joule 1 == Newton 2 * Metre 2,
        NewtonMetre 1 == Newton 2 X Metre 2,
        Metre 1 == Metre 2 X __ 2,
    ]
}

measures::measurement_vector_property! { Torque }

measures::measurement_unit! {
    name: NewtonMetre,
    property: Torque,
    suffix: " N\u{b7}m",
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
