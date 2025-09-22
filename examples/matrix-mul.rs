// Build and run with:
//     cargo run --release --example matrix-mul

measures::define_measure_types! {
    exact with_approx,
    [
        Joule 1 == Newton 1 * Metre 1,
    ]
}

pub struct Length;
impl VectorProperty for Length {}

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
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

const SIZE: usize = 192;

fn elapsed(start: Instant, x: f64) -> Duration {
    if (x - 10546443672.111534).abs() / x < 1e-14 {
        start.elapsed()
    } else {
        Duration::ZERO
    }
}

#[allow(clippy::needless_range_loop)]
fn array_naked_matrix_multiplication() -> Duration {
    let mut mat1 = [[0.; SIZE]; SIZE];
    let mut mat2 = [[0.; SIZE]; SIZE];
    let mut mat3 = [[0.; SIZE]; SIZE];
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[row][column] += x;
            x += 1.7;
            mat2[row][column] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    elapsed(start, mat3[SIZE / 2][SIZE / 2])
}

extern crate nalgebra as na;
use std::time::{Duration, Instant};

use na::{ArrayStorage, Const, Matrix};

fn nalgebra_naked_matrix_builtin_multiplication() -> Duration {
    type Mat = Matrix<f64, Const<SIZE>, Const<SIZE>, ArrayStorage<f64, SIZE, SIZE>>;
    let mut mat1 = Mat::zeros();
    let mut mat2 = Mat::zeros();
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    let mat3: Mat = mat1 * mat2;
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

fn nalgebra_naked_matrix_explicit_multiplication() -> Duration {
    type Mat = Matrix<f64, Const<SIZE>, Const<SIZE>, ArrayStorage<f64, SIZE, SIZE>>;
    let mut mat1 = Mat::zeros();
    let mut mat2 = Mat::zeros();
    let mut mat3 = Mat::zeros();
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

use faer::Mat;

fn faer_naked_matrix_builtin_multiplication() -> Duration {
    let mut mat1 = Mat::<f64>::zeros(SIZE, SIZE);
    let mut mat2 = Mat::<f64>::zeros(SIZE, SIZE);
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    let mat3: Mat<f64> = mat1 * mat2;
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

fn faer_naked_matrix_explicit_multiplication() -> Duration {
    let mut mat1 = Mat::<f64>::zeros(SIZE, SIZE);
    let mut mat2 = Mat::<f64>::zeros(SIZE, SIZE);
    let mut mat3 = Mat::<f64>::zeros(SIZE, SIZE);
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

use ndarray::{Array2, ShapeBuilder};

impl<Unit, Number> num_traits::identities::Zero for Measure<Unit, Number>
where
    Unit: MeasurementUnit,
    Number: ArithmeticOps,
{
    fn zero() -> Self {
        Self::new(Number::ZERO)
    }
    fn is_zero(&self) -> bool {
        self.value == Number::ZERO
    }
}

fn ndarray_naked_matrix_builtin_multiplication() -> Duration {
    let mut mat1 = Array2::<f64>::zeros((SIZE, SIZE).f());
    let mut mat2 = Array2::<f64>::zeros((SIZE, SIZE).f());
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    let mat3: Array2<f64> = mat1.dot(&mat2);
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

fn ndarray_naked_matrix_explicit_multiplication() -> Duration {
    let mut mat1 = Array2::<f64>::zeros((SIZE, SIZE).f());
    let mut mat2 = Array2::<f64>::zeros((SIZE, SIZE).f());
    let mut mat3 = Array2::<f64>::zeros((SIZE, SIZE).f());
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += x;
            x += 1.7;
            mat2[(row, column)] += x;
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)])
}

fn ndarray_measure_matrix_explicit_multiplication() -> Duration {
    let mut mat1 = Array2::<Measure<Newton>>::zeros((SIZE, SIZE).f());
    let mut mat2 = Array2::<Measure<Metre>>::zeros((SIZE, SIZE).f());
    let mut mat3 = Array2::<Measure<Joule>>::zeros((SIZE, SIZE).f());
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[(row, column)] += Measure::<Newton>::new(x);
            x += 1.7;
            mat2[(row, column)] += Measure::<Metre>::new(x);
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    elapsed(start, mat3[(SIZE / 2, SIZE / 2)].value)
}

#[allow(clippy::needless_range_loop)]
fn array_measure_matrix_multiplication() -> Duration {
    let mut mat1 = [[Measure::<Newton>::new(0.); SIZE]; SIZE];
    let mut mat2 = [[Measure::<Metre>::new(0.); SIZE]; SIZE];
    let mut mat3 = [[Measure::<Joule>::new(0.); SIZE]; SIZE];
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[row][column] += Measure::<Newton>::new(x);
            x += 1.7;
            mat2[row][column] += Measure::<Metre>::new(x);
            x -= 1.3;
        }
    }
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    elapsed(start, mat3[SIZE / 2][SIZE / 2].value)
}

#[allow(clippy::needless_range_loop)]
fn array_approx_measure_matrix_multiplication() -> Duration {
    let mat1 = [[ApproxMeasure::<Newton>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mat2 = [[ApproxMeasure::<Metre>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat3 = [[ApproxMeasure::<Joule>::with_variance(0., 0.); SIZE]; SIZE];
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    elapsed(start, mat3[SIZE / 2][SIZE / 2].value)
}

#[allow(clippy::needless_range_loop)]
fn array_approx_measure_values_matrix_multiplication() -> Duration {
    let mat1 = [[ApproxMeasure::<Newton>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mat2 = [[ApproxMeasure::<Metre>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat3 = [[ApproxMeasure::<Joule>::with_variance(0., 0.); SIZE]; SIZE];
    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[row][column].value += mat1[row][step].value * mat2[step][column].value;
            }
        }
    }
    elapsed(start, mat3[SIZE / 2][SIZE / 2].value)
}

fn bench(f: fn() -> Duration) -> f64 {
    f();
    let mut t = Duration::ZERO;
    for _ in 0..16 {
        t += f();
    }
    t.as_nanos() as f64 / 1e7
}

fn main() {
    println!(
        "array naked: {:.3}.",
        bench(array_naked_matrix_multiplication)
    );
    println!(
        "nalgebra naked builtin: {:.3}.",
        bench(nalgebra_naked_matrix_builtin_multiplication)
    );
    println!(
        "nalgebra naked explicit: {:.3}.",
        bench(nalgebra_naked_matrix_explicit_multiplication)
    );
    println!(
        "faer naked builtin: {:.3}.",
        bench(faer_naked_matrix_builtin_multiplication)
    );
    println!(
        "faer naked explicit: {:.3}.",
        bench(faer_naked_matrix_explicit_multiplication)
    );
    println!(
        "ndarray naked builtin: {:.3}.",
        bench(ndarray_naked_matrix_builtin_multiplication)
    );
    println!(
        "ndarray naked explicit: {:.3}.",
        bench(ndarray_naked_matrix_explicit_multiplication)
    );
    println!(
        "ndarray measure explicit: {:.3}.",
        bench(ndarray_measure_matrix_explicit_multiplication)
    );
    println!(
        "array measure: {:.3}.",
        bench(array_measure_matrix_multiplication)
    );
    println!(
        "array approx measure: {:.3}.",
        bench(array_approx_measure_matrix_multiplication)
    );
    println!(
        "array approx measure values: {:.3}.",
        bench(array_approx_measure_values_matrix_multiplication)
    );
}
