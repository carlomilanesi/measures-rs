use units::{Measure, U1};

mod units {
    measures::define_measure_types! {
        exact,
        vector_properties [
            P1 [
                U1 { suffix: " u1" }
            ]
        ]
    }
}

fn main() {
    let _m1 = Measure::<U1, i32>::new(1.);
}
