#[derive(Clone, Copy, PartialEq)]
enum Fixed {
    A,
    B,
    Free,
}

#[derive(Clone)]
struct Node {
    current: f64,
    fixed: Fixed,
}

impl Node {
    fn new() -> Self {
        Self {
            current: 0.,
            fixed: Fixed::Free,
        }
    }
}

type Mesh = Vec<Vec<Node>>;

fn set_boundary(m: &mut Mesh) {
    m[1][1] = Node {
        current: 1.,
        fixed: Fixed::A,
    };
    m[6][7] = Node {
        current: -1.,
        fixed: Fixed::B,
    };
}

fn calculate_difference(m: &Mesh, d: &mut Mesh) -> f64 {
    let w = m[0].len();
    let h = m.len();
    let mut total = 0.;
    for i in 0..h {
        for j in 0..w {
            let mut current = 0.;
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

fn iter(m: &mut Mesh) -> f64 {
    let w = m[0].len();
    let h = m.len();
    let mut d = vec![vec![Node::new(); w]; h];
    let mut diff = 1e10;
    let tolerance = 1e-24;
    while diff > tolerance {
        set_boundary(m);
        diff = calculate_difference(m, &mut d);
        for i in 0..h {
            for j in 0..w {
                m[i][j].current -= d[i][j].current;
            }
        }
    }

    let mut current_a = 0.;
    let mut current_b = 0.;
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
    let r = 2. / iter(&mut mesh);
    println!("R = {r:.14}");
}
