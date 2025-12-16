# Tutorial for the library `measures`

This documents explains how to use, in Rust software development projects, the library (or _crate_) `measures`, contained in the package `measures-rs`.
The document [Motivation](Motivation.md) explains which are the advantages of using this library, with respect to using other libraries or to using only primitive numbers.

## Creating a tutorial project

Let's create a Rust project by using these commands:
```
cargo new measures-tut
cd measures-tut
cargo add measures-rs
```

Now we need to define the units of measurement we want to use.
For simplicity, let's import a large collection of definitions of units of measurement.

To do that, open the Web page https://github.com/carlomilanesi/measures-rs/blob/main/examples/units/mod.rs.
Copy all its contents into the clipboard.
Create a file named `src/units.rs`, and paste the copied contents into it.
Save it.

## Creating a measure variable

Edit the file `src/main.rs`, so that it has this contents:
```rust
mod units;
use units::*;
fn main() {
    let distance = Measure::<Kilometre>::new(100.);
    println!("The distance is {distance}.");
}
```

Run your project.
It should print: `The distance is 100 km`.

Notice that the three characters "` km`" are not part of your code.
They are printed because of the type of the variable `distance`.
The type of such a variable is `Measure::<Kilometre>`, meaning _"a measure whose unit of measurement is kilometres (or kilometers)"_.

Such a variable encapsulates an `f64` number, whose value is `100`.

The unit of measurement is not stored in memory.
It is just in the type of the variable.

You can access the value of a measure by adding these two statements at the end of your function `main`:
```rust
    let distance_value: f64 = distance.value;
    println!("The distance is {distance_value}.");
```

Now the program will print:
```
The distance is 100 km.
The distance is 100.
```

The second printed line has no unit, because a primitive number is printed there.

This code is valid with `distance_value` having type `f64`, because `f64` is the default value type of the generic type `Measure`.
The statement `let distance_value: f32 = distance.value;` would have been illegal.

Though, you can also use `f32` as value type, as long as you specify it explicitly, like in the following statement:
```rust
    let _distance: f32 = Measure::<Kilometre, f32>::new(100.).value;
```

Currently, the library supports only the value types `f32` and `f64`.

From here on, to run the example code, just put it as the body of the function `main`.

## `Clone` and `Copy`

Measures are just wrappers of numbers that implement the traits `Clone` and `Copy`.
Therefore, such traits have been implemented also for `Measures`, as this code demonstrates:
```rust
    let d1 = Measure::<Metre>::new(100.);
    let _d2 = d1;
    let _d3 = d1;
    let _d4 = d1.clone();
```

## Numeric conversions

You can convert the value type of a measure, by using the standard `from` and `into` methods, like in these statements:
```rust
    let distance32 = Measure::<Metre, f32>::new(100.);
    let distance64 = Measure::<Metre, f64>::new(100.);

    _ = Measure::<_, f32>::from(distance32); // From f32 to f32, just a copy
    _ = Measure::<_, f64>::from(distance32); // From f32 to f64, numeric conversion with no information loss
    // _ = Measure::<_, f32>::from(distance64); // From f64 to f32, not allowed
    _ = Measure::<_, f64>::from(distance64); // From f64 to f64, just a copy

    let _: Measure<_, f32> = distance32.into(); // From f32 to f32, just a copy
    let _: Measure<_, f64> = distance32.into(); // From f32 to f64, numeric conversion with
    // let _: Measure<_, f32> = distance64.into(); // From f64 to f32, not allowed
    let _: Measure<_, f64> = distance64.into(); // From f64 to f64, just a copy
```

The code above demonstrates that you can convert a measure using an `f32` value to a measure using the same unit but an `f64` version.

You can also trivially convert a measure to an identical measure, having the same unit, the same numeric type and the same numeric value.

But you cannot use such functions to convert a measure which uses an `f64` value to a measure which uses the same unit but an `f32` version, because such conversion could lose significant digits, or be even impossible.

To perform numeric conversions of measures even when information could be lost, there is the method `lossy_into`:
```rust
    let distance32 = Measure::<Metre, f32>::new(100.);
    let distance64 = Measure::<Metre, f64>::new(100.);

    _ = distance32.lossy_into::<f32>(); // From f32 to f32, just a copy
    _ = distance32.lossy_into::<f64>(); // From f32 to f64, numeric conversion with no information loss
    _ = distance64.lossy_into::<f32>(); // From f32 to f64, numeric conversion with possible information loss
    _ = distance64.lossy_into::<f64>(); // From f64 to f64, just a copy
```

At last, the lossy conversions performed by the functions `from` and `into` shown above are a bit verbose.
So, there is also the equivalent shorter method `lossless_into`:
```rust
    let distance32 = Measure::<Metre, f32>::new(100.);
    let distance64 = Measure::<Metre, f64>::new(100.);

    _ = distance32.lossless_into::<f32>(); // From f32 to f32, just a copy
    _ = distance32.lossless_into::<f64>(); // From f32 to f64, numeric conversion with no information loss
    // _ = distance64.lossless_into::<f32>(); // From f64 to f32, not allowed
    _ = distance64.lossless_into::<f64>(); // From f64 to f64, just a copy
```

## Printing measure objects

We have already seen that objects of type `Measure` can be printed.
This means that such a type implements the trait `Display`, and it has the method `to_string()`.

Printing a measure means printing its value followed by a suffix depending on its unit of measurement.
For the unit `Kilometer`, the suffix is `" km"`, with a leading space.

Measures implement also the trait `Debug`.
So, you can write:
```rust
    #[allow(dead_code)]
    #[derive(Debug)]
    struct TimeElapsed {
        hours: Measure<Hour>,
        minutes: Measure<Minute>,
    }
    let time_elapsed = TimeElapsed {
        hours: Measure::<Hour>::new(3.),
        minutes: Measure::<Minute>::new(17.),
    };
    println!("{time_elapsed:?}");
```

It will print: `TimeElapsed { hours: 3 h, minutes: 17 min }`.

Also formatting options are supported.
For example, the statement `print!("{:.2}", Measure::<Minute>::new(17.138276));` will print `17.14 min`.

## Arithmetic operations on measures

All the common arithmetic operations can be applied to the objects of type `Measure`, provided they have the same unit of measurement and the same value type of the other operand.
Here are some examples.

You can negate a measure:
```rust
    let length1 = Measure::<Metre>::new(7.);
    let length2 = -length1;
    assert_eq!(length2.value, -7.);
```

You can add or subtract two measures:
```rust
    let length1 = Measure::<Metre>::new(7.);
    let length2 = Measure::<Metre>::new(4.);
    let length3 = length1 + length2;
    assert_eq!(length3.value, 11.);
    let length3 = length1 - length2;
    assert_eq!(length3.value, 3.);
```

You can increment or decrement a mutable measures by another measure:
```rust
    let mut length1 = Measure::<Metre>::new(7.);
    let length2 = Measure::<Metre>::new(4.);
    length1 += length2;
    assert_eq!(length1.value, 11.);
    length1 -= Measure::<Metre>::new(9.);
    assert_eq!(length1.value, 2.);
```

You can multiply or divide a measure by a number:
```rust
    let length1 = Measure::<Metre>::new(15.);
    assert_eq!((length1 * 3.).value, 45.); // Multiplication of a measure by a number
    assert_eq!((3. * length1).value, 45.); // Multiplication of a number by a measure
    assert_eq!((length1 / 3.).value, 5.); // Division of a measure by a number
```

You can upscale or downscale a mutable measures by a number:
```rust
    let mut length1 = Measure::<Metre>::new(6.);
    length1 *= 3.;
    assert_eq!(length1.value, 18.);
    length1 /= 2.;
    assert_eq!(length1.value, 9.);
```

You can also compute the division between two measures of the same unit, obtaining a dimensionless number, which is represented by a measure having, as unit of measurement, the built-in unit `One`, of the built-in property `Dimensionless`.

Any measure can be multiplied or divided by such dimensionless measures, obtaining a Measure of the same original unit of measurement:
```rust
    let mut length1 = Measure::<Metre>::new(7.);
    let mut length2 = Measure::<Metre>::new(5.);
    let ratio: Measure<measures::dimensionless::One> = length1 / length2; // 7. / 5. -> 1.4
    assert_eq!(ratio.value, 1.4);
    let length3: Measure<Metre> = ratio * length2; // 1.4 * 5. -> 7.
    assert_eq!(length3.value, 7.);
    let length4: Measure<Metre> = length2 * ratio; // 5. * 1.4 -> 7.
    assert_eq!(length4.value, 7.);
    let length5: Measure<Metre> = length1 / ratio; // 7. / 1.4 -> 5.
    assert_eq!(length5.value, 5.);
    length2 *= ratio; // 5. * 1.4 -> 7.
    assert_eq!(length2.value, 7.);
    length1 /= ratio; // 7. / 1.4 -> 5.
    assert_eq!(length1.value, 5.);
```

## Comparisons between measures

Measures can also be compared, using the comparison operators, because they implement the traits `PartialEq` and `PartialOrd`:
```rust
    let length1 = Measure::<Metre>::new(5.);
    let length2 = Measure::<Metre>::new(7.);
    assert!(length1 == length1);
    assert!(length1 != length2);
    assert!(length1 < length2);
    assert!(length1 <= length2);
    assert!(length2 > length1);
    assert!(length2 >= length1);
```

For the type `Measure`, the following methods have also been implemented:
* `min`: It returns the lesser of two measures.
* `max`: It returns the greater of two measures.
* `clamp`: It forces the receiver to be between the specified bounds, without requiring that the bounds are ordered.

