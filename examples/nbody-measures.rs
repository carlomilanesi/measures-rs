// Conversion of the benchmark "The Computer Language Benchmarks Game"
// to use the library `measures` with exact values
// https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-rust-6.html
//
// To build this, type:
// cargo build --release --example nbody-measures
// And then, to run the generated executable program, type:
// /bin/time target/release/examples/nbody-measures 50000000
// Expected output:
// -0.169075164 J
// -0.169059907 J

measures::define_measure_types! {
    with_points with_3d exact,
    scalar_properties [
        Area [
            SquareMetre {
                suffix: " m2",
            }
        ]
        Volume [
            CubicMetre {
                suffix: " m3",
            }
        ]
        TimePerVolume [
            SecondPerCubicMetre {
                suffix: " s/m3",
            }
        ]
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
        Mass [
            Kilogram {
                suffix: " kg",
            }
        ]
        GravitationalConstant [
            NewtonSquareMetrePerSquareKilogram {
                suffix: " G",
            }
        ]
        SquareVelocity [
            SquareMetrePerSquareSecond {
                suffix: " m2/s2",
            }
        ]
        SquareMass [
            SquareKilogram {
                suffix: " kg2",
            }
        ]
        SquareMassPerLength [
            SquareKilogramPerMetre {
                suffix: " km2/m",
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
        TimePerArea [
            SecondPerSquareMetre {
                suffix: " s/m2",
            }
        ]
        MassTimePerArea [
            KilogramSecondPerSquareMetre {
                suffix: " kg s/m2",
            }
        ]
    ]
    relationships [
        Metre 3 == MetrePerSecond 3 * Second 1,
        SquareMetre 1 == Metre 1 * __ 1,
        SquareMetre 1 == Metre 3 * __ 3,
        CubicMetre 1 == SquareMetre 1 * Metre 1,
        Second 1 == SecondPerCubicMetre 1 * CubicMetre 1,
        SquareMetrePerSquareSecond 1 == MetrePerSecond 3 * __ 3,
        SquareKilogram 1 == Kilogram 1 * __ 1,
        Joule 1 == Kilogram 1 * SquareMetrePerSquareSecond 1,
        SquareKilogram 1 == SquareKilogramPerMetre 1 * Metre 1,
        Joule 1 == NewtonSquareMetrePerSquareKilogram 1 * SquareKilogramPerMetre 1,
        SecondPerSquareMetre 3 == Metre 3 * SecondPerCubicMetre 1,
        KilogramSecondPerSquareMetre 3 == SecondPerSquareMetre 3 * Kilogram 1,
        MetrePerSecond 3 == KilogramSecondPerSquareMetre 3 * NewtonSquareMetrePerSquareKilogram 1,
    ]
}

use std::f64::consts::PI;

const SOLAR_MASS: Measure<Kilogram> = Measure::<Kilogram>::new(4.0 * PI * PI);
const DPY: f64 = 365.24;

pub struct Body {
    pub x: MeasurePoint3d<Metre>,
    pub mass: Measure<Kilogram>, // By putting `mass` here, the alignment is improved.
    pub v: Measure3d<MetrePerSecond>,
}

const N_BODIES: usize = 5;
#[allow(clippy::excessive_precision)]
fn bodies() -> [Body; N_BODIES] {
    [
        // sun:
        Body {
            x: MeasurePoint3d::<Metre>::default(),
            v: Measure3d::<MetrePerSecond>::default(),
            mass: SOLAR_MASS,
        },
        // jupiter:
        Body {
            x: MeasurePoint3d::new([
                4.84143144246472090e+00,
                -1.16032004402742839e+00,
                -1.03622044471123109e-01,
            ]),
            v: Measure3d::new([
                1.66007664274403694e-03 * DPY,
                7.69901118419740425e-03 * DPY,
                -6.90460016972063023e-05 * DPY,
            ]),
            mass: 9.54791938424326609e-04 * SOLAR_MASS,
        },
        // saturn:
        Body {
            x: MeasurePoint3d::new([
                8.34336671824457987e+00,
                4.12479856412430479e+00,
                -4.03523417114321381e-01,
            ]),
            v: Measure3d::new([
                -2.76742510726862411e-03 * DPY,
                4.99852801234917238e-03 * DPY,
                2.30417297573763929e-05 * DPY,
            ]),
            mass: 2.85885980666130812e-04 * SOLAR_MASS,
        },
        // uranus:
        Body {
            x: MeasurePoint3d::new([
                1.28943695621391310e+01,
                -1.51111514016986312e+01,
                -2.23307578892655734e-01,
            ]),
            v: Measure3d::new([
                2.96460137564761618e-03 * DPY,
                2.37847173959480950e-03 * DPY,
                -2.96589568540237556e-05 * DPY,
            ]),
            mass: 4.36624404335156298e-05 * SOLAR_MASS,
        },
        // neptune:
        Body {
            x: MeasurePoint3d::new([
                1.53796971148509165e+01,
                -2.59193146099879641e+01,
                1.79258772950371181e-01,
            ]),
            v: Measure3d::new([
                2.68067772490389322e-03 * DPY,
                1.62824170038242295e-03 * DPY,
                -9.51592254519715870e-05 * DPY,
            ]),
            mass: 5.15138902046611451e-05 * SOLAR_MASS,
        },
    ]
}

pub fn offset_momentum(bodies: &mut [Body; N_BODIES]) {
    let (sun, rest) = bodies.split_at_mut(1);
    let sun = &mut sun[0];
    for body in rest {
        let m_ratio = body.mass / SOLAR_MASS;
        sun.v -= body.v * m_ratio;
    }
}

pub fn energy(bodies: &[Body; N_BODIES]) -> Measure<Joule> {
    let g = Measure::<NewtonSquareMetrePerSquareKilogram>::new(1.);
    let mut e = Measure::<Joule>::default();
    for i in 0..N_BODIES {
        let bi = &bodies[i];
        e += bi.mass * (bi.v * bi.v) * 0.5;
        for bj in &bodies[i + 1..] {
            let dx = bi.x - bj.x;
            e -= g * (bi.mass * bj.mass / (dx * dx).sqrt());
        }
    }
    e
}

pub fn advance(bodies: &mut [Body; N_BODIES], dt: Measure<Second>) {
    const N: usize = N_BODIES * (N_BODIES - 1) / 2;

    // compute distance between bodies:
    let mut r = [Measure3d::<Metre>::default(); N];
    let mut i = 0;
    for j in 0..N_BODIES {
        for k in j + 1..N_BODIES {
            r[i] = bodies[j].x - bodies[k].x;
            i += 1;
        }
    }

    let mut mag = [Measure::<SecondPerCubicMetre>::default(); N];
    for i in 0..N {
        let d1 = r[i] * r[i];
        mag[i] = dt / (d1 * d1.sqrt());
    }

    let g = Measure::<NewtonSquareMetrePerSquareKilogram>::new(1.);
    let mut i = 0;
    for j in 0..N_BODIES {
        for k in j + 1..N_BODIES {
            let f = r[i] * mag[i];
            bodies[j].v -= f * bodies[k].mass * g;
            bodies[k].v += f * bodies[j].mass * g;
            i += 1
        }
    }

    for body in bodies {
        body.x += body.v * dt;
    }
}

fn run(n: usize) -> (Measure<Joule>, Measure<Joule>) {
    let mut bodies = bodies();
    offset_momentum(&mut bodies);
    let energy_before = energy(&bodies);
    for _ in 0..n {
        advance(&mut bodies, Measure::<Second>::new(0.01));
    }
    let energy_after = energy(&bodies);
    (energy_before, energy_after)
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let (energy_before, energy_after) = run(n);
    println!("{:.9}\n{:.9}", energy_before, energy_after,);
}
