// To run this, type:
// cargo run --release --example matrix-mul

// This program performs a matrix multiplication of two matrices of size 192x192
// using different libraries and approaches, measuring the time taken for each
// method. It compares naked floating-point operations, as well as operations
// using the crates `measures`, `nalgebra`, `faer`, `ndarray`.

// In particular, it compares:
// - Arrays of f64 with explicit loops
// - Naked arrays with `nalgebra` built-in multiplication
// - Naked arrays with `nalgebra` explicit loops
// - Naked arrays with `faer` built-in multiplication
// - Naked arrays with `faer` explicit loops
// - Naked arrays with `ndarray` built-in multiplication
// - Naked arrays with `ndarray` explicit loops
// - Measure arrays with `ndarray` explicit loops
// - Measure arrays with explicit loops
// - ApproxMeasure arrays with explicit loops
// - ApproxMeasure arrays with explicit loops, only using the values

// For each of these cases, two 192x192 matrixes are filled and then multiplied for 21 times.
// The time taken for the 20 multiplications after the first one (used as warm-up) is averaged and reported.

measures::define_measure_types! {
    exact with_approx,
    scalar_properties [
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
        Force [
            Newton {
                suffix: " N",
            }
        ]
    ]
    angle_measurement_units [
        Degree {
            suffix: " deg",
            cycle_fraction: 360.,
        }
    ]
    relationships [
        Joule 1 == Newton 1 * Metre 1,
    ]
}

fn elapsed<const SIZE: usize>(start: Instant, x: f64) -> Duration {
    if SIZE == 4 && ((x - 1005.8783999999987) / x).abs() > 1e-14 {
        eprint!("Error");
        return Duration::ZERO;
    }
    if SIZE == 1192 && ((x - 7060895259.18578) / x).abs() > 1e-14 {
        eprint!("Error");
        return Duration::ZERO;
    }
    start.elapsed()
}

#[allow(clippy::needless_range_loop)]
fn native_f64s<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[SIZE / 3][SIZE / 2])
}

#[allow(clippy::needless_range_loop)]
fn native_f64_measures<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[SIZE / 3][SIZE / 2].value)
}

#[allow(clippy::needless_range_loop)]
fn native_f64_approx_measures<const SIZE: usize>() -> Duration {
    let mut mat1 = [[ApproxMeasure::<Newton>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat2 = [[ApproxMeasure::<Metre>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat3 = [[ApproxMeasure::<Joule>::with_variance(0., 0.); SIZE]; SIZE];
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[row][column] += ApproxMeasure::<Newton>::with_variance(x, 1e-8);
            x += 1.7;
            mat2[row][column] += ApproxMeasure::<Metre>::with_variance(x, 1e-8);
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
    elapsed::<SIZE>(start, mat3[SIZE / 3][SIZE / 2].value)
}

#[allow(clippy::needless_range_loop)]
fn native_f64_approx_measure_values<const SIZE: usize>() -> Duration {
    let mut mat1 = [[ApproxMeasure::<Newton>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat2 = [[ApproxMeasure::<Metre>::with_variance(0., 1e-8); SIZE]; SIZE];
    let mut mat3 = [[ApproxMeasure::<Joule>::with_variance(0., 0.); SIZE]; SIZE];
    let mut x = 12.34;
    for row in 0..SIZE {
        for column in 0..SIZE {
            mat1[row][column] += ApproxMeasure::<Newton>::with_variance(x, 1e-8);
            x += 1.7;
            mat2[row][column] += ApproxMeasure::<Metre>::with_variance(x, 1e-8);
            x -= 1.3;
        }
    }

    let start = Instant::now();
    for row in 0..SIZE {
        for column in 0..SIZE {
            for step in 0..SIZE {
                mat3[row][column].value += mat1[row][step].value * mat2[step][column].value;
            }
        }
    }
    elapsed::<SIZE>(start, mat3[SIZE / 3][SIZE / 2].value)
}

extern crate nalgebra as na;
use std::time::{Duration, Instant};

use na::{ArrayStorage, Const, Matrix};

fn nalgebra_f64s_built_in<const SIZE: usize>() -> Duration {
    type Mat<const SIZE: usize> =
        Matrix<f64, Const<SIZE>, Const<SIZE>, ArrayStorage<f64, SIZE, SIZE>>;
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
    let mat3: Mat<SIZE> = mat1 * mat2;
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
}

fn nalgebra_f64s_item_wise<const SIZE: usize>() -> Duration {
    type Mat<const SIZE: usize> =
        Matrix<f64, Const<SIZE>, Const<SIZE>, ArrayStorage<f64, SIZE, SIZE>>;
    let mut mat1 = Mat::<SIZE>::zeros();
    let mut mat2 = Mat::<SIZE>::zeros();
    let mut mat3 = Mat::<SIZE>::zeros();
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
}

fn nalgebra_f64_measures_item_wise<const SIZE: usize>() -> Duration {
    type MatN<const SIZE: usize> = Matrix<
        Measure<Newton>,
        Const<SIZE>,
        Const<SIZE>,
        ArrayStorage<Measure<Newton>, SIZE, SIZE>,
    >;
    type MatM<const SIZE: usize> =
        Matrix<Measure<Metre>, Const<SIZE>, Const<SIZE>, ArrayStorage<Measure<Metre>, SIZE, SIZE>>;
    type MatJ<const SIZE: usize> =
        Matrix<Measure<Joule>, Const<SIZE>, Const<SIZE>, ArrayStorage<Measure<Joule>, SIZE, SIZE>>;
    let mut mat1 = MatN::<SIZE>::zeros();
    let mut mat2 = MatM::<SIZE>::zeros();
    let mut mat3 = MatJ::<SIZE>::zeros();

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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)].value)
}

use faer::Mat;

fn faer_f64s_built_in<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
}

fn faer_f64s_item_wise<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
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

fn ndarray_f64s_built_in<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
}

fn ndarray_f64s_item_wise<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)])
}