Here is an example:
```rust
    let m1 = Measure::<Celsius>::new(17.);
    let m2 = Measure::<Celsius>::new(27.);
    let m3 = Measure::<Celsius>::new(37.);
    println!("{}, {};", m1.min(m2), m1.max(m2));
    println!("{}, {};", m1.clamp(m2, m3), m1.clamp(m3, m2));
    println!("{}, {};", m2.clamp(m1, m3), m2.clamp(m3, m1));
    println!("{}, {};", m3.clamp(m1, m2), m3.clamp(m2, m1));
```

It will print:
```text
17 °C, 27 °C;
27 °C, 27 °C;
27 °C, 27 °C;
27 °C, 27 °C;
```

## Conversions between units

Conversions between units of measurement are one of the main advantages of encapsulating measures, because of the added simplicity and safety.

For example, you can convert between units of measurement of the same physical or geometrical property, using the following code:
```rust
    let distance1 = Measure::<Kilometre>::new(100.);
    let distance2 = distance1.convert::<Mile>();
    let time1 = Measure::<Hour>::new(2.);
    let time2 = time1.convert::<Minute>();
    println!("The distances [{distance1}] and [{distance2}] are equivalent.");
    println!("The time spans [{time1}] and [{time2}] are equivalent.");
```

It will print:
```
The distances [100 km] and [62.13711922373339 mi] are equivalent.
The time spans [2 h] and [120 min] are equivalent.
```

Of course, it makes no sense to convert between units of measurement representing different physical or geometrical properties, and so such operations cause compilation errors:
```rust
    _ = Measure::<Kilometre>::new(100.).convert::<Hour>(); // ILLEGAL
    _ = Measure::<Mile>::new(100.).convert::<SquareMile>(); // ILLEGAL
```

For the first of the above statements, the compilation will emit the error message:
```
type mismatch resolving `<Hour as MeasurementUnit>::Property == Length`
```
because the property of `Kilometre` is `Length`, but the property of `Hour` is `Time`.

## The `default` method

The type `Measure` implements the `Default` trait, and therefore, instead of the expression `Measure::<Metre>::new(0.)`, you can write `Measure::<Metre>::default()`.

A good reason to stick to the method `new` is that it is a `const` function, and so you can compile this statement:
```rust
const ZERO_LENGTH: Measure<Metre> = Measure::<Metre>::new(0.);
```

Instead, the following statement is illegal:
```rust
const ZERO_LENGTH: Measure<Metre> = Measure::<Metre>::default();
```

## Norms

Given a real number, we can compute its "absolute value", a.k.a. "modulus" or "norm".
Such terms have different meanings in other mathematical spaces, but, when speaking of real numbers, they are equivalent.

So, we could need the absolute value of a `Measure` object.
For numbers, the standard library provides the method `abs`.
Instead, the type `Measure` has the method `norm`, which returns another `Measure`, like this example demonstrates:
```rust
    let m1 = Measure::<Metre>::new(5.);
    let m2 = Measure::<Metre>::new(-5.);
    println!("{}, {};", m1.norm(), m2.norm());
```

It will print `5 m, 5 m;`.

Other available method are `squared_norm`, which returns the squared value of the measure, as a number, and `normalized`, which returns a `Measure` having norm 1, and the same sign of the original measure:
```rust
    let m1 = Measure::<Metre>::new(5.);
    let m2 = Measure::<Metre>::new(-5.);
    let m3 = Measure::<Metre>::new(0.);
    let m4 = Measure::<Metre>::new(-0.);
    println!("{}, {}, {}, {};", m1.normalized(), m2.normalized(), m3.normalized(), m4.normalized());
```

It will print `1 m, -1 m, 1 m, -1 m;`.

## Absolute values

Does it make sense to sum the temperature at which water freezes to the temperature at which water boils? No, it doesn't.

Does it make sense to multiply by 2 the date of today? No, it doesn't.

For some kinds of measures, to add two values of the same type or to multiply a value by a number do not make sense.

This happens because they are absolute measures, not relative measures, like the ones shown in the previous sections.

Mathematically speaking, such absolute measures are *points* of an *affine space*, while the relative measures, like lengths, temperature variations, and time durations are *vectors* of a *vector space*.

To avoid using points instead of vectors or vice versa, the generic type `MeasurePoint` should be used, instead of using the generic type `Measure`, like demonstrated by the following code:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let variation = Measure::<Celsius>::new(2.);
    let point2 = point1 + variation;
    println!("When increasing temperature [{point1}] by [{variation}], we reach temperature [{point2}].");
```

It will print:
```
When increasing temperature [at 10 °C] by [2 °C], we reach temperature [at 12 °C].
```

When a measure point is printed, the three characters "`at `" are prefixed to it.

With absolute values, the following operations are forbidden:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    _ = point1 + point2; // ILLEGAL: you cannot sum two points
    _ = point1 * 3.; // ILLEGAL: you cannot multiply a point by a number
```

On the other hand, some operations are specific to absolute values.
The difference between two points represents the vector from the second point to the first one:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    let variation = point1 - point2; // Variation from point2 to point1.
    assert_eq!(variation.value, 8.);
```

We can also compute the average between two points.
The simplest way is by calling the function `midpoint`:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    assert_eq!(midpoint(point1, point2).value, 6.);
```

The function `midpoint` assumes that the two points have the same weight.
In case the points have different weight, you can call the function `weighted_midpoint`:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    assert_eq!(weighted_midpoint(point1, point2, 0.25).value, 4.);
```

The third argument of this function is the weight of the first point, normalized to one.
In this example, it is `0.25`, or 25%, meaning that the second point has weight 75%.

Calling `midpoint` is equivalent to calling `weighted_midpoint` with a weight of `0.5`.

Having a slice of several points, each with its own weight, the weighted average of all of them, named *barycentric combination*, can be computed.
To make sense, the sum of all the weights must be 1.
Here is an example, with three points:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    let point3 = MeasurePoint::<Celsius>::new(-6.);
    let center: MeasurePoint<Celsius> =
        barycentric_combination(&[point1, point2, point3], &[0.1, 0.4, 0.5]);
    assert_eq!(center.value, -1.2);
```

## Angles

With the library `measures`, you can also handle angles.

Consider a screw mechanism, that can be turned by several cycles (or revolutions).
```rust
    let mut position = MeasurePoint::<Cycle>::new(3.);
    let rotation = Measure::<Degree>::new(90.).convert::<Cycle>();
    position += rotation;
    assert_eq!(position.value, 3.25);
```

The initial position of the screw is 3 cycles.
We turn it by 90 degrees, that is a quarter of a cycle, and so we get a position of three cycles and one quarter.

For some applications, such unconstrained angular positions and rotations are useful.
Though, for some other applications, angles are used just to indicate a direction, or a spherical coordinate.
For such cases, when a full cycle is completed, we reach the same position than before the turn.
It is a kind of *modulo* arithmetic.

This is supported by this library, using the type `UnsignedDirection` or the type `SignedDirection`, as demonstrated in this code:
```rust
    let mut direction = UnsignedDirection::<Degree>::new(350.);
    let rotation = Measure::<Degree>::new(30.);
    direction += rotation;
    assert_eq!(direction.value, 20.);
```

When 30 degrees is added to 350 degrees, you get 380 degrees, but because we want to restrict our angles in the range from 0 to 360 degrees, that value becomes 20 degrees.

The name `UnsignedDirection` contains the word `Direction`, because it is meant to represent directions in the plane, not positions of a screw.

And it contains the word `Unsigned` because its values are constrained to be non-negative numbers.

Another common convention to represent directions is to use the range from -180 to +180 degrees.
For example, it is the one used to represent geographical coordinates (latitude and longitude).

Here is an example using such a type:
```rust
    let mut direction = SignedDirection::<Degree>::new(160.);
    let rotation = Measure::<Degree>::new(30.);
    direction += rotation;
    assert_eq!(direction.value, -170.);
```

In this case, we used the type `SignedDirection`, and so, when its value is incremented from 160 to 190 degrees, that value becomes -170 degrees.

So, we have seen that there are three types of directions: `MeasurePoint<Unit>`, where `Unit` is an angular unit of measurement, `UnsignedDirection`, and `SignedDirection`.

One obvious way to convert between them is to pass through the numeric value of the angle, by accessing their field `value`.
Though, there are also some functions that allow a direct conversion, in a safer way:
```rust
    let angle_measure_point = MeasurePoint::<Degree>::new(-362.);
    assert_eq!(angle_measure_point.value, -362.);

    // From MeasurePoint to SignedDirection, -362 becomes -2.
    let signed_direction = SignedDirection::from_measure_point(angle_measure_point);
    assert_eq!(signed_direction.value, -2.);

    // From MeasurePoint to UnsignedDirection, -362 becomes 358.
    let unsigned_direction = UnsignedDirection::from_measure_point(angle_measure_point);
    assert_eq!(unsigned_direction.value, 358.);

    // From UnsignedDirection to MeasurePoint, 358 remains 358.
    let mp2 = unsigned_direction.to_measure_point();
    assert_eq!(mp2.value, 358.);

    // From UnsignedDirection to SignedDirection, 358 becomes -2.
    let sd2 = unsigned_direction.to_signed_direction();
    assert_eq!(sd2.value, -2.);

    // From SignedDirection to MeasurePoint, -2 remains -2.
    let mp3 = signed_direction.to_measure_point();
    assert_eq!(mp3.value, -2.);

    // From SignedDirection to UnsignedDirection, -2 becomes 358.
    let ud2 = signed_direction.to_unsigned_direction();
    assert_eq!(ud2.value, 358.);
```

