#[macro_export]
macro_rules! if_all_true {
    ( { } $( $fragment:item )* ) => { $( $fragment )* };
    ( { false $($rest:tt)* } $( $fragment:item )* ) => { };
    ( { true  $($rest:tt)* } $( $fragment:item )* ) => { measures::if_all_true! { { $($rest)* } $( $fragment )* } };
}

#[macro_export]
macro_rules! define_measure_types_aux {
    // Empty case
    {
        ,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
        use measures::{
            dimensionless::{Dimensionless, One},
            angle::{Angle, Radian},
            traits::{
                AngleMeasurementUnit, ArithmeticOps, LossyFrom, MeasurementUnit, Sqrt, VectorProperty, Trigonometry,
            },
        };
        use std::fmt;
        use std::marker::PhantomData;

        measures::inner_define_measure! { $with_approx }
        measures::if_all_true! { { $with_approx }
            measures::inner_define_approx_measure! { $exact }
        }
        measures::if_all_true! { { $with_points }
            measures::inner_define_measure_point! { $with_approx }
        }
        measures::if_all_true! { { $with_approx $with_points }
            measures::inner_define_approx_measure_point ! { $exact }
        }
        measures::if_all_true! { { $exact $with_directions }
            measures::inner_define_unsigned_direction! { $with_points }
        }
        measures::if_all_true! { { $exact $with_directions }
            measures::inner_define_signed_direction! { $with_points }
        }
        measures::if_all_true! { { $with_2d }
            measures::inner_define_measure_2d! { $with_points $with_directions $with_approx }
        }
        measures::if_all_true! { { $with_2d $with_points $exact }
            measures::inner_define_measure_point_2d! { $with_approx }
        }
        measures::if_all_true! { { $with_3d }
            measures::inner_define_measure_3d! { $with_approx }
        }
        measures::if_all_true! { { $with_approx $with_3d }
            measures::inner_define_approx_measure_3d! { $exact }
        }
        measures::if_all_true! { { $with_points $with_3d }
            measures::inner_define_measure_point_3d! { $with_approx }
        }
        measures::if_all_true! { { $with_approx $with_points $with_3d }
            measures::inner_define_approx_measure_point_3d! { $exact }
        }
        measures::if_all_true! { { $with_2d $with_transformations }
            measures::inner_define_linear_map_2d! {}
        }
        measures::if_all_true! { { $with_3d $with_transformations }
            measures::inner_define_linear_map_3d! {}
        }
        measures::if_all_true! { {$with_2d $with_transformations $with_points}
            measures::inner_define_affine_map_2d! {}
        }
        measures::if_all_true! { {$with_3d $with_transformations $with_points}
            measures::inner_define_affine_map_3d! {}
        }
        $(
            measures::define_units_relationship! {
                $exact $with_approx $with_correlation,
                $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
            }
        )*
    };

    // First option is "with_points"
    {
        with_points $( $flag:ident ) *,
        with_points: false,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: true,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_points $( $flag:ident ) *,
        with_points: true,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_points` specified several times");
    };

    // First option is "with_directions"
    {
        with_directions $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: false,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: true,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_directions $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: true,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_directions` specified several times");
    };

    // First option is "with_2d"
    {
        with_2d $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: false,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: true,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_2d $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: true,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_2d` specified several times");
    };

    // First option is "with_3d"
    {
        with_3d $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: false,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: true,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_3d $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: true,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_3d` specified several times");
    };

    // First option is "with_transformations"
    {
        with_transformations $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: false,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: true,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_transformations $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: true,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_transformations` specified several times");
    };

    // First option is "exact"
    {
        exact $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: false,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: true,
        with_approx: $with_approx,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        exact $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: true,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `exact` specified several times");
    };

    // First option is "with_approx"
    {
        with_approx $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: false,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: true,
        with_correlation: $with_correlation,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_approx $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: true,
        with_correlation: $with_correlation:tt,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_approx` specified several times");
    };

    // First option is "with_correlation"
    {
        with_correlation $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: false,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => { measures::define_measure_types_aux!{
        $( $flag ) *,
        with_points: $with_points,
        with_directions: $with_directions,
        with_2d: $with_2d,
        with_3d: $with_3d,
        with_transformations: $with_transformations,
        exact: $exact,
        with_approx: $with_approx,
        with_correlation: true,
        [
            $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 ,)*
        ]
    }};
    {
        with_correlation $( $flag:ident ) *,
        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: true,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        compile_error!("Feature `with_correlation` specified several times");
    };
}

