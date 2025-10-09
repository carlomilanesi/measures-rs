#[macro_export]
macro_rules! if_all_true {
    ( { } $( $fragment:item )* ) => { $( $fragment )* };
    ( { false $($rest:tt)* } $( $fragment:item )* ) => { };
    ( { true $($rest:tt)* } $( $fragment:item )* ) => { measures::if_all_true! { { $($rest)* } $( $fragment )* } };
}

#[macro_export]
macro_rules! define_scalar_property {
    ( $scalar_property:tt ) => {};
}

#[macro_export]
macro_rules! define_vector_property {
    // ( $vector_property:tt ) => {};
    ( $property_name:ident [ $( $unit:tt )* ]) => {};
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
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
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

        // `scalar_properties` section
        $(
            //println!(" - scalar property: {}", stringify!($scalar_prop));

            measures::measurement_scalar_property! { $scalar_prop }

            $(
                measures::measurement_unit! {
                    name: $scalar_unit,
                    property: $scalar_prop,
                    $(
                        $scalar_key: $scalar_val,
                    )*
                }
                //println!("   * unit: {}", stringify!($scalar_unit));
                //$(
                //    println!("     {} = {}", stringify!($scalar_key), $scalar_val);
                //)*
            )*
        )*

        // `vector_properties` section
        $(
            //println!(" - vector property: {}", stringify!($vector_prop));

            measures::measurement_vector_property! { $vector_prop }

            $(
                measures::measurement_unit! {
                    name: $vector_unit,
                    property: $vector_prop,
                    $(
                        $vector_key: $vector_val,
                    )*
                }
                //println!("   * unit: {}", stringify!($vector_unit));
                //$(
                //    println!("     {} = {}", stringify!($vector_key), $vector_val);
                //)*
            )*
        )*

        // `dimensionless_measurement_units` section
        $(
            //println!(" - dimensionless unit: {}", stringify!($dimless_unit));

            measures::measurement_unit! {
                name: $dimless_unit,
                property: Dimensionless,
                $(
                    $dimless_key: $dimless_val,
                )*
            }
            //println!("   * unit: {}", stringify!($dimless_unit));
            //$(
            //    println!("     {} = {}", stringify!($dimless_key), $dimless_val);
            //)*
        )*

        // `angle_measurement_units` section
        $(
            //println!(" - angle unit: {}", stringify!($angle_unit));

            measures::angle_measurement_unit! {
                name: $angle_unit,
                $(
                    $angle_key: $angle_val,
                )*
            }
            //println!("   * unit: {}", stringify!($angle_unit));
            //$(
            //    println!("     {} = {}", stringify!($angle_key), $angle_val);
            //)*
        )*

        // `relationships` section
        $(
            measures::define_units_relationship! {
                $exact $with_approx $with_correlation,
                $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
            }
        )*
    };

    // First option is "with_points"
    {
        with_points $( $flag:ident )*,

        with_points: false,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: true,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_directions"
    {
        with_directions $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: false,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: true,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_2d"
    {
        with_2d $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: false,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: true,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_3d"
    {
        with_3d $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: false,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: true,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_transformations"
    {
        with_transformations $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: false,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: true,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "exact"
    {
        exact $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: false,
        with_approx: $with_approx:tt,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: true,
            with_approx: $with_approx,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_approx"
    {
        with_approx $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: false,
        with_correlation: $with_correlation:tt;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,

            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: true,
            with_correlation: $with_correlation;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };

    // First option is "with_correlation"
    {
        with_correlation $( $flag:ident )*,

        with_points: $with_points:tt,
        with_directions: $with_directions:tt,
        with_2d: $with_2d:tt,
        with_3d: $with_3d:tt,
        with_transformations: $with_transformations:tt,
        exact: $exact:tt,
        with_approx: $with_approx:tt,
        with_correlation: false;

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    } => {
        measures::define_measure_types_aux!{
            $( $flag ) *,
            with_points: $with_points,
            with_directions: $with_directions,
            with_2d: $with_2d,
            with_3d: $with_3d,
            with_transformations: $with_transformations,
            exact: $exact,
            with_approx: $with_approx,
            with_correlation: true;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
            ]
        }
    };
}

#[macro_export]
macro_rules! define_measure_types {
    (
        $( $flag:ident )* ,

        scalar_properties [
            $(
                $scalar_prop:ident [
                    $(
                        $scalar_unit:ident {
                            $($scalar_key:ident : $scalar_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        vector_properties [
            $(
                $vector_prop:ident [
                    $(
                        $vector_unit:ident {
                            $($vector_key:ident : $vector_val:expr),* $(,)?
                        }
                    )*
                ]
            )*
        ]

        dimensionless_measurement_units [
            $(
                $dimless_unit:ident {
                    $($dimless_key:ident : $dimless_val:expr),* $(,)?
                }
            )*
        ]

        angle_measurement_units [
            $(
                $angle_unit:ident {
                    $($angle_key:ident : $angle_val:expr),* $(,)?
                }
            )*
        ]

        relationships [
            $(
                $unit1:ident $dim1:tt == $unit2:ident $dim2:tt $op:tt $unit3:ident $dim3:tt $(,)?
            )*
        ]
    ) => {
        measures::define_measure_types_aux! {
            $( $flag )* ,

            with_points: false,
            with_directions: false,
            with_2d: false,
            with_3d: false,
            with_transformations: false,
            exact: false,
            with_approx: false,
            with_correlation: false;

            scalar_properties [
                $(
                    $scalar_prop [
                        $(
                            $scalar_unit {
                                $($scalar_key : $scalar_val),*
                            }
                        )*
                    ]
                )*
            ]

            vector_properties [
                $(
                    $vector_prop [
                        $(
                            $vector_unit {
                                $($vector_key : $vector_val),*
                            }
                        )*
                    ]
                )*
            ]

            dimensionless_measurement_units [
                $(
                    $dimless_unit {
                        $($dimless_key : $dimless_val),*
                    }
                )*
            ]

            angle_measurement_units [
                $(
                    $angle_unit {
                        $($angle_key : $angle_val),*
                    }
                )*
            ]

            relationships [
                $(
                    $unit1 $dim1 == $unit2 $dim2 $op $unit3 $dim3
                )*
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
            suffix: $suffix,
            offset: 0.,
            cycle_fraction: $cycle_fraction,
        }
    };
    {
        name: $name:ident,
        suffix: $suffix:expr,
        offset: $offset:expr,
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
            suffix: $suffix,
            ratio: 1.,
        }
    };
    {
        name: $name:ident,
        property: $property:ident,
        suffix: $suffix:expr,
        ratio: $ratio:expr,
    } => {
        $crate::measurement_unit! {
            name: $name,
            property: $property,
            suffix: $suffix,
            ratio: $ratio,
            offset: 0.,
        }
    };
    {
        name: $name:ident,
        property: $property:ident,
        suffix: $suffix:expr,
        ratio: $ratio:expr,
        offset: $offset:expr,
    } => {
        pub struct $name;
        impl measures::traits::MeasurementUnit for $name {
            type Property = $property;
            const RATIO: f64 = $ratio;
            const OFFSET: f64 = $offset;
            const SUFFIX: &'static str = $suffix;
        }

        /// Measure<One> * Measure -> Measure
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