fn ndarray_f64_measures_item_wise<const SIZE: usize>() -> Duration {
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
    elapsed::<SIZE>(start, mat3[(SIZE / 3, SIZE / 2)].value)
}

fn bench<const SIZE: usize>(f: fn() -> Duration) -> f64 {
    f(); // warm up
    let mut t = Duration::ZERO;
    for _ in 0..20_000_000 / SIZE / SIZE {
        t += f();
    }
    t.as_nanos() as f64 / 1e7
}

fn main() {
    bench_matrices::<4>();
    bench_matrices::<192>();
}

fn bench_matrices<const SIZE: usize>() {
    println!();
    println!("**** Benchmark of matrices of side {SIZE} ****");
    println!("Native matrices of:");
    println!("    f64s: {:.3}.", bench::<SIZE>(native_f64s::<SIZE>));
    println!(
        "    f64 measures: {:.3}.",
        bench::<SIZE>(native_f64_measures::<SIZE>)
    );
    println!(
        "    f64 approx measures: {:.3}.",
        bench::<SIZE>(native_f64_approx_measures::<SIZE>)
    );
    println!(
        "    f64 approx measure values: {:.3}.",
        bench::<SIZE>(native_f64_approx_measure_values::<SIZE>)
    );

    println!("Nalgebra matrices of:");
    println!(
        "    f64s, built-in multiplication: {:.3}.",
        bench::<SIZE>(nalgebra_f64s_built_in::<SIZE>)
    );
    println!(
        "    f64s, item-wise multiplication: {:.3}.",
        bench::<SIZE>(nalgebra_f64s_item_wise::<SIZE>)
    );
    println!(
        "    f64 measures, item-wise multiplication: {:.3}.",
        bench::<SIZE>(nalgebra_f64_measures_item_wise::<SIZE>)
    );

    println!("Faer matrices of:");
    println!(
        "    f64s, built-in multiplication: {:.3}.",
        bench::<SIZE>(faer_f64s_built_in::<SIZE>)
    );
    println!(
        "    f64s, item-wise multiplication: {:.3}.",
        bench::<SIZE>(faer_f64s_item_wise::<SIZE>)
    );

    println!("Ndarray matrices of:");
    println!(
        "    f64s, built-in multiplication: {:.3}.",
        bench::<SIZE>(ndarray_f64s_built_in::<SIZE>)
    );
    println!(
        "    f64s, item-wise multiplication: {:.3}.",
        bench::<SIZE>(ndarray_f64s_item_wise::<SIZE>)
    );
    println!(
        "    f64 measures, item-wise multiplication: {:.3}.",
        bench::<SIZE>(ndarray_f64_measures_item_wise::<SIZE>)
    );
}
