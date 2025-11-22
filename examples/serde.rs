//! Example of serializing and deserializing a struct in JSON format, using serde and serde_json.
/*
To run this, type:
cargo run --example serde
Expected output:
"{\"position\":[178.0,179.0]}"
S { position: at (178, 179) m }
"{\"position\":[178.0,179.0,180.0]}"
S { position: at (178, 179, 180) m }
"{\"direction\":-2.9203136}"
S { direction: at -2.9203136 rad (in -180°..180°) }
"{\"direction\":3.362863}"
S { direction: at 3.362863 rad (in 0°..360°) }
"{\"name\":\"John Doe\",\"mass\":[72.3,0.1],\"height\":[178.0,0.4]}"
Person { name: "John Doe", mass: [µ=72.3 σ²=0.1] kg, height: [µ=178 σ²=0.4] m }
"{\"displacement\":[[178.0,179.0],[[0.1,0.2],[0.3,0.4]]]}"
S { displacement: values=(178, 179), covariances=
 ⎡ 0.1 0.2 ⎤ m
 ⎣ 0.3 0.4 ⎦ }
"{\"displacement\":[[178.0,179.0,180.0],[[0.1,0.2,0.3],[0.4,0.5,0.6],[0.7,0.8,0.9]]]}"
S { displacement: values=(178, 179, 180), covariances=
 ⎡ 0.1 0.2 0.3 ⎤ m
 ⎢ 0.4 0.5 0.6 ⎥
 ⎣ 0.7 0.8 0.9 ⎦ }
"{\"name\":\"John Doe\",\"position\":[278.0,0.1]}"
Person { name: "John Doe", position: at 278 ± 0.31622776 m }
"{\"position\":[[278.0,179.0],[[0.1,0.2],[0.3,0.4]]]}"
S { position: at values=(278, 179), covariances=
 ⎡ 0.1 0.2 ⎤ m
 ⎣ 0.3 0.4 ⎦ }
"{\"position\":[[278.0,179.0,180.0],[[0.1,0.2,0.3],[0.4,0.5,0.6],[0.7,0.8,0.9]]]}"
S { position: at values=(278, 179, 180), covariances=
 ⎡ 0.1 0.2 0.3 ⎤ m
 ⎢ 0.4 0.5 0.6 ⎥
 ⎣ 0.7 0.8 0.9 ⎦ }
*/
use serde::{Deserialize, Serialize};

measures::define_measure_types! {
    exact with_approx with_points with_directions with_2d with_3d with_serde,
    scalar_properties [
        Mass [
            Kilogram {
                suffix: " kg",
            }
        ]
    ]
    vector_properties [
        Length [
            Centimetre {
                suffix: " m",
            }
        ]
    ]
}

fn use_measure() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        mass: Measure<Kilogram, f32>,
        height: Measure<Centimetre, f32>,
    }
    let p1 = Person {
        name: "John Doe".to_string(),
        mass: Measure::<Kilogram, f32>::new(72.3),
        height: Measure::<Centimetre, f32>::new(178.),
    };

    let json = serde_json::to_string(&p1).unwrap();
    println!("{json:#?}");
    let p2 = serde_json::from_str::<Person>(&json).unwrap();
    println!("{p2:?}");
    assert_eq!(p1, p2);
}

fn use_measure_2d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        displacement: Measure2d<Centimetre, f32>,
    }
    let s1 = S {
        displacement: Measure2d::<Centimetre, f32>::new([178., 179.]),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_measure_3d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        displacement: Measure3d<Centimetre, f32>,
    }
    let s1 = S {
        displacement: Measure3d::<Centimetre, f32>::new([178., 179., 180.]),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_measure_point() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        position: MeasurePoint<Centimetre, f32>,
    }
    let p1 = Person {
        name: "John Doe".to_string(),
        position: MeasurePoint::<Centimetre, f32>::new(178.),
    };

    let json = serde_json::to_string(&p1).unwrap();
    println!("{json:#?}");
    let p2 = serde_json::from_str::<Person>(&json).unwrap();
    println!("{p2:?}");
    assert_eq!(p1, p2);
}

