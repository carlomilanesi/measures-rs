measures::define_measure_types! {
    with_points: false,
    with_directions: false,
    with_2d: false,
    with_3d: false,
    with_transformations: false,
    exact: true,
    with_approx: false,
    [
        Volt 1 == Ampere 1 * Ohm 1,
        SquareAmpere 1 == Ampere 1 * __ 1,
    ]
}

pub struct ElectricPotential;

pub struct Volt;
impl MeasurementUnit for Volt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V";
}

pub struct SquareElectricCurrent;

pub struct SquareAmpere;
impl MeasurementUnit for SquareAmpere {
    type Property = SquareElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A\u{b2}";
}

pub struct ElectricCurrent;

pub struct Ampere;
impl MeasurementUnit for Ampere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A";
}

pub struct ElectricalResistance;

pub struct Ohm;
impl MeasurementUnit for Ohm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}";
}

#[derive(Clone, Copy, PartialEq)]
enum Fixed {
    A,
    B,
    Free,
}

#[derive(Clone)]
struct Node {
    current: Measure<Ampere>,
    fixed: Fixed,
}

impl Node {
    fn new() -> Self {
        Self {
            current: Measure::<Ampere>::new(0.),
            fixed: Fixed::Free,
        }
    }
}

type Mesh = Vec<Vec<Node>>;

fn set_boundary(m: &mut Mesh) {
    m[1][1] = Node {
        current: Measure::<Ampere>::new(1.),
        fixed: Fixed::A,
    };
    m[6][7] = Node {
        current: Measure::<Ampere>::new(-1.),
        fixed: Fixed::B,
    };
}

fn calculate_difference(m: &Mesh, d: &mut Mesh) -> Measure<SquareAmpere> {
    let w = m[0].len();
    let h = m.len();
    let mut total = Measure::<SquareAmpere>::new(0.);
    for i in 0..h {
        for j in 0..w {
            let mut current = Measure::<Ampere>::new(0.);
            let mut n = 0;
            if i > 0 {
                current += m[i - 1][j].current;
                n += 1;
            }
            if j > 0 {
                current += m[i][j - 1].current;
                n += 1;
            }
            if i + 1 < h {
                current += m[i + 1][j].current;
                n += 1;
            }
            if j + 1 < w {
                current += m[i][j + 1].current;
                n += 1;
            }
            current = m[i][j].current - current / n as f64;
            d[i][j].current = current;
            if m[i][j].fixed == Fixed::Free {
                total += current * current;
            }
        }
    }
    total
}

fn iter(m: &mut Mesh) -> Measure<Ampere> {
    let w = m[0].len();
    let h = m.len();
    let mut d = vec![vec![Node::new(); w]; h];
    let mut diff = Measure::<SquareAmpere>::new(1e10);
    let tolerance = Measure::<SquareAmpere>::new(1e-24);
    while diff > tolerance {
        set_boundary(m);
        diff = calculate_difference(m, &mut d);
        for i in 0..h {
            for j in 0..w {
                m[i][j].current -= d[i][j].current;
            }
        }
    }

    let mut current_a = Measure::<Ampere>::new(0.);
    let mut current_b = Measure::<Ampere>::new(0.);
    for i in 0..h {
        for j in 0..w {
            let mut k = 0;
            if i != 0 {
                k += 1;
            }
            if j != 0 {
                k += 1;
            }
            if i < h - 1 {
                k += 1;
            }
            if j < w - 1 {
                k += 1;
            }
            let increment = d[i][j].current * k as f64;
            match m[i][j].fixed {
                Fixed::A => current_a += increment,
                Fixed::B => current_b += increment,
                Fixed::Free => {}
            }
        }
    }
    (current_a - current_b) / 2.
}

fn main() {
    const SIZE: usize = 10;
    let mut mesh = vec![vec![Node::new(); SIZE]; SIZE];
    let r = Measure::<Volt>::new(2.) / iter(&mut mesh);
    println!("R = {r:.14}");
}
