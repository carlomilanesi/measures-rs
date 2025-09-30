// Build and run with (where 10000000 is the size of the data set; the default is 100):
//     cargo run --release --example bench 10000000

use rand::{rngs::StdRng, Rng, SeedableRng};

measures::define_measure_types! {
    with_points exact with_approx,
    []
}

measures::measurement_vector_property! { Length }

measures::measurement_unit! {
    name: Metre,
    property: Length,
    suffix: " m",
}

measures::measurement_scalar_property! { Time }

measures::measurement_unit! {
    name: NanoSecond,
    property: Time,
    ratio: 1e-9,
    suffix: " ns",
}

const N_ITERATIONS: usize = 1_000;

fn main() {
    let n_data_size = parse_command_line();
    println!("Using {n_data_size} items.");

    use_primitive_numbers(n_data_size);
    use_measures(n_data_size);
    use_approx_measures(n_data_size);
}

fn parse_command_line() -> usize {
    const MAX_DATA_SIZE: usize = 100_000_000; // items, using 2.4 GB
    const DEFAULT_DATA_SIZE: usize = 10_000; // items, using 240 KB
    let mut args = std::env::args();
    args.next();
    if let Some(arg) = args.next() {
        if let Ok(n) = arg.parse::<usize>() {
            if n > MAX_DATA_SIZE {
                println!("Warning: {arg} is too big as data size.");
                MAX_DATA_SIZE
            } else {
                n
            }
        } else {
            println!("Warning: {arg} is not allowed as data size.");
            DEFAULT_DATA_SIZE
        }
    } else {
        println!("No data size specified.");
        DEFAULT_DATA_SIZE
    }
}

fn use_primitive_numbers(n_data_size: usize) {
    let mut mp1 = Vec::<f64>::with_capacity(n_data_size);
    let mut mp2 = Vec::<f64>::with_capacity(n_data_size);
    let mut m1 = Vec::<f64>::with_capacity(n_data_size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    for _ in 0..n_data_size {
        mp1.push(rng.random::<f64>());
        mp2.push(rng.random::<f64>());
        m1.push(rng.random::<f64>());
    }

    let using_directly_f64 = || {
        for i in 0..n_data_size {
            let r1 = mp1[i] + m1[i] * 3.1 - mp2[i];
            let r2 = mp1[i] - (mp2[i] - 3.1 * m1[i]);
            let r = r1 - r2;
            if r.abs() > 1.0e-15 {
                println!("{r1} {r2} {r}");
            }
        }
    };
    using_directly_f64();
    let start = std::time::Instant::now();
    for _ in 0..N_ITERATIONS {
        using_directly_f64();
    }
    let duration = Measure::<NanoSecond, f64>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using directly f64: {}.",
        duration / (n_data_size * N_ITERATIONS) as f64
    );
}

fn use_measures(n_data_size: usize) {
    let mut mp1 = Vec::<MeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut mp2 = Vec::<MeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut m1 = Vec::<Measure<Metre>>::with_capacity(n_data_size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    for _ in 0..n_data_size {
        mp1.push(MeasurePoint::<Metre>::new(rng.random::<f64>()));
        mp2.push(MeasurePoint::<Metre>::new(rng.random::<f64>()));
        m1.push(Measure::<Metre>::new(rng.random::<f64>()));
    }

    let using_measure = || {
        for i in 0..n_data_size {
            let r1 = mp1[i] + m1[i] * 3.1 - mp2[i];
            let r2 = mp1[i] - (mp2[i] - 3.1 * m1[i]);
            let r = r1 - r2;
            if r.value.abs() > 1.0e-15 {
                println!("{r1} {r2} {r}");
            }
        }
    };
    using_measure();
    let start = std::time::Instant::now();
    for _ in 0..N_ITERATIONS {
        using_measure();
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using Measure<Metre, f64>: {}.",
        duration / (n_data_size * N_ITERATIONS) as f64
    );

    let using_measure_value = || {
        for i in 0..n_data_size {
            let r1 = mp1[i].value + m1[i].value * 3.1 - mp2[i].value;
            let r2 = mp1[i].value - (mp2[i].value - 3.1 * m1[i].value);
            let r = r1 - r2;
            if r.abs() > 1.0e-15 {
                println!("{r1} {r2} {r}");
            }
        }
    };
    using_measure_value();
    let start = std::time::Instant::now();
    for _ in 0..N_ITERATIONS {
        using_measure_value();
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using Measure<Metre, f64>.value: {}.",
        duration / (n_data_size * N_ITERATIONS) as f64
    );
}

fn use_approx_measures(n_data_size: usize) {
    let mut mp1 = Vec::<ApproxMeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut mp2 = Vec::<ApproxMeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut m1 = Vec::<ApproxMeasure<Metre>>::with_capacity(n_data_size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    const UNCERTAINTY: f64 = 0.001;
    for _ in 0..n_data_size {
        mp1.push(ApproxMeasurePoint::<Metre>::with_variance(
            rng.random::<f64>(),
            UNCERTAINTY * UNCERTAINTY,
        ));
        mp2.push(ApproxMeasurePoint::<Metre>::with_variance(
            rng.random::<f64>(),
            UNCERTAINTY * UNCERTAINTY,
        ));
        m1.push(ApproxMeasure::<Metre>::with_variance(
            rng.random::<f64>(),
            UNCERTAINTY * UNCERTAINTY,
        ));
    }

    let using_approx_measure = || {
        for i in 0..n_data_size {
            let r1 = mp1[i] + m1[i] * 3.1 - mp2[i];
            let r2 = mp1[i] - (mp2[i] - 3.1 * m1[i]);
            let r = r1 - r2;
            if r.value.abs() > 1.0e-15 {
                println!("{r1} {r2} {r}");
            }
        }
    };
    using_approx_measure();
    let start = std::time::Instant::now();
    for _ in 0..N_ITERATIONS {
        using_approx_measure();
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using ApproxMeasure<Metre, f64>: {}.",
        duration / (n_data_size * N_ITERATIONS) as f64
    );

    let using_approx_measure_value = || {
        for i in 0..n_data_size {
            let r1 = mp1[i].value + m1[i].value * 3.1 - mp2[i].value;
            let r2 = mp1[i].value - (mp2[i].value - 3.1 * m1[i].value);
            let r = r1 - r2;
            if r.abs() > 1.0e-15 {
                println!("{r1} {r2} {r}");
            }
        }
    };
    using_approx_measure_value();
    let start = std::time::Instant::now();
    for _ in 0..N_ITERATIONS {
        using_approx_measure_value();
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using ApproxMeasure<Metre, f64>.value: {}.",
        duration / (n_data_size * N_ITERATIONS) as f64
    );
}