fn use_measure_point_2d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        position: MeasurePoint2d<Centimetre, f32>,
    }
    let s1 = S {
        position: MeasurePoint2d::<Centimetre, f32>::new([178., 179.]),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_measure_point_3d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        position: MeasurePoint3d<Centimetre, f32>,
    }
    let s1 = S {
        position: MeasurePoint3d::<Centimetre, f32>::new([178., 179., 180.]),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_signed_direction() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        direction: SignedDirection<Radian, f32>,
    }
    let s1 = S {
        direction: SignedDirection::<Radian, f32>::new(550.),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_unsigned_direction() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        direction: UnsignedDirection<Radian, f32>,
    }
    let s1 = S {
        direction: UnsignedDirection::<Radian, f32>::new(550.),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_approx_measure() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        mass: ApproxMeasure<Kilogram, f32>,
        height: ApproxMeasure<Centimetre, f32>,
    }
    let p1 = Person {
        name: "John Doe".to_string(),
        mass: ApproxMeasure::<Kilogram, f32>::with_variance(72.3, 0.1),
        height: ApproxMeasure::<Centimetre, f32>::with_variance(178., 0.4),
    };

    let json = serde_json::to_string(&p1).unwrap();
    println!("{json:#?}");
    let p2 = serde_json::from_str::<Person>(&json).unwrap();
    println!("{p2:?}");
    assert_eq!(p1, p2);
}

fn use_approx_measure_2d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        displacement: ApproxMeasure2d<Centimetre, f32>,
    }
    let s1 = S {
        displacement: ApproxMeasure2d::<Centimetre, f32>::with_covariances(
            [178., 179.],
            [[0.1, 0.2], [0.3, 0.4]],
        ),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_approx_measure_3d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        displacement: ApproxMeasure3d<Centimetre, f32>,
    }
    let s1 = S {
        displacement: ApproxMeasure3d::<Centimetre, f32>::with_covariances(
            [178., 179., 180.],
            [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6], [0.7, 0.8, 0.9]],
        ),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_approx_measure_point() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        position: ApproxMeasurePoint<Centimetre, f32>,
    }
    let p1 = Person {
        name: "John Doe".to_string(),
        position: ApproxMeasurePoint::<Centimetre, f32>::with_variance(278., 0.1),
    };

    let json = serde_json::to_string(&p1).unwrap();
    println!("{json:#?}");
    let p2 = serde_json::from_str::<Person>(&json).unwrap();
    println!("{p2:?}");
    assert_eq!(p1, p2);
}

fn use_approx_measure_point_2d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        position: ApproxMeasurePoint2d<Centimetre, f32>,
    }
    let s1 = S {
        position: ApproxMeasurePoint2d::<Centimetre, f32>::with_covariances(
            [278., 179.],
            [[0.1, 0.2], [0.3, 0.4]],
        ),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn use_approx_measure_point_3d() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S {
        position: ApproxMeasurePoint3d<Centimetre, f32>,
    }
    let s1 = S {
        position: ApproxMeasurePoint3d::<Centimetre, f32>::with_covariances(
            [278., 179., 180.],
            [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6], [0.7, 0.8, 0.9]],
        ),
    };
    let json = serde_json::to_string(&s1).unwrap();
    println!("{json:#?}");
    let s2 = serde_json::from_str::<S>(&json).unwrap();
    println!("{s2:?}");
    assert_eq!(s1, s2);
}

fn main() {
    use_measure();
    use_measure_2d();
    use_measure_3d();
    use_measure_point();
    use_measure_point_2d();
    use_measure_point_3d();
    use_signed_direction();
    use_unsigned_direction();

    use_approx_measure();
    use_approx_measure_2d();
    use_approx_measure_3d();
    use_approx_measure_point();
    use_approx_measure_point_2d();
    use_approx_measure_point_3d();
}