## Trigonometry

For signed or unsigned directions, but also for absolute or relative measures representing an angle, it makes sense to compute the trigonometric functions for such an angle.

Though, if you use directly the encapsulated value, you risk to make the mistake demonstrated by the following code:
```rust
    let a = Measure::<Degree>::new(90.);
    let _sine = a.value.sin();
```

This is wrong, because it applies the method `sin` to the number `90`, which is meant to be the value of the angle in degrees, while the method `sin` expects an angle in radians.

Instead, using the library `measures`, you can write this correct code:
```rust
    use measures::traits::Trigonometry;
    let a = Measure::<Degree>::new(90.);
    let _sine = a.sin();
```

The `sin` method of the type `Measure` implicitly converts the unit to `Radians`, before applying the standard-library method `sin` to the value.

The trigonometric functions available for the types `Measure`, `MeasurePoint`, `SignedDirection`, and `UnsignedDirection` are:
* `sin`, which returns the sine of the angle.
* `cos`, which returns the cosine of the angle.
* `tan`, which returns the tangent of the angle.
* `sin_cos`, which returns a pair containing the sine and the cosine of the angle.

Such four functions are available only for angles, that is when the property of the unit of measurement of the measure is the built-in type `Angle`.

The next sections explain how to use 2D measures in a plane and 3D measures in space.
If you are not interested in them, you can jump to the section [Mixed-unit operations](#mixed-unit-operations).

## Working in a plane

If our measures represent vectors or points in a plane, they must be represented by two numbers, which may be its norm and direction or its X and Y components.
The `measures` library uses the latter representation, like this example demonstrates:
```rust
    let position1 = MeasurePoint2d::<Metre>::new([4., 7.]);
    let displacement = Measure2d::<Metre>::new([2., -12.]);
    let position2 = position1 + displacement;
    println!("After I moved from position [{position1}] in the plane \
    by [{displacement}], my position had become [{position2}].");
```

It will print:
```
After I moved from position [at (4, 7) m] in the plane by [(2, -12) m], my position had become [at (6, -5) m].
```

Notice that relative 2D-measures are encapsulated in the type `Measure2d`, and absolute 2D-measures are encapsulated in the type `MeasurePoint2d`.

They are printed as a tuple of numbers, followed by a single unit symbol.
Absolute measures are precede by "`at `".

Actually, such objects contain just an array of two numbers.
You can access them as demonstrated in this code:
```rust
    let position = Measure2d::<Metre>::new([4., 7.]);
    let x_value: f64 = position.values[0];
    let y_value: f64 = position.values[1];
    println!("The numeric components of {position} are {x_value} and {y_value}.");
```

It will print:
```
The numeric components of (4, 7) m are 4 and 7.
```

Any 2D measure or measure point, in addition to being able to be created from an array of numbers, can also be created from an array of 1D-measures of the same unit and number type, using the `from` method:
```rust
    let measure_array = [
        Measure::<Metre, f32>::new(4.),
        Measure::<Metre, f32>::new(7.),
    ];
    let displacement = Measure2d::<Metre, f32>::from(measure_array);
    let measure_point_array = [
        MeasurePoint::<Metre, f32>::new(4.),
        MeasurePoint::<Metre, f32>::new(7.),
    ];
    let position = MeasurePoint2d::<Metre, f32>::from(measure_point_array);
```

Such arrays can be extracted from a slice in this way:
```rust
    use std::convert::TryFrom;
    let number_vector = vec![4., 7., 12.];
    let measure_vector = vec![
        Measure::<Metre, f32>::new(4.),
        Measure::<Metre, f32>::new(7.),
        Measure::<Metre, f32>::new(12.),
    ];
    let position1 = Measure2d::<Metre, f32>::new(
        <[f32; 2]>::try_from(&number_vector[1..]).unwrap()); // Use [7, 12]
    let position2 = Measure2d::<Metre, f32>::from(
        <[Measure<Metre, f32>; 2]>::try_from(&measure_vector[1..]).unwrap(), // Use [7 m, 12 m]
    );
```

The components of 2D-measures or measure points can be accessed also by the methods `x()` and `y()`, which returns measures, instead of primitive numbers.

```rust
    let position = Measure2d::<Metre>::new([4., 7.]);
    let x_measure: Measure<Metre> = position.x();
    let y_measure: Measure<Metre> = position.y();
    println!("The measure components of {position} are {x_measure} and {y_measure}.");
```

It will print:
```
The measure components of (4, 7) m are 4 m and 7 m.
```

Many operations available for 1D-measures are also available for 2D-measures, with some reasonable changes.
Here are some examples:
```rust
    let m = Measure2d::<Metre>::new([3., 4.]);
    let squared_norm = m.squared_norm();
    let norm = m.norm();
    println!("The squared norm of {m} is {squared_norm}; its norm is {norm}.");
```

It will print:
```
The squared norm of (3, 4) m is 25; its norm is 5 m.
```

The method `squared_norm` returns the value of the formula `m.x * m.x + m.y * m.y`.
According with the Pythagorean theorem, this value is the square of the length of the vector.

The method `norm` returns the one-dimensional measure representing the length of the vector. Its value is the square root of the value returned by `squared_norm`.

The method `normalized` returns a vector having the same direction of the original vector, but with norm one.
```rust
    let m = Measure2d::<Metre>::new([3., 4.]);
    let normalized = m.normalized();
    let norm_of_normalized = normalized.norm();
    println!("The norm of {normalized} is {norm_of_normalized}.");
```

It will print:
```
The norm of (0.6000000000000001, 0.8) m is 1 m.
```

The vector whose all components are zero, is known as the *zero vector*.
It cannot really be normalized:
```rust
fn main() {
    let zero_vector = Measure2d::<Metre>::new([0., 0.]);
    let norm_of_zero_vector = zero_vector.norm();
    let normalized_zero_vector = zero_vector.normalized();
    println!("The norm of {zero_vector} is {norm_of_zero_vector}; \
    by normalizing it, we get {normalized_zero_vector}.");
}
```

It will print:
```
The norm of (0, 0) m is 0 m; by normalizing it, we get (NaN, NaN) m.
```

Its norm is zero, and if we try to normalize it, we obtain a vector whose components are `NaN` (not-a-number).

2D measures can be compared for equality, but not for order:
```rust
    let m1 = Measure2d::<Metre>::new([3., 4.]);
    let m2 = Measure2d::<Metre>::new([4., 3.]);
    assert!(m1 != m2);

    // binary operation `<` cannot be applied to type `units::Measure2d<units::Metre>`
    // assert!(m1 < m2); // ILLEGAL
```

## Converting a direction to a `Measure2d` and conversely

As we said before, a 2D measure is a vector in a plane, and so it has a direction in that plane; though, a direction in a plane can be specified also by the types `MeasurePoint` of an angle unit, `UnsignedDirection` and `SignedDirection`.

Therefore we can, in a way, make conversions between them, like this code demonstrates:
```rust
    let downward = MeasurePoint::<Degree>::new(-90.);
    println!("The direction downward specified as a MeasurePoint (that is {downward}) can be used to construct \
    a 2D measure having any unit and norm 1.");
    let m = Measure2d::<MilePerHour>::from_angle(downward);
    println!("Using miles per hour, we get {m}.");
```

It will print:
```
The direction downward specified as a MeasurePoint (that is at -90 deg) can be used to construct a 2D measure having any unit and norm 1.
Using miles per hour, we get (0.00000000000000006123233995736766, -1) mi/h.
```

So, the function `from_angle` gets a `MeasurePoint` having an angle unit, and constructs a `Measure2d` of norm one.
Because any measure must have a unit, the unit of the constructed 2D vector must be specified or inferred from the statement in which this function is used.
In the example, the vector unit used was `MilePerHour`.

The method `from_angle` accepts also arguments of type `SignedDirection` or `UnsignedDirection`, like the following example demonstrates:
```rust
    let s_downward = SignedDirection::<Degree>::new(-90.);
    println!("The direction downward specified as a SignedDirection (that is {s_downward}) can be used to construct \
    a 2D measure having any unit and norm 1.");
    let s_m = Measure2d::<MilePerHour>::from_angle(s_downward);
    println!("Using miles per hour, we get {s_m}.");

    let u_downward = UnsignedDirection::<Degree>::new(-90.);
    println!("The direction downward specified as an UnsignedDirection (that is {u_downward}) can be used to construct \
    a 2D measure having any unit and norm 1.");
    let u_m = Measure2d::<MilePerHour>::from_angle(u_downward);
    println!("Using miles per hour, we get {u_m}.");
```

It will print:
```
The direction downward specified as a SignedDirection (that is at -90 deg (in -180°..180°)) can be used to construct a 2D measure having any unit and norm 1.
Using miles per hour, we get (0.00000000000000006123233995736766, -1) mi/h.
The direction downward specified as an UnsignedDirection (that is at 270 deg (in 0°..360°)) can be used to construct a 2D measure having any unit and norm 1.
Using miles per hour, we get (-0.00000000000000018369701987210297, -1) mi/h.
```

Conversely, any object of type Measure2d can be converted to an `UnsignedDirection` by its method `unsigned_direction`, and it can be converted to a `SignedDirection` by its method `signed_direction`.
Also in this case, the unit of the destination angle must be specified or inferred:
```rust
    let m = Measure2d::<MilePerHour>::new([0., -10.]);
    let signed_direction = m.signed_direction::<Degree>();
    print!("A 2D measure can be used to construct its signed direction {signed_direction} ");
    let unsigned_direction = m.unsigned_direction::<Degree>();
    println!("or its unsigned direction {unsigned_direction}.");
```

It will print:
```
A 2D measure can be used to construct its signed direction at -90 deg (in -180°..180°) or its unsigned direction at 270 deg (in 0°..360°).
```

## Working in the 3D space

Similarly to 2D measures, 3D measures are supported:
```rust
    let position1 = MeasurePoint3d::<Metre>::new([4., 7., -3.]);
    let displacement = Measure3d::<Metre>::new([2., -12., -2.]);
    let position2 = position1 + displacement;
    println!("After I moved from position [{position1}] in the space \
    by [{displacement}], my position had become [{position2}].");
```

It will print:
```
After I moved from position [at (4, 7, -3) m] in the space by [(2, -12, -2) m], my position had become [at (6, -5, -5) m].
```

Notice that _relative_ 3D-measures are encapsulated in the type `Measure3d`, and _absolute_ 3D-measures in the type `MeasurePoint3d`.

Of course, the member `value` is now a three-number array, and the method `z()` has been added.

The operations available for 3D measures are very similar to the ones for 2D measures, excluding the conversions from and to directions.

## Linear transformations in a plane

When working with vectors or points, some geometrical operations are quite common.

First, let's see how a rotation of a vector in a plane can be performed:
```rust
    let m1 = Measure2d::<Metre>::new([3., 1.]);
    let angle = Measure::<Degree>::new(20.);
    let rotation = LinearMap2d::rotation(angle);
    let m2 = rotation.apply_to(m1);
    println!("If measure [{m1}] is rotated counterclockwise by [{angle}], \
    measure [{m2}] is obtained.");
```

It will print:
```
If measure [(3, 1) m] is rotated counterclockwise by [20 deg], measure [(2.4770577190320564, 1.9657530507629146) m] is obtained.
```

To rotate a vector by an angle, we didn't use a method of the vector, nor a method of the angle.
Instead, we created an object specialized in performing a 2D linear transformation, whose type is `LinearMap2d`.

Rotations of vectors in a plane happen to be 2D linear transformations, and so we use such an object.

To create a specific linear transformation representing a rotation, the method `rotation` was used.
It receives an angle, and creates an object capable of rotating any vector by that angle.

To rotate a vector using a linear transformation, the method `apply_to` was called, and the vector is specified in such a call.

How is implemented a 2D linear transformation?
It is just a 2x2 matrix of numbers:
```rust
    let angle = Measure::<Degree>::new(20.);
    let rotation = LinearMap2d::rotation(angle);
    println!("A rotation by {angle} is this matrix:\n{rotation}.");
```

It will print:
```
A rotation by 20 deg is this matrix:
 ⎡ 0.9396926207859084 -0.3420201433256687 ⎤
 ⎣ 0.3420201433256687  0.9396926207859084 ⎦.
```

Linear transformations do not depend on units of measurement.

There are linear transformation objects to perform other kinds of transformations:
```rust
    let line_vector = Measure2d::<measures::dimensionless::One>::new([3., 1.]);
    let unit_vector = line_vector.normalized();
    let projection = LinearMap2d::projection_by_unit_vector(unit_vector);
    let m1 = Measure2d::<MilePerHour>::new([2., 5.]);
    let m2 = projection.apply_to(m1);
    println!("If vector [{m1}] is projected onto the line whose direction is \
    that of the vector [{line_vector}], the vector [{m2}] is obtained.");
```

It will print:
```
If vector [(2, 5) mi/h] is projected onto the line whose direction is that of the vector [(3, 1)], the vector [(3.3, 1.1) mi/h] is obtained.
```

We had a planar measure used to specify a direction. Its unit is `measures::dimensionless::One`; it is a kind of dummy built-in unit, to use when we don't need to specify a real unit of measurement.

Then, we needed to normalize it, otherwise the result would make no sense.

Then, we created a linear transformation using that unit vector, by calling the method `projection_by_unit_vector`.

Then, we created the planar measure that we want to project.

And finally, we applied the linear transformation to the planar measure, obtaining another measure having the same unit.

Here is the complete list of the methods of `LinearMap2d`, that can be used to create linear transformations in a plane:
* `new(coefficients: [[Number; 2]; 2])`: It allows to construct any 2D linear transformation, if you know its 4 coefficients.
* `rotation(angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle.
* `right_rotation()`: It is equivalent to `rotation(Measure::<Degree>::new(-90.))`. Though, it has no rounding errors, and it is more efficient.
* `left_rotation()`: It is equivalent to `rotation(Measure::<Degree>::new(90.))`. Though, it has no rounding errors, and it is more efficient.
* `projection_by_angle(angle: impl Into<MeasurePoint>)`: It returns a transformation which projects vectors onto the line which has the direction specified by an absolute angle, i.e. an angle `MeasurePoint` or a direction that can be converted to a `MeasurePoint`.
* `projection_by_unit_vector(unit_vector: Measure2d)`: Similar to `projection_by_point_angle`, but using a `Measure2d` of any unit, and having norm one.
* `reflection_by_angle(angle: impl Into<MeasurePoint>)`: It returns a transformation which reflects (a.k.a. *mirrors*) over the line which has the direction specified by an absolute angle, i.e. an angle `MeasurePoint` or a direction that can be converted to a `MeasurePoint`.
* `reflection_by_unit_vector(unit_vector: Measure2d)`: Similar to `reflection_by_point_angle`, but using a `Measure2d` of any unit, and having norm one.
* `scaling(factors: [Number; 2])`: It returns a transformation which multiplies the first coordinate by the first factor and the second coordinate by the second factor. To have an isotropic scaling, the two factors must be equal.

## Manipulating transformations

As we have seen, when you have a transformation object, you can use it by calling its method `apply_to`.
Though, you can also do other operations on a transformation object:
* `inverted()`: It returns a 2d linear transformation which has the opposite effect of the original transformation. For example, the inverse of a clockwise rotation is a counterclockwise rotation; and the inverse of a scaling which doubles the size is a scaling which halves the size.
* `combined_with(map: &LinearMap2d)`: It returns a 2D linear transformation which is equivalent to applying first the argument `map` of the call, and then the transformation on which this function is called. Notice that transformations are non-commutative, and so `lm1.combined_with(&lm2)` generally is not equal to `lm2.combined_with(&lm1)`.

## Affine transformations in a plane

2D linear transformations can operate only on 2d relative measures, i.e. values of type `Measure2d`, that are mathematical vectors.
Though, you may need to transform some points in a plane.
Here is how to rotate a point in a plane around another specified point:
```rust
    let mp1 = MeasurePoint2d::<Metre>::new([3., 1.]);
    let angle = Measure::<Degree>::new(20.);
    print!("If measure point {mp1} is rotated counterclockwise by {angle} ");

    let fixed_point = MeasurePoint2d::<Metre>::new([1., 2.]);
    print!("around measure point {fixed_point}, ");

    let rotation = AffineMap2d::rotation(fixed_point, angle);
    let mp2 = rotation.apply_to(mp1);
    println!("measure point {mp2} is obtained.");
```

It will print:
```
If measure point at (3, 1) m is rotated counterclockwise by 20 deg around measure point at (1, 2) m, measure point at (3.2214053848974853, 1.744347665865429) m is obtained.
```

The first thing to notice is that the type `AffineMap2d` is used, instead of the type `LinearMap2d`.
This is because, to transform a point measure, an affine transformation is needed.

For many affine transformations, it is needed to specify a fixed point in the transformation.
Such a point must have the same unit of measurement of the point we want to transform, and so also the objects of type `AffineMap2d` will be characterized by that unit.
You cannot use an `AffineMap2d<Kilometre>` to transform a `MeasurePoint2d<Mile>`.

This introduces the need to change the unit of an affine transformation, like this code does:
```rust
    let fixed_point = MeasurePoint2d::<Metre, f32>::new([2., 3.]);
    let angle = Measure::<Degree, f32>::new(20.);
    let rotation_in_metres = AffineMap2d::rotation(fixed_point, angle);
    println!("The rotation matrix in metres is:\n{rotation_in_metres}");

    let rotation_in_yards = AffineMap2d::rotation(fixed_point.convert::<Yard>(), angle);
    println!("The rotation matrix in yards is:\n{rotation_in_yards}");

    let rotation_converted_from_yards_to_metres = rotation_in_yards.convert::<Metre>();
    println!("The rotation matrix converted from yards to metres is:\n\
    {rotation_converted_from_yards_to_metres}");
```

It will print:
```
The rotation matrix in metres is:
 ⎡ 0.9396926  -0.34202012  1.1466751 ⎤ m
 ⎣ 0.34202012  0.9396926  -0.5031183 ⎦
The rotation matrix in yards is:
 ⎡ 0.9396926  -0.34202012  1.254019  ⎤ yd
 ⎣ 0.34202012  0.9396926  -0.5502167 ⎦
The rotation matrix converted from yards to metres is:
 ⎡ 0.9396926  -0.34202012  1.146675  ⎤ m
 ⎣ 0.34202012  0.9396926  -0.5031181 ⎦
```

Here you can see that affine transformations are 2x3 matrices of numbers, and they have a unit of measurement.

The matrix in yards differs from the one in metres only for its third column.
When this matrix is converted to metres, it becomes equal to the first matrix, with small rounding errors.

With respect to linear transformations, there is an additional affine transformation: *translation*.
Translations have no sense for linear transformations.
Here is an example of it:
```rust
    let mp1 = MeasurePoint2d::<Metre>::new([3., 1.]);
    let displacement = Measure2d::<Metre>::new([7., -12.]);
    print!("If measure point {mp1} is translated by {displacement}, ");

    let translation = AffineMap2d::translation(displacement);
    let mp2 = translation.apply_to(mp1);
    println!("measure point {mp2} is obtained.");

    let mp3 = mp1 + displacement;
    println!("If such displacement is just added to the measure point, \
    measure point {mp3} is obtained.");
```

It will print:
```
If measure point at (3, 1) m is translated by (7, -12) m, measure point at (10, -11) m is obtained.
If such displacement is just added to the measure point, measure point at (10, -11) m is obtained.
```

As you can see, applying a translation transformation to a point is equivalent to adding the displacement vector to that point.
Therefore, translation transformations are actually needed only when they are combined with other affine transformations.

Here is the complete list of the methods of `AffineMap2d`, which can be used to create affine transformations in a plane:
* `new(coefficients: [[Number; 3]; 2])`: It allows to construct any 2D affine transformation, if you know its 6 coefficients.
* `translation(displacement: Measure2d)`: It returns a transformation which adds the specified vector to measure points.
* `rotation(fixed_point: MeasurePoint2d, angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle, around the specified fixed point.
* `right_rotation(fixed_point: MeasurePoint2d)`: It is equivalent to `rotation(fixed_point: MeasurePoint2d, Measure::<Degree>::new(-90.))`. Though, it has no rounding errors, and it is more efficient.
* `left_rotation(fixed_point: MeasurePoint2d)`: It is equivalent to `rotation(fixed_point: MeasurePoint2d, Measure::<Degree>::new(90.))`. Though, it has no rounding errors, and it is more efficient.
* `projection_by_angle(fixed_point: MeasurePoint2d, angle: impl Into<MeasurePoint>)`: It returns a transformation which projects vectors onto the line going through the specified fixed point and having the direction specified by an absolute angle.
* `projection_by_unit_vector(fixed_point: MeasurePoint2d, unit_vector: Measure2d)`: Similar to `projection_by_point_angle`, but using a `Measure2d` of any unit, and having norm one.
* `reflection_by_angle(fixed_point: MeasurePoint2d, angle: impl Into<MeasurePoint>)`: It returns a transformation which reflects (a.k.a. *mirrors*) over the line going through the specified fixed point and having the direction specified by an absolute angle.
* `reflection_by_unit_vector(fixed_point: MeasurePoint2d, unit_vector: Measure2d)`: Similar to `reflection_by_point_angle`, but using a `Measure2d` of any unit, and having norm one.
* `scaling(fixed_point: MeasurePoint2d, factors: [Number; 2])`: It returns a transformation which multiplies by the corresponding factors the differences with the specified fixed point coordinates.

Here is the list of the methods using existing affine transformations in a plane:
* `convert()`: It returns an equivalent 2d affine transformation, which uses the specified unit of measurement, for the same property.
* `inverted()`: It returns a 2d affine transformation which has the opposite effect of the original transformation.
* `combined_with(map: &AffineMap2d)`: It returns a 2D affine transformation which is equivalent to applying first the argument `map` of the call, and then the transformation on which this function is called. Notice that transformations are non-commutative.
* `apply_to(m: MeasurePoint2d)`: It returns a 2D measure point obtained by applying the transformation to the specified 2D measure point.

## Transformations in 3d space

Regarding linear transformations of `Measure3d` objects and affine transformations of `MeasurePoint3d` objects, not much is to be explained, because they are quite similar to the corresponding planar transformations, just replacing `2d` with `3d`, and specifying the additional Z component when required.

The differences to take into account are these:
* There are no specific rotation at right nor rotation at left.
* Line directions are specified only by 3D unit vectors, not by angles.
* In addition to projections and reflections about a line, projections onto a plane and reflections over a plane are possible.
Such planes are specified by a unit vector orthogonal to them.

Here is the complete list of methods of `LinearMap3d`, which can be used to create linear transformations in the space:
* `new(coefficients: [[Number; 3]; 3])`: It allows to construct any 3D linear transformation, if you know its 9 coefficients.
* `rotation(unit_vector: Measure3d, angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle around the axis specified by a unit vector.
* `projection_onto_line(unit_vector: Measure3d)`: It returns a transformation which projects vectors onto the line having the direction specified by a unit vector.
* `projection_onto_plane(unit_vector: Measure3d)`: It returns a transformation which projects vectors onto the plane specified by its orthogonal unit vector.
* `reflection_over_line(unit_vector: Measure3d)`: It returns a transformation which reflects vectors over the line having the direction specified by a unit vector.
* `reflection_over_plane(unit_vector: Measure3d)`: It returns a transformation which reflects vectors over the plane specified by its orthogonal unit vector.
* `scaling(factors: [Number; 3])`: It returns a transformation which multiplies the first coordinate by the first factor, the second coordinate by the second factor, and the third coordinate by the third factor. To have an isotropic scaling, the factors must be equal.

Here is the list of the methods using existing linear transformations in the space:
* `inverted()`: It returns a 3d linear transformation which has the opposite effect of the original transformation.
* `combined_with(map: &LinearMap3d)`: It returns a 3D linear transformation which is equivalent to applying first the argument `map` of the call, and then the transformation on which this function is called. Notice that transformations are non-commutative.
* `apply_to(m: MeasurePoint3d)`: It returns a 3D measure point obtained by applying the transformation to the specified 3D measure point.

And here is the complete list of methods of `AffineMap3d`, which can be used to create affine transformations in the space:
* `new(coefficients: [[Number; 4]; 3])`: It allows to construct any 3D affine transformation, if you know its 12 coefficients.
* `translation(displacement: Measure3d)`: It returns a transformation which adds the specified vector to measure points.
* `rotation(fixed_point: MeasurePoint3d, unit_vector: Measure3d, angle: Measure)`: It returns a transformation which rotates points counterclockwise by the specified angle, around the axis going through the specified fixed point and having the direction specified by a unit vector.
* `projection_onto_line(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which projects points onto the line going through the specified fixed point and having the direction specified by a unit vector.
* `projection_onto_plane(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which projects points onto the plane going through the specified fixed point and having the specified orthogonal unit vector.
* `reflection_over_line(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which reflects points over the line going through the specified fixed point and having the direction specified by a unit vector.
* `reflection_over_plane(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which reflects points over the plane going through the specified fixed point and having the specified orthogonal unit vector.
* `scaling(fixed_point: MeasurePoint3d, factors: [Number; 3])`: It returns a transformation which multiplies by the corresponding factors the differences with the specified fixed point coordinates.

Here is the list of the methods using existing affine transformations in the space:
* `convert()`: It returns an equivalent 3d affine transformation, which uses the specified unit of measurement, for the same property.
* `inverted()`: It returns a 3d affine transformation which has the opposite effect of the original transformation.
* `combined_with(map: &AffineMap3d)`: It returns a 3D affine transformation which is equivalent to applying first the argument `map` of the call, and then the transformation on which this function is called. Notice that transformations are non-commutative.
* `apply_to(m: MeasurePoint3d)`: It returns a 3D measure point obtained by applying the transformation to the specified 3D measure point.

## Mixed-unit operations

So far, we have never multiplied one measure by another, nor divided two measures having different units.
Though, using the crate `measures`, it is allowed to write this:
```rust
    let length = Measure::<Metre>::new(10.);
    let time = Measure::<Second>::new(4.);
    print!("If I move by a distance of {length} in a time span of {time}, ");

    let velocity: Measure<MetrePerSecond> = length / time;
    println!("then I have an average speed of {velocity}.");
```

It will print:
```
If I move by a distance of 10 m in a time span of 4 s, then I have an average speed of 2.5 m/s.
```

When a measure having unit `Metre` is divided by a measure having unit `Second`, the result is a measure having unit `MetrePerSecond`.

Even square roots are supported:
```rust
    use measures::traits::Sqrt;
    let length = Measure::<Metre>::new(10.);
    let width = Measure::<Metre>::new(4.);
    print!("If a rectangle has length {length} and width {width}, ");

    let area: Measure<SquareMetre> = length * width;
    println!("its area is {area}.");

    let side: Measure<Metre> = area.sqrt();
    println!("If a square had that area, its side would be {side}.");
```

It will print:
```
If a rectangle has length 10 m and width 4 m, its area is 40 m².
If a square had that area, its side would be 6.324555320336759 m.
```

When a measure having unit `Metre` is multiplied by a measure having the same unit, the result is a measure having unit `SquareMetre`.
And when the square root is extracted from a measure having unit `SquareMetre`, the result is a measure having unit `Metre`.

Such operations are available also in two or three dimensions:
```rust
    let distance = Measure3d::<Metre>::new([10., 20., 40.]);
    let time = Measure::<Second>::new(4.);
    print!("If I move by a distance of {distance} in a time span of {time}, ");

    let velocity: Measure3d<MetrePerSecond> = distance / time;
    println!("then I have an average speed of {velocity}.");
```

It will print:
```
If I move by a distance of (10, 20, 40) m in a time span of 4 s, then I have an average speed of (2.5, 5, 10) m/s.
```

## Dot products and cross products

In one dimension, there is only one kind of multiplication between measures, but in a plane and in space there are two kinds of multiplication: the *dot product* and the *cross product*.

For example, using force and displacement, in one dimension, we have just a simple multiplication:
```rust
    let force = Measure::<Newton>::new(10.);
    print!("If a force of {force} is applied to an object, ");

    let distance = Measure::<Metre>::new(4.);
    print!("while it moves in the same direction by {distance}, ");

    let work: Measure<Joule> = force * distance;
    println!("then a work of {work} is performed.");
```

It will print:
```
If a force of 10 N is applied to an object, while it moves in the same direction by 4 m, then a work of 40 J is performed.
```

But in a plane, we can have two kinds of multiplication: the *dot product* and the *cross product*:
```rust
    use measures::traits::CrossProduct;
    let force = Measure2d::<Newton>::new([10., 20.]);
    print!("If, in a plane, a force of {force} is applied to an object, ");

    let distance = Measure2d::<Metre>::new([4., 3.]);
    print!("while it moves in that plane by {distance}, ");

    let work: Measure<Joule> = force * distance;
    println!("then a work of {work} is performed.");

    let torque: Measure<NewtonMetre> = force.cross_product(distance);
    println!("If that force is applied to a point \
    which is detached from a pivot by an arm of {distance}, \
    then that force causes a torque of {torque}.");
```

It will print:
```
If, in a plane, a force of (10, 20) N is applied to an object, while it moves in that plane by (4, 3) m, then a work of 100 J is performed.
If that force is applied to a point which is detached from a pivot by an arm of (4, 3) m, then that force causes a torque of -50 N·m.
```

Notice that the operator `*`, when applied to two 2D vectors, computes the dot product between them.
Instead, to compute the cross product, the method `cross_product` is called. It needs the declaration:
```
    use measures::traits::CrossProduct;
```

In physics, while the dot product between a force and a displacement represents a work (or energy), and so that expression returns a measure in `Joule`, a cross product between a force and a displacement represents a torque (or moment of force), and so such an expression returns a measure in `NewtonMetre`. The property of `Joule` is `Energy`, while the property of `NewtonMetre` is `Torque`.

In 3D space we have this:
```rust
    use measures::traits::CrossProduct;
    let force = Measure3d::<Newton>::new([10., 20., 0.4]);
    print!("In space, if a force of {force} is applied to an object, ");

    let distance = Measure3d::<Metre>::new([4., 3., -5.]);
    println!("while it moves by {distance}, ");

    let work: Measure<Joule> = force * distance;
    println!("then a work of {work} is performed.");

    let torque: Measure3d<NewtonMetre> = force.cross_product(distance);
    println!("If that force is applied to a point \
    which is detached from a pivot by an arm of {distance}, \
    that force causes a torque of {torque}.");
```

It will print:
```
In space, if a force of (10, 20, 0.4) N is applied to an object, while it moves by (4, 3, -5) m,
then a work of 98 J is performed.
If that force is applied to a point which is detached from a pivot by an arm of (4, 3, -5) m, that force causes a torque of (-101.2, 51.6, -50) N·m.
```

Even in this case, the dot product returns a scalar, having type `Measure<Joule>`.

Though, regarding the cross product, the result is different.

For vectors in a plane, their torque is always perpendicular to that plane, and so we are not interested in its X and Y components, which are always zero.
Therefore, cross products in a plane expression return a scalar, containing the only interesting component of the torque, Z.

Instead, in 3D space, the torque is a vector which could have any direction, and so the cross product returns a value of type `Measure3d`.

## Measures with uncertainty

_Warning: The implementation of uncertainty handling with measures is not complete yet._

So far, every 1-dimension measure encapsulated just one number, every 2-dimension measure encapsulated two numbers, and every 3-dimension measure encapsulated three numbers.
So, every measure was represented by as many numeric values as the dimensions of the space.
Such numbers represented exact measures.

Though, in many application fields, measures are probability distributions, described by their mean and variance.

The library `measures` allows to specify also such kind of measures, as demonstrated by the following code:

```rust
    let mass = ApproxMeasure::<Kilogram>::with_variance(87.3, 0.04); // mean and variance
    assert_eq!(mass.value, 87.3);
    assert_eq!(mass.variance, 0.04);
    assert_eq!(mass.uncertainty(), Measure::<Kilogram>::new(0.2)); // standard deviation
    println!("{mass}, {mass:?}"); // It prints: 87.3 ± 0.2 Kg, 87.3 [σ²=0.04] Kg
```

The variable `mass` is an approximate measure, having 87.3 as mean and 0.04 as variance of its probability distribution.
It is create by the function `with_variance`, which requires to specify also the variance, in addition to the mean.

The mean is accessed by the member used for exact measures, `value`.
The unit of measurement of the mean is the same of the measure.
So, in this example, it is `Kilogram`, although the member `value` is a primitive number.

The variance is stored in a separate public field, named `variance`.

In theory, the unit of measurement of the variance is the square of the unit of measurement of the measure.
In this example, it would be `Kilogram²`.
Though, such a value is not actually needed as a measure, and so there is no need to define such a unit of measurement.

The square root of the variance is used often.
Regarding probability distributions, it is named `standard deviation`.
Though, regarding measurements, it is usually named, inappropriately, `absolute error` or, better, `absolute uncertainty`.

Its unit of measurement is the same of the measure.
The method `uncertainty` computes the square root of the variance, which is returned encapsulated in a `Measure` object.

When an approximate measure is printed or converted to a string, also its uncertainty is printed.
Instead, when an approximate measure is printed for debug, its variance is printed.
Here are some examples:

```rust
    let mass = ApproxMeasure::<Kilogram>::with_variance(87.3, 0.04); // mean and variance
    assert_eq!(mass.uncertainty(), Measure::<Kilogram>::new(0.2)); // standard deviation
    assert_eq!(format!("{mass}, {mass:?}"), "87.3 ± 0.2 kg, [µ=87.3 σ²=0.04] kg");
```

The variable `mass` represents an approximate measure of a mass in kilograms.
Its probability distribution has mean 87.3 and variance 0.04, and so the standard deviation of the distribution, or absolute uncertainty of the measure, is 0.2 kilograms.

When operations are applied to approximate measures, their variances must be appropriately propagated.
The library `measures` makes the following assumptions:
* The uncertainty is much smaller than the measure value, and so only first-degree effects can be taken into account.
* In case of binary operations between measures (sum, subtraction, multiplication, division), the two measures participating in the operation are statistically independent, or uncorrelated.

Such assumptions allow to keep simple the library API and rather efficient the implementation.
Here are some examples:

```rust
    let mass1 = ApproxMeasure::<Kilogram>::with_variance(87.3, 0.04);
    assert_eq!((mass1 * 3.).uncertainty(), Measure::<Kilogram>::new(0.6));
    let mass2 = ApproxMeasure::<Kilogram>::with_variance(100., 0.09);
    assert_eq!((mass1 + mass2).variance, 0.13);
    assert_eq!((mass1 - mass2).variance, 0.13);
```

The above example demonstrates that:
* When a measure is multiplied by a number, its variance is also multiplied by that number.
* When two measures are added or subtracted, their variances are added.

Here are examples involving multiplication and division:
```rust
    let mass1 = ApproxMeasure::<Kilogram>::with_variance(87.3, 0.04);
    let mass2 = ApproxMeasure::<Kilogram>::with_variance(100., 0.09);
    let computed_variance = (0.04 / 87.3 / 87.3 + 0.09 / 100. / 100.) * (87.3 / 100.) * (87.3 / 100.);
    assert_eq!((mass1 / mass2).variance, computed_variance);
    let velocity = ApproxMeasure::<MetrePerSecond>::with_variance(2.3, 0.000144); // mean and variance
    let computed_variance = (0.04 / 87.3 / 87.3 + 0.000144 / 2.3 / 2.3) * (87.3 * 2.3) * (87.3 * 2.3);
    assert_eq!((mass1 * velocity).variance, computed_variance);
```

The above example demonstrates that when two measures are multiplied or divided, their relative variances are added.
The relative variance is the ratio between the variance and the square of the mean.

## How units of measurement and their relationships are defined

So far, we have used types, functions, and units of measurement without needing to define them.

Actually, our small program begins by importing all there is in the module `units`, defined by yourself at the beginning of this tutorial.

Let's see its contents.

### Features

It begins with this statement:
```rust
measures::define_measure_types! {
    with_points with_directions with_2d with_3d with_transformations exact with_approx,
```

This is a macro invocation, which expands to the code which defines these types:
* `Measure`: 1D relative measure.
* `MeasurePoint`: 1D absolute measure.
* `Measure2d`: 2D relative measure.
* `MeasurePoint2d`: 2D absolute measure.
* `Measure3d`: 3D relative measure.
* `MeasurePoint3d`: 3D absolute measure.
* `LinearMap2d`: Linear transformation of objects of type `Measure2d`.
* `AffineMap2d`: Affine transformation of objects of type `MeasurePoint2d`.
* `LinearMap3d`: Linear transformation of objects of type `Measure3d`.
* `AffineMap3d`: Affine transformation of objects of type `MeasurePoint3d`.
* `UnsignedDirections`: Directional angle between 0 and 360 degrees.
* `SignedDirections`: Directional angle between -180 and +180 degrees.
* `DecibelsMeasureFormatter`: Object to print measures as decibels.
* `ApproxMeasure`: 1D relative measure with uncertainty variance.
* `ApproxMeasurePoint`: 1D absolute measure with uncertainty variance.
* `ApproxMeasure2d`: 2D relative measure with uncertainty variances.
* `ApproxMeasurePoint2d`: 2D absolute measure with uncertainty variances.
* `ApproxMeasure3d`: 3D relative measure with uncertainty variances.
* `ApproxMeasurePoint3d`: 3D absolute measure with uncertainty variances.
* `ApproxDecibelsMeasureFormatter`: Object to print measures as decibels with uncertainty variance.
* `ApproxLinearMap2d`: Linear transformation of objects of type `Measure2d` with uncertainty variances.
* `ApproxAffineMap2d`: Affine transformation of objects of type `MeasurePoint2d` with uncertainty variances.
* `ApproxLinearMap3d`: Linear transformation of objects of type `Measure3d` with uncertainty variances.
* `ApproxAffineMap3d`: Affine transformation of objects of type `MeasurePoint3d` with uncertainty variances.
* `Dimensionless`: Property of dimensionless units.
* `One`: The only built-in unit having the property `Dimensionless`.
* `Angle`: Property of angle units.
* `Radian`: The only built-in unit having the property `Angle`.

The first line in the macro invocation contains the list of desired features.
In this example, all the features are specified.

Therefore, if there is the need to work using relative and absolute measures, using angular directions, in 1D, in 2D, in 3D, using transformation objects, with exact measures and measures with uncertainty, the macro `define_measure_types` with such arguments should be used.
It will declare every kind of measures, directions and transformations.

Though, if only some of such types are needed, you can remove some of those arguments, so reducing the number of declared types.
In this way, the resulting code would be quicker to compile, and developers would not be bothered by unneeded types.

Relative measures cannot be removed, but if absolute measures, i.e. points in affine spaces, are not needed, you can remove the feature *`with_points`* from the macro invocation.
The following types will not be generated: `MeasurePoint`, `MeasurePoint2d`, `MeasurePoint3d`, `AffineMap2d`, and `AffineMap3d`.

If angular directions are not needed, you can remove the feature *`with_directions`*.
The types `UnsignedDirections` and `SignedDirections` will not be generated.

1-dimension measures cannot be removed, but if 2-dimension measures are not needed, you can remove the feature *`with_2d`*.
The following types will not be generated: `Measure2d`, `MeasurePoint2d`, `LinearMap2d` and `AffineMap2d`.

If 3-dimension measures are not needed, you can remove the feature *`with_3d`*.
The following types will not be generated: `Measure3d`, `MeasurePoint3d`, `LinearMap3d` and `AffineMap3d`.

If the linear or affine transformations in a plane or in the space are not needed, you can remove the feature *`with_transformations`*.
The following types will not be generated: `LinearMap2d`, `AffineMap2d`, `LinearMap3d` and `AffineMap3d`.

If exact measures are not needed, because you are going to use only measures with uncertainty, you can remove the feature *`exact`*.
The following types will not be generated: `Measure`, `MeasurePoint`, `Measure2d`, `MeasurePoint2d`, `Measure3d`, `MeasurePoint3d`, `LinearMap2d`, `AffineMap2d`, `LinearMap3d`, `AffineMap3d`, `MeasurePoint3d`, `UnsignedDirections`,`SignedDirections`, `DecibelsMeasureFormatter`.

Conversely, if measures with uncertainty are not needed, because you are going to use only exact measures, you can remove the feature *`with_approx`*.
The following types will not be generated: `ApproxMeasure`, `ApproxMeasurePoint`, `ApproxMeasure2d`, `ApproxMeasurePoint2d`, `ApproxMeasure3d`, `ApproxMeasurePoint3d`, `ApproxLinearMap2d`, `ApproxAffineMap2d`, `ApproxLinearMap3d`, `ApproxAffineMap3d`, `ApproxMeasurePoint3d`, `ApproxUnsignedDirections`,`ApproxSignedDirections`, `ApproxDecibelsMeasureFormatter`.

Notice that it makes no sense to remove both the features `exact` and `with_approx`.

Actually, these definitions do not define all the properties and all the units of measurement we used in our examples.

The only predefined properties are `Dimensionless` and `Angle`, and the only predefined units of measurement are `Radian`, used for angles, and `One` used for dimensionless measures.

Every other property and every other unit must be explicitly defined.

### Sections

The file `units.rs`, from the third line, contains the sections
```
    scalar_properties [
      ...
    ]
    vector_properties [
      ...
    ]
    angle_measurement_units [
      ...
    ]
    relationships [
      ...
    ]
```

For some properties, it makes sense to have a 2D measure or 3D measure, but for others properties it does not make sense.
To forbid the creation of nonsensical expressions like `Measure2d<Kilogram>` or `MeasurePoint3d<Minute>`, properties are classified as:
* *Scalar properties*: The ones for which it is forbidden to create multidimensional measures. Examples of scalar properties are `Angle`, `Time`, `Mass`, `Temperature`, `ElectricCharge`, and `Information`.
* *Vector properties*: The ones for which it is allowed to create multidimensional measures. Examples of vector properties are `Dimensionless`, `Length`, `Velocity`, `Acceleration`, `Force`, `Torque`, `ElectricFieldStrength`, and `MagneticFieldStrength`.

The section starting with `scalar_properties` contains the definitions of scalar properties and their units of measurement.

The section starting with `vector_properties` contains the definitions of vector properties and their units of measurement.

The section starting with `angle_measurement_units` contains the definitions of the units of measurement of the only angle property, which is built-in, it is named `Angle`.
The property `Angle` is a scalar property, and it has a built-in unit of measurement, named `Radian`.
In this section, additional units of measurement can be defined, like `Degree` or `Cycle`.

Also the property `Dimensionless` is built-in.
It is a vector property, and it has a built-in unit of measurement, named `One`.
Yet, no other unit of measurement can be defined for this property.

The section starting with `relationships` contains the definitions of the relationships among units of measurement, used to divide or multiply two measures.

### Scalar and vector properties

Inside the section `scalar_properties`, there is a subsection for each scalar property to define.
For example, a possible content to define the property `Time` is this:
```rust
        Time [
            Second {
                suffix: " s",
            }
            Day {
                suffix: " D",
                ratio: 86400.,
            }
        ]
```

It declares that there is a scalar property named `Time`, for which there are two units of measurement, named `Second` and `Day`.

For another example, a possible content to define the property `Temperature` is this:
```rust
        Temperature [
            Kelvin {
                suffix: " \u{b0}K", // °K
            }
            Celsius {
                suffix: " \u{b0}C", // °C
                ratio: 1.,
                offset: 273.15,
            }
        ]
```

It declares that there is a scalar property named `Temperature`, for which there are two units of measurement, named `Kelvin` and `Celsius`.

The definition of a unit of measurement includes some fields.
Here are the possible fields:
* *`suffix`*: This is the only mandatory field for every unit of measurement.
It is the text to be appended after the numeric value, when a measure is converted to a string.
* *`ratio`* is an `f64` value that represents how many base units there are in this unit of measurement. For example, the base unit of `Time` is the second, and there are 86400 seconds in a single day, and so the ratio for `Day` is `86400.`.
If this field is not specified, it is assumed to be equal to 1.
Therefore, for the base unit, which, by definition, has ratio 1, this field is not needed.
* *`offset`* is an `f64` value that represents how many base units there are from the origin of the base unit and the origin of this unit of measurement. For example, the base unit of `Temperature` is the kelvin, and there are 273.15 kelvins from the origin of the kelvin scale to the origin of the celsius scale, and so the offset for `Celsius` is `273.15`.
If this field is not specified, it is assumed to be equal to 0.
Therefore, for the base unit, which, by definition, has offset 0, this field is not needed.
Actually, the property `offset` is very rarely used. In this example file is used only for temperature scales. It could be used also for calendars.

Vector properties and their units are defined with a similar syntax.
The only difference is that, for vector properties, the field `offset` is not allowed, because it makes little sense to have a different origin for two vector units of the same property.

### Defining units of angles

Angles are a particular property.

The property `Angle` is predefined.
The unit `Radian` is the only predefined unit of `Angle`, and its ratio is 1, i.e. it is the base unit of angles.

Other angle units can be defined, but only in the section `angle_measurement_units`, like it is demonstrated here:
```rust
        Cycle {
            suffix: " rev",
            cycle_fraction: 1.,
        }
        Degree {
            suffix: " deg",
            cycle_fraction: 360.,
        }
```

The definition of an angle unit of measurement includes two mandatory properties.
* *`suffix`* is the string printed after the number, when the measure is printed, similarly to other units.
* *`cycle_fraction`* is ratio between the full cycle and the current unit.

Notice that the properties `offset` and `ratio` are not allowed.

The unit `Radian` is predefined with these values:
```rust
            suffix: " rad",
            cycle_fraction: core::f64::consts::TAU,
```
Where the constant TAU is about 6.283185307.

## Defining relationships among units

With the above declarations you can define measures, points, directions, and transformations, you can make conversions between units of the same property, you can compute additions, subtractions and divisions between measures of the same unit, and you can multiply or divide a measure by a number.

Though, you still cannot compute multiplications or divisions involving measures of different properties, like `Measure<Newton> * Measure<Metre>`.

To do that, you need to teach it to your application.

How to do this is demonstrated in the initial part of the file `units.rs`, after the open square bracket.

For example, there is a line with this content:
```rust
        Joule 1 == Newton 1 * Metre 1,
```

This is a rule which, conceptually, means: "A 1D measure in Joule can be obtained by multiplying 1D measure in Newton by a 1D measure in Metre".

Actually, such line causes the generation of 4 methods:
* An implementation of the operator "`*`" to multiply a `Measure<Newton>` by a `Measure<Metre>` getting a `Measure<Joule>`.
* An implementation of the operator "`*`" to multiply a `Measure<Metre>` by a `Measure<Newton>` getting a `Measure<Joule>`.
* An implementation of the operator "`/`" to divide a `Measure<Joule>` by a `Measure<Metre>` getting a `Measure<Newton>`.
* An implementation of the operator "`/`" to divide a `Measure<Joule>` by a `Measure<Newton>` getting a `Measure<Metre>`.

That file contains hundreds of similar rules.

Such rules have one of the following allowed formats:
* `U1 1 == U2 1 * U3 1`: A 1D measure in unit U1 can be obtained by multiplying a 1D measure in unit U2 by a 1D measure in a a different unit U3.
* `U1 1 == U2 1 * __ 1`: A 1D measure in unit U1 can be obtained by multiplying a 1D measure in unit U2 by a 1D measure in the same unit U2.
* `U1 2 == U2 1 * U3 2`: A 2D measure in unit U1 can be obtained by multiplying a 1D measure in unit U2 by a 2D measure in unit U3.
* `U1 2 == U2 2 * U3 1`: A 2D measure in unit U1 can be obtained by multiplying a 2D measure in unit U2 by a 1D measure in unit U3.
But, because multiplication between scalars and vectors is commutative, this rule is equivalent to the previous one.
* `U1 3 == U2 1 * U3 3`: A 3D measure in unit U1 can be obtained by multiplying a 1D measure in unit U2 by a 3D measure in unit U3.
* `U1 3 == U2 3 * U3 1`: A 3D measure in unit U1 can be obtained by multiplying a 3D measure in unit U2 by a 1D measure in unit U3.
But, because multiplication between scalars and vectors is commutative, this rule is equivalent to the previous one.
* `U1 1 == U2 2 * __ 2`: A 1D measure in unit U1 can be obtained by multiplying a 2D measure in unit U2 by a 2D measure in the same unit U2.
This one is the dot product in a plane.
* `U1 1 == U2 2 * U3 2`: A 1D measure in unit U1 can be obtained by multiplying a 2D measure in unit U2 by a 2D measure in a different unit U3.
Also this one is the dot product in a plane.
* `U1 1 == U2 3 * __ 3`: A 1D measure in unit U1 can be obtained by multiplying a 3D measure in unit U2 by a 3D measure in the same unit U2.
This one is the dot product in the space.
* `U1 1 == U2 3 * U3 3`: A 1D measure in unit U1 can be obtained by multiplying a 3D measure in unit U2 by a 3D measure in a different unit U3.
Also this one is the dot product in the space.
* `U1 1 == U2 2 X __ 2`: A 1D measure in unit U1 can be obtained by computing the cross product of a 2D measure in unit U2 and a 2D measure in the same unit U2.
* `U1 1 == U2 2 X U3 2`: A 1D measure in unit U1 can be obtained by computing the cross product of a 2D measure in unit U2 and a 2D measure in a different unit U3.
* `U1 3 == U2 3 X __ 3`: A 3D measure in unit U1 can be obtained by computing the cross product of a 3D measure in unit U2 and a 3D measure in the same unit U2.
* `U1 3 == U2 3 X U3 3`: A 3D measure in unit U1 can be obtained by computing the cross product of a 3D measure in unit U2 and a 3D measure in a different unit U3.

Each of these rules causes the generation of a handful of trait implementations.

## Creating a custom file `units.ts`

The file `units.ts` is quite useful for learning, for experimenting, and for copying&pasting useful definitions.
Though, it is not recommended for production use, for the following reasons:
* Such large file increases your code base.
* Such large file increases compilation time.
* Such file uses words or output suffix you may dislike.
For example, if you prefer, you could replace `Length` with `Space`, or `Metre` with `Meter`, or `" m"` with `" [m]"`.
* Such files have names which conflicts with some names already used in your project.
* Such files miss some properties or units or relations you may need.

Therefore, the suggested procedure for production code is the following one:
* Create your own file `units.rs` for your project, with this contents:
```rust
measures::define_measure_types! {
    with_points with_directions with_2d with_3d with_transformations exact with_approx,
}
```
* Remove from the second line of the file the features you don't need.
* Search the provided example file for the rules, the property definitions, and the unit definitions you need, or the ones most similar to what you need.
Copy and paste them into your own file.
* Edit your file according to your needs.

Here is an example of contents for a file `units.ts`;
```rust
measures::define_measure_types! {
    exact,
    [
        Bit 1 == BitPerSecond 1 * Second 1,
        Byte 1 == BytePerSecond 1 * Second 1,
    ]
}

measures::measurement_scalar_property! { Information }
measures::measurement_scalar_property! { Time }
measures::measurement_scalar_property! { InformationRate }

measures::measurement_unit! {
    name: Bit,
    property: Information,
    suffix: " b",
}

measures::measurement_unit! {
    name: Byte,
    property: Information,
    ratio: 8.,
    suffix: " B",
}

measures::measurement_unit! {
    name: Second,
    property: Time,
    suffix: " s",
}

measures::measurement_unit! {
    name: BitPerSecond,
    property: InformationRate,
    suffix: " b/s",
}

measures::measurement_unit! {
    name: BytePerSecond,
    property: InformationRate,
    ratio: 8.,
    suffix: " B/s",
}
```

Then, you should be able to write and compile a file `main.rs`, having this contents:
```rust
mod units;
use units::{Bit, Byte, Measure, Second};
fn main() {
    let info_size = Measure::<Byte>::new(2700.);
    let time = Measure::<Second>::new(5.1);
    println!("The transmission rate is {:.2}.", info_size.convert::<Bit>() / time);
}
```

It should print:
```
The transmission rate is 4235.29 b/s.
```

## Using decibels

For some kinds of measures, it is customary to use logarithmic values, typically in the form of decibels.
Decibels are tenths of the 10-base logarithm of the actual value.

For example, if we have a measure of 0.001 watts, we can compute its 10-base logarithms, obtaining -3.
So, this measure can be said to be -30 decibels of watt, or `-30 db W`.

The crate `measures` supports decibels only for the types `Measure` and `ApproxMeasure`, as demonstrated in the following code:
```rust
    use measures::traits::Decibel;
    let one_milliwatt = Measure::<Watt>::new(0.001);
    print!("{one_milliwatt:.4} "); // 0.0010 W
    let one_milliwatt_in_db = one_milliwatt.value.to_decibels();
    print!(",{one_milliwatt_in_db:.1}, "); // -30.0
    let one_milliwatt_value = one_milliwatt_in_db.decibels_to_value();
    print!("{one_milliwatt_value:.4}, "); // 0.0010
    print!("{:.4};", one_milliwatt.decibels_formatter()); // -30.0000 dB W
```

It will print: `0.0010 W, -30.0, 0.0010, -30.0000 dB W;`.

Actually, the variable `one_milliwatt` is a usual measure, with its inner value of `0.001` watts, i.e. one milliwatt.
It is printed as `0.0010 W`.

Then, its value is converted to its corresponding decibels value, by calling the method `to_decibels`.
The result is a number, and it is printed as `-30.0`.

Then, this decibels value is converted back to its corresponding linear value, by calling the method `decibels_to_value`.
The result is a number, and it is printed as `0.0010`.

These operations are defined in the trait `Decibel`, implemented for the types `f32` and `f64`.

Then, to print the decibels corresponding to the measure `one_milliwatt`, its method `decibels_formatter` is called.
It returns an object of type `DecibelsMeasureFormatter`, which encapsulates the measure, and which implements the traits `Display` and `Debug`.
Such implementations cause that, when this object is printed, it will show the string "` dB`" between the numeric value and the unit suffix.
Therefore, it is printed as `-30.0 dB W`.

## The trait `Default`

For any measure type, the trait `Default` is implemented.

For the transformations types, the method `default()` returns an *identity* transformation, not a zero transformation.
For all the other types, `default()` returns a *zero* object, i.e. an object encapsulating one or more zero numbers.

Here is an example:
```rust
    println!("{}", Measure3d::<Newton>::default());
    let m = Measure3d::<Newton>::new([7., -3., 1.2]);
    println!("{m}");
    let map = LinearMap3d::default();
    println!("{map}");
    println!("{}", map.apply_to(m));
```

It will print:
```text
(0, 0, 0) N
(7, -3, 1.2) N
 ⎡ 1 0 0 ⎤
 ⎢ 0 1 0 ⎥
 ⎣ 0 0 1 ⎦
(7, -3, 1.2) N
```

## Creating the automatic documentation

The library `measures` is essentially a set of macros, with some utility types, traits, and functions.

Therefore, the documentation automatically generated for the library itself is useful only for calling the main macro, which is `define_measure_types`.

When editing application code, the documentation that can be really useful is the one generated for the code that is generated by the macros.
The `measures` macros must be called from inside application code, best if inside a specific module, like `units`.
So, also the generated code will be part of that application module.

Therefore, to generate the documentation for the `measures` library, you should simply generate the documentation for your application, by typing the command:
```
cargo doc --open
```

and then look inside the `units` module of the generated documentation.

## Abbreviations

You may think this library is somewhat verbose.
Actually, in some other libraries, you can specify a mass of 2 kilograms by writing simply `2. * Mass` or `2. * kg`.

This library has the design principle of avoiding very short names, because they could too easily collide with other names.
In addition, in this library there is not just one kind of measures, and so it is useful to specify which is meant.

However, if you prefer to be more concise, it is quite easy to do so, by using some small definitions. For example:
```rust
const M: Measure<Metre> = Measure::<Metre>::new(1.);
const KG: Measure<Kilogram> = Measure::<Kilogram>::new(1.);
```

After the previous definitions, you can write this code:
```rust
let _length = 2. * M;
let _mass = 2. * KG;
```

---

**Happy measuring!**