#[macro_export]
macro_rules! define_measure_types {
    {
        $( $flag:ident )* ,
        [
            $( $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt ,)*
        ]
    } => {
        measures::define_measure_types_aux! {
            $( $flag ) * ,
            with_points: false,
            with_directions: false,
            with_2d: false,
            with_3d: false,
            with_transformations: false,
            exact: false,
            with_approx: false,
            with_correlation: false,
            [
                $( $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3 , )*
            ]
        }
    };
}

#[macro_export]
macro_rules! measurement_scalar_property {
    ($name:ident) => {
        #[allow(dead_code)]
        pub struct $name;
        impl $crate::traits::MeasurementProperty for $name {}
        impl $crate::traits::ScalarProperty for $name {}
    };
}

#[macro_export]
macro_rules! measurement_vector_property {
    ($name:ident) => {
        #[allow(dead_code)]
        pub struct $name;
        impl $crate::traits::MeasurementProperty for $name {}
        impl $crate::traits::VectorProperty for $name {}
    };
}

#[macro_export]
macro_rules! angle_measurement_unit {
    {
        name: $name:ident,
        suffix: $suffix:expr,
        cycle_fraction: $cycle_fraction:expr,
    } => {
        $crate::angle_measurement_unit! {
            name: $name,
            offset: 0.,
            suffix: $suffix,
            cycle_fraction: $cycle_fraction,
        }
    };
    {
        name: $name:ident,
        offset: $offset:expr,
        suffix: $suffix:expr,
        cycle_fraction: $cycle_fraction:expr,
    } => {
        pub struct $name;
        impl measures::traits::MeasurementUnit for $name {
            type Property = $crate::angle::Angle;
            const RATIO: f64 = core::f64::consts::TAU / ($cycle_fraction);
            const OFFSET: f64 = $offset;
            const SUFFIX: &'static str = $suffix;
        }

        impl measures::traits::AngleMeasurementUnit for $name {
            const CYCLE_FRACTION: f64 = $cycle_fraction;
        }

        impl<Number> std::ops::Mul<Measure<$name, Number>> for Measure<measures::dimensionless::One, Number>
        where
            Number: measures::traits::ArithmeticOps,
        {
            type Output = Measure<$name, Number>;
            fn mul(self, other: Measure<$name, Number>) -> Measure<$name, Number> {
                Measure::<$name, Number>::new(self.value * other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! measurement_unit {
    {
        name: $name:ident,
        property: $property:ident,
        suffix: $suffix:expr,
    } => {
        $crate::measurement_unit! {
            name: $name,
            property: $property,
            ratio: 1.,
            offset: 0.,
            suffix: $suffix,
        }
    };
    {
        name: $name:ident,
        property: $property:ident,
        ratio: $ratio:expr,
        suffix: $suffix:expr,
    } => {
        $crate::measurement_unit! {
            name: $name,
            property: $property,
            ratio: $ratio,
            offset: 0.,
            suffix: $suffix,
        }
    };
    {
        name: $name:ident,
        property: $property:ident,
        ratio: $ratio:expr,
        offset: $offset:expr,
        suffix: $suffix:expr,
    } => {
        pub struct $name;
        impl measures::traits::MeasurementUnit for $name {
            type Property = $property;
            const RATIO: f64 = $ratio;
            const OFFSET: f64 = $offset;
            const SUFFIX: &'static str = $suffix;
        }

        impl<Number> std::ops::Mul<Measure<$name, Number>> for Measure<measures::dimensionless::One, Number>
        where
            Number: measures::traits::ArithmeticOps,
        {
            type Output = Measure<$name, Number>;
            fn mul(self, other: Measure<$name, Number>) -> Measure<$name, Number> {
                Measure::<$name, Number>::new(self.value * other.value)
            }
        }
    };
}
