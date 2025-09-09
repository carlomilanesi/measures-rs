#[macro_export]
macro_rules! if_all_true {
    ( { } $( $fragment:item )* ) => { $( $fragment )* };
    ( { false $($rest:tt)* } $( $fragment:item )* ) => { };
    ( { true  $($rest:tt)* } $( $fragment:item )* ) => { rs_measures::if_all_true! { { $($rest)* } $( $fragment )* } };
}

#[macro_export]
macro_rules! define_measure_types {
    {
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
        use rs_measures::{
            dimensionless::{Dimensionless, One},
            angle::{Angle, Radian},
            traits::{
                AngleMeasurementUnit, ArithmeticOps, LossyFrom, MeasurementUnit, Sqrt, VectorProperty,Trigonometry,
            },
        };
        use std::fmt;
        use std::marker::PhantomData;

        rs_measures::inner_define_measure! { $with_approx }
        rs_measures::if_all_true! { { $with_approx }
            rs_measures::inner_define_approx_measure! { $exact }
        }
        rs_measures::if_all_true! { { $with_points }
            rs_measures::inner_define_measure_point! { $with_approx }
        }
        rs_measures::if_all_true! { { $with_approx $with_points }
            rs_measures::inner_define_approx_measure_point ! { $exact }
        }
        rs_measures::if_all_true! { { $exact $with_directions }
            rs_measures::inner_define_unsigned_direction! { $with_points }
        }
        rs_measures::if_all_true! { { $exact $with_directions }
            rs_measures::inner_define_signed_direction! { $with_points }
        }
        rs_measures::if_all_true! { { $with_2d }
            rs_measures::inner_define_measure_2d! { $with_points $with_directions }
        }
        rs_measures::if_all_true! { { $with_2d $with_points $exact }
            rs_measures::inner_define_measure_point_2d! { $with_approx }
        }

        rs_measures::if_all_true! { { $with_3d }
            rs_measures::inner_define_measure_3d! { $with_approx }
        }
        rs_measures::if_all_true! { { $with_approx $with_3d }
            rs_measures::inner_define_approx_measure_3d! { $exact }
        }
        rs_measures::if_all_true! { { $with_points $with_3d }
            rs_measures::inner_define_measure_point_3d! { $with_approx }
        }
        rs_measures::if_all_true! { { $with_approx $with_points $with_3d }
            rs_measures::inner_define_approx_measure_point_3d! { $exact }
        }
        rs_measures::if_all_true! { { $with_2d $with_transformations }
            rs_measures::inner_define_linear_map_2d! {}
        }
        rs_measures::if_all_true! { { $with_3d $with_transformations }
            rs_measures::inner_define_linear_map_3d! {}
        }
        rs_measures::if_all_true! { {$with_2d $with_transformations $with_points}
            rs_measures::inner_define_affine_map_2d! {}
        }
        rs_measures::if_all_true! { {$with_3d $with_transformations $with_points}
            rs_measures::inner_define_affine_map_3d! {}
        }
        $(
            rs_measures::define_units_relationship! {
                $exact $with_approx,
                $unit1 $dim1 $unit2 $dim2 $op $unit3 $dim3
            }
        )*
    };
}
