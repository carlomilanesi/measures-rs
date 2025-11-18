use units::{Measure, U1, U2};

mod units {
    measures::define_measure_types! {
        exact,
        vector_properties [
            P1 [
                U1 { suffix: " u1" }
                U2 { suffix: " u2" }
            ]
        ]
    }
}

fn main() {
    let m1 = Measure::<U1>::new(1.);
    let mut m2 = Measure::<U2>::new(2.);
    m2 = m1;
}
