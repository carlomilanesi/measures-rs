use units::{Measure, U1};

mod units {
    measures::define_measure_types! {
        exact,
        vector_properties [
            P1 [ U1 { suffix: " u1" } ]
        ]
    }
}

fn main() {
    let m1 = Measure::<U1, f32>::new(1.);
    let m2 = Measure::<U1, f64>::new(2.);
    _ = m1 + m2;
}
