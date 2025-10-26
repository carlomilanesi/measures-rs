# Motivation

This document describes the rationale for the library `measures`, contained in the package `measures-rs`.

You can store the value of physical or geometrical quantities by using only the primitive Rust data types, like `f32` or `f64`.
Though, if you do this, several programming errors are possible.
But some of them can be avoided, at no run-time cost, by encapsulating these values in custom types.

## Mixing measures having different unit of measurement

Consider the following Rust code:

```rust
    let mass_in_kg = 1.2; // In kilograms.
    let mut mass_in_pounds: f64; // In pounds.
    mass_in_pounds = mass_in_kg; // Wrong.
    fn set_mass_in_pounds(val: f64) { /* ... */ }
    set_mass_in_pounds(mass_in_kg); // Wrong.
```

In the third statement, a value meant to be in kilograms is assigned to a variable meant to be in pounds.
In the last statement, a function meant to receive a value in pounds receives a value meant to be in kilograms.

This is avoided by using this code:

```rust
    let mass_in_kg = Measure::<KiloGram>::new(1.2);
    let mut mass_in_pounds: Measure<Pound>;
    mass_in_pounds = mass_in_kg; // Compilation error.
    fn set_mass(val: Measure<Pound>) { /* ... */ }
    set_mass(mass_in_kg); // Compilation error.
```

The third and the last statements cause "type mismatch" compilation errors, because we are trying to assign a `Measure<KiloGram>` to a `Measure<Pound>`.
Here, the values are encapsulated in objects whose types is characterized by a unit of measurement.
In Rust, the left and right side of an assignment must have the same type.

## Unit conversions

Consider the following Rust code:

```rust
    let mass_in_kilograms = 1.2; // kilograms
    let mut mass_in_pounds: f64; // pounds

    // Wrong conversion operation; it should be a division.
    mass_in_pounds = mass_in_kilograms * 0.45359237;

    // Wrong conversion ratio; it should be 0.45359237.
    mass_in_pounds = mass_in_kilograms / 453.59237;
```

The third statement tries to convert a value from kilograms to pounds, but it uses a multiplication instead of a division.
Also the fourth statement tries to do the same unit conversion, but it uses a wrong divisor.

This is avoided by using the following code:

```rust
    let mass_in_kilograms = Measure::<KiloGram>::new(1.2);
    let mut mass_in_pounds: Measure<Pound>;
    mass_in_pounds = mass_in_kilograms.convert();
    // or
    let another_mass_in_pounds = mass_in_kilograms.convert::<Pound>();
```

The third statement simply invokes the method `convert`.
The correct conversion formula is inferred from the type of the source expression and the type of the destination variable.
Instead, in the last statement, the destination variable does not have a type annotation, and so the unit conversion must specify the destination unit of measurement.

## Non-standard units of measurement

Some other libraries, like, for example, the crate UOM, encapsulate each value in an object which represents that value in a standard unit of measurement, typically taken from the SI international standard.
This raises the problem of how to handle non-standard units, like milligrams or pounds for mass.
These libraries always use internally a standard unit, and so they can convert, at run-time, any input value from a user-chosen unit to a standard unit, and any output value from a standard unit to a user-chosen unit.

Such technique has the following drawbacks:

1. It forces developers to use units that are different from the ones actually seen by users.
   For example, a developer wants to use only the pound as a unit of mass.
   But if he tries to explore with a debugger the contents of an object encapsulating a mass, he finds a value in kilograms.
2. It may introduce rounding errors.
3. It introduces a small conversion overhead for every I/O operation.

Instead, when using the library `measures`, the developer can choose the pound as the unit of mass, so that the input operations are in pounds, the computations are in pounds, and the output operations are in pounds.
This eases debugging, avoids unexpected rounding errors, and avoids run-time conversion overhead.

Regarding the rounding errors, consider, for example, the following Rust code, which uses the crate UOM:

```rust
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::f32::*;
use uom::si::mass::{gram, kilogram, pound};

fn main() {
    let mass1 = Mass::new::<gram>(8030.);
    let mass2 = Mass::new::<pound>(27.);
    let as_grams = Mass::format_args(gram, Abbreviation);
    let as_kilograms = Mass::format_args(kilogram, Abbreviation);
    let as_pounds = Mass::format_args(pound, Abbreviation);
    println!(
        "{}, {}, {}.",
        as_grams.with(mass1),
        as_kilograms.with(mass1),
        as_pounds.with(mass2)
    );
}
```

It defines the variables `mass1` and `mass2` by specifying a value in grams and a value in pounds, but it stores internally those values in kilograms.
Then, it prints the first variable using two different formatters, one for grams, and one for kilograms, and the second variable using the formatter for pounds.
The output is: `8030.0005 g, 8.030001 kg, 27.000002 lb.`.

As you can see, even the value in grams and the value in pounds are different from the original values.
This happens even with such small integer values, which usually do not introduce rounding errors in floating-point numbers.

The corresponding program using `measures` is this:

```rust
fn main() {
    let mass1 = Measure::<Gram, f32>::new(8030.);
    let mass2 = Measure::<Pound, f32>::new(27.);
    println!("{}, {}, {}.", mass1, mass1.convert::<KiloGram>(), mass2);
}
```

It will print `8030 g, 8.030001 kg, 27 lb.`.

Only the value in kilograms is not exact, because its value is stored internally in grams, and the conversion to kilograms introduced a rounding error.

## All-encompassing libraries

To represent all possible physical or geometrical properties, such libraries try to define inside them all existing properties and all existing units.
This has the following drawbacks:
1. It makes the library heavy to compile, as the resulting types are quite numerous.
2. If the library is not extensible, it may be incomplete, as there are some little used units that some applications may require and which have not been included in the library.
   If the library is extensible, it is usually somewhat difficult to extend it.
3. It forces developers to use a specific natural language to name the properties and their units of measurements, instead of the natural language preferred by the developers.
   For example, some people prefer to write `metre` and others prefer to write `meter`, for the same unit.

Instead, the library `measures` defines just two built-in properties, `Dimensionless` and `Angle`, and it defines just two built-in units of measurements, `One` for the property `Dimensionless`, and `Radian` for the property `Angle`.

Every other property and every other unit of measurement can be easily defined by the application developers. As consequences:
1. For typical applications, this library uses little compile-time and run-time resources, because it will include only the few properties and units needed by the application.
2. For this library it is easy to add custom properties or custom units.
3. The developers can define their preferred names for properties and for units of measurement, and they can define also the preferred unit-of-measurement suffixes for outputting measures.

## Specifying derived units

If you have a value representing a mass and another value representing a volume, the division of the first value by the second value must represent a mass density.
This can be enforced by specifying relationships among units.
In other libraries, these relationships are built-in, and implemented by very complex types, in a way that cannot be modified by the application developers.
This has the following drawbacks:
1. The complexity of the resulting types makes the application heavy to compile.
2. The complexity of the resulting types makes the compiler error messages quite cryptic.
3. No relationships can be added by application developers.

Instead, the library `measures` has no built-in relationships to define derived properties.
Every derivation relationship can be easily defined by the application developers.
As consequences:
1. The simplicity of the resulting types makes the application quick to compile.
2. The simplicity of the resulting types makes the compiler error messages quite clear.
3. Application developers can define relationships for exotic properties or units of measurement.

Regarding the compilation error messages, let's see some examples.
If, using the crate UOM, we write the statement `let _: Length = Mass::new::<gram>(0.);`, which tries to assign a mass to a length, we get the following hard-to-understand compilation error message, shown in a 110-column terminal:

```text
error[E0308]: mismatched types
  --> src/main.rs:14:21
   |
14 |     let _: Length = Mass::new::<gram>(0.);
   |            ------   ^^^^^^^^^^^^^^^^^^^^^ expected `Z0`, found `PInt<UInt<..., ...>>`
   |            |
   |            expected due to this
   |
   = note: expected struct `Quantity<dyn Dimension<I = ..., J = ..., Kind = ..., L = ..., M = ..., N = ..., T
= ..., Th = ...>, ..., ...>` (`Z0`)
              found struct `Quantity<dyn Dimension<I = ..., J = ..., Kind = ..., L = ..., M = ..., N = ..., T
= ..., Th = ...>, ..., ...>` (`PInt<UInt<..., ...>>`)
```

Instead, using the library `measures`, if we write the statement `let _: Measure<Metre> = Measure::<Gram>::new(0.);`, which tries to assign a mass in grams to a length in metres, we get the following compilation error message:

```text
error[E0308]: mismatched types
  --> src/main.rs:50:29
   |
50 |     let _: Measure<Metre> = Measure::<Gram>::new(0.);
   |            --------------   ^^^^^^^^^^^^^^^^^^^^^^^^ expected `Measure<Metre>`, found `Measure<Gram>`
   |            |
   |            expected due to this
   |
   = note: expected struct `units::Measure<units::Metre>`
              found struct `units::Measure<units::Gram>`
```

From it, it is clear that unit `Gram` was used where unit `Metre` was expected (or vice versa).

Instead, if we write the statement `let _ = Measure::<Metre>::new(0.).convert::<Second>();`, that tries to convert a measure in metres into a measure in seconds, the generated error message begins with these lines:

```text
error[E0271]: type mismatch resolving `<Second as MeasurementUnit>::Property == Length`
    --> src/main.rs:10:49
     |
10   |     let _ = Measure::<Metre>::new(0.).convert::<Second>();
     |                                                 ^^^^^^ type mismatch resolving `<Second as MeasurementU
nit>::Property == Length`
     |
note: expected this to be `Length`
```

From it, it is enough clear that, for the destination unit of the conversion, a unit of `Length` was expected, but a unit of `Time` was specified.

## Meaningless operations for measure points

There are other features that makes this library useful to prevent programming errors.

Consider the following Rust code:

```rust
    let position1 = 1.2;
    let position2 = 2.3;
    let displacement1 = 0.3;
    let position3 = position1 + displacement1; // Meaningful.
    let what1 = position1 + position2; // Meaningless.
    let displacement2 = position2 - position1; // Meaningful.
    let what2 = position1 * 2.; // Meaningless.
```

The value of the variables `position1` and `position2` are meant to be positions of an thing along a line.
The value of the variable `displacement1` is meant to a movement of that thing along that line.

So, the value of the variable `position3` is also a position, obtained by incrementing position `position1` by displacement `displacement1`.
This is a meaningful operation.

Instead, the value of the variable `what1` is obtained by incrementing position `position1` by position `position12`.
Incrementing a position by another position is a meaningless operation, yet it is allowed by the compiler.

The value of the variable `displacement2` is obtained by computing the difference between position `position2` and position `position1`.
It represents the movement to go from `position1` to `position2`.
It is a meaningful operation.

Instead, the variable `what2` is obtained by multiplying by 2 position `position1`.
Multiplying a position by a number is a meaningless operation, yet it is allowed by the compiler.

These meaningless operations can avoided by using this code:

```rust
    let position1 = MeasurePoint::<Metre, f64>::new(1.2);
    let position2 = MeasurePoint::<Metre, f64>::new(2.3);
    let displacement1 = Measure::<Metre, f64>::new(0.3);
    let position3 = position1 + displacement1; // Allowed.
    let what1 = position1 + position2; // Compilation error.
    let displacement2 = position2 - position1; // Allowed.
    let what2 = position1 * 2.; // Compilation error.
```

In this code, displacements are encapsulated in objects of type `Measure`, and positions are encapsulated in objects of type `MeasurePoint`.
Using such types, the fifth statement, which attempts to add two positions, and the last statement, which attempts to multiply a position by a number, cause compilation errors.

## Different origins for measure points

Consider the following Rust code:

```rust
    let fahrenheit_temperature_variation = 3.;
    let celsius_temperature_variation = fahrenheit_temperature_variation
        / 9. * 5.; // Right.
    let fahrenheit_temperature_point = 70.;
    let celsius_temperature_point = fahrenheit_temperature_point
        / 9. * 5.; // Wrong; different scale origin.
```

To compute the value to assign to the variable `celsius_temperature_variation`, a temperature variation is correctly converted from the Fahrenheit scale to the Celsius scale.

Instead, to compute the value to assign to the variable `celsius_temperature_point`, the same formula is wrongly applied.

It is wrong, because this statements should convert between temperature points, not between temperature variations.
To convert between temperature points, the fact that these two scales have different origins should be taken into account.

This is avoided by using this code:

```rust
    let fahrenheit_temperature_variation = Measure::<Fahrenheit>::new(3.);
    let celsius_temperature_variation =
        fahrenheit_temperature_variation.convert::<Celsius>();
    let fahrenheit_temperature_point = MeasurePoint::<Fahrenheit>::new(70.);
    let celsius_temperature_point =
        fahrenheit_temperature_point.convert::<Celsius>();
```

Both unit conversions use the clause `.convert::<Celsius>()`; though, the second one is applied to an object of type `MeasurePoint`, and so that conversion takes into account the difference between the origins.

## Wrong derived units of measurement

Consider the following Rust code:

```rust
    let distance = 8.; // Miles.
    let time_elapsed = 2.; // Hours.

    // Wrong; this value is in miles per hour.
    let average_speed_in_metres_per_second = distance / time_elapsed;
```

The value of the variable `average_speed_in_metres_per_second` is computed in miles per hours, but it is wrongly assigned to a variable meant to keep a value in metres per second.

This is avoided by using this code:

```rust
    let distance = Measure::<Mile>::new(8.);
    let time_elapsed = Measure::<Hour>::new(2.);
    let average_speed1: Measure<MilePerHour>
        = distance / time_elapsed; // Allowed.
    let average_speed2 = distance / time_elapsed; // Allowed.
    let average_speed3: Measure<MetrePerSecond>
        = distance / time_elapsed; // Compilation error.
```

The compilation of the assignment to `average_speed1` checks that the value assigned has the same derived unit of measurement of the variable that receives that value.

The compilation of the assignment to `average_speed2` infers such unit of measurement.

The compilation of the assignment to `average_speed3` generates a type-mismatch compilation error.

## Plane (2D) And space (3D) measures

So far, we have seen features comparable to what is offered by most other libraries handling units of measurement.

Though, the library `measures` has some multi-dimensional features, which make it useful to work with bi-dimensional or three-dimensional quantities, in an ergonomic and safe manner.

Some physical or geometrical properties are properly described as multidimensional.

For example, a displacement in a plane has two dimensions (X and Y), and a displacement in space has three dimensions (X, Y, and Z).
They are called “vector measures”.
Plane vector measures (a.k.a. 2D measures) have two components, and space vector measures (a.k.a. 3D measures) have three components.

For vector measures, usually it is good to have the same unit of measurement and the same numeric type for all the components.
So, the library `measures` allows to encapsulate all the components of a vector measure into a single object, like `Measure3d<Metre, f64>`, in which all three components have `Metre` as unit of measurement and `f64` as inner type.

For example, to represent a planar force and a planar displacement, this code could be written:

```rust
    let force = (Measure::<Newton>::new(3.), Measure::<Newton>::new(4.));
    let displacement = (Measure::<Metre>::new(7.), Measure::<Metre>::new(1.));
```

but this other code is definitely better:

```rust
    let force = Measure2d::<Newton>::new([3., 4.]);
    let displacement = Measure2d::<Metre>::new([7., 1.]);
```

Similarly, it is possible to create 3D measures in space:

```rust
    let force = Measure3d::<Newton>::new([3., 4., 5.]);
    let displacement = Measure3d::<Metre>::new([7., 1., -6.]);
```

Also points in a plane or in space can be represented by single objects:

```rust
    let position_in_plane = MeasurePoint2d::<Metre>::new([3., 4.]);
    let position_in_space = MeasurePoint3d::<Metre>::new([3., 4., 5.]);
```

Dot products and cross products between measures have the right units of measurement:

```rust
   // In a plane
    let force = Measure2d::<Newton>::new([3., 4.]);
    let displacement = Measure2d::<Metre>::new([7., 1.]);
    let work: Measure<Joule> = force * displacement;
    use measures::traits::CrossProduct;
    let torque: Measure<NewtonMetre> = force.cross_product(displacement);
```

The value of `work` is obtained by multiplying `force`, a 2D force expressed in newtons, by `displacement`, a 2D displacement expressed in metres.
Such multiplication, which uses the "`*`" operator, is the *dot product* between the two 2D vectors.
So, the result is an energy (or work), expressed in joules.

The value of `torque` is obtained by computing the cross product between `force` and `displacement`, resulting in a torque, expressed in newton-metres.

Notice that the unit `Joule` and `NewtonMetre` are different types, even if they have the same dimensional composition (mass space space / time time).

Similarly, it happens in 3D space:

```rust
    // In space
    let force = Measure3d::<Newton>::new([3., 4., 5.]);
    let displacement = Measure3d::<Metre>::new([7., 1., -6.]);
    let work: Measure<Joule> = force * displacement;
    use measures::traits::CrossProduct;
    let torque: Measure3d<NewtonMetre> = force.cross_product(displacement);
```

Notice a difference: the 2D torque is a scalar (a `Measure`), while the 3D torque is a 3D vector (a `Measure3d`).

## Angles and directions

Angles deserve a special treatment.
Consider the following Rust code:

```rust
    let mut angular_position = 300.; // In degrees.
    let rotation = 400.; // In degrees.
    angular_position += rotation;
    assert_eq!(angular_position, 700.);
```

The third statement increments an angular position by a rotation.
The resulting position is 700 degrees, which is more than one cycle.

It may be just what we needed, but it may be that we needed instead a position in the circumference, i.e. a directional angle.
And in such a case, 700 degrees is not a valid result.

There are two widely used conventions to specify a direction: using an angle from 0 to 360 degrees, or an angle from -180 to +180 degrees.
The former case, which uses only non-negative values, is computed by the the following statement:

```rust
    let direction360 = angular_position % 360.;
    assert_eq!(direction360, 340.);
```

The other case, using positive or negative values, is computed by the following statement:

```rust
    let direction180 = (angular_position + 180.) % 360. - 180.;
    assert_eq!(direction180, -20.);
```

Though, these formulas are error-prone, and they depend on the angular unit of measurement.
These unsafe expressions are avoided by using this code:

```rust
    // An unconstrained angular position.
    let mut angular_position = MeasurePoint::<Degree>::new(300.);

    // A rotation.
    let rotation = Measure::<Degree>::new(400.);

    // The position is incremented by the rotation.
    angular_position += rotation;
    assert_eq!(angular_position.value, 700.);

    // Direction in the range [0..360].
    let unsigned_direction = UnsignedDirection::<Degree>::from_measure_point(angular_position);
    assert_eq!(unsigned_direction.value, 340.);

    // Direction in the range [-180..180].
    let signed_direction = SignedDirection::<Degree>::from_measure_point(angular_position);
    assert_eq!(signed_direction.value, -20.);
```

## Vector and affine transformations

When using vector measures in a plane or in the 3D space, it is quite typical to need to perform vector operations.
There are good linear algebra libraries, like the crate `nalgebra`, which could be used for this.
Though, when using them, it is not possible to keep the measurement unit information of components.
Consider the following Rust program, which uses the crate `nalgebra`:

```rust
extern crate nalgebra;
use nalgebra::{Rotation2, Vector2};
fn main() {
    let displacement = Vector2::new(4., 5.);
    let rotation = Rotation2::new(0.1);
    let rotated_displacement = rotation * displacement;
}
```

First, it stores a 2D measure into the variable `displacement`, with no units of measurement.
Then, it stores a 2D linear transformation into the variable `rotation`.
Then, such rotation is applied to the displacement, using a matrix multiplication, obtaining a transformed 2D measure.
Such resulting measure is stored into the variable `rotated_displacement`.
Of course, such a variable is meant to have the same unit of measurement of the first variable, yet this is not specified by the used types.

So, let’s try to attach units of measurement to these measures:

```rust
    let displacement = Vector2::new(
        Measure::<Metre>::new(4.),
        Measure::<Metre>::new(5.),
    );
    let rotation = Rotation2::new(Measure::<measures::angle::Radian>::new(0.1));
```

The first statement is allowed, but the second one is not allowed by `nalgebra`.
It generates the compilation error: ``the trait bound `units::Measure<Radian>: SimdRealField` is not satisfied``.

This happens because the `nalgebra` library  allows only values which implement the trait `SimdRealField`, as arguments of a call to `Rotation2::new`.
And the type `Measure` does not implement that trait.

Then, we can try to use a primitive number for the rotation:

```rust
    let displacement = Vector2::new(
        Measure::<Metre>::new(4.),
        Measure::<Metre>::new(5.),
    );
    let rotation = Rotation2::new(0.1);
    let rotated_displacement = rotation * displacement;
```

This causes a compilation error on the last statement, because `nalgebra` is not able to apply a linear transformation to a vector of `Measure` values, for a similar reason.
And even if it were able, probably the resulting value, which is assigned to `rotated_displacement`, wouldn’t have the proper unit of measurement.

Instead, by using the library `measures`, it is possible to compile the following code:

```rust
    // A 2D measure in metres.
    let displacement = Measure2d::<Metre>::new([4., 5.]);

    // A 2D linear transformation, corresponding
    // to a counterclockwise rotation on 0.1 radians.
    let rotation = LinearMap2d::rotation(
        Measure::<measures::angle::Radian>::new(0.1));

    // The rotation is applied to the planar measure,
    // obtaining a rotated measure,
    // having the same type of the original measure.
    let rotated_displacement = rotation.apply_to(displacement);
```

The above code uses the type `LinearMap2d` to transform objects of type `Measure2d`.
Instead, to transform objects of type `MeasurePoint2d`, the type `AffineMap2d` should be used.

And when working in three dimensions, the types `LinearMap3d` and `AffineMap3d` should be used.

So, when simple linear or affine transformations are needed, it is simpler to avoid including a linear algebra package in the project.
However, when more complex linear or affine transformations are needed, it is possible to include also a linear algebra package in the project, and to exchange data between that library and the library `measures`.

## Working with uncertainty

In experimental science and in engineering, you rarely handle exact measures.
Typically, every measure is affected by some _uncertainty_ (often improperly named "error").
This means that a measure is not a number, but a probability distribution.

Measuring something means to extract samples from this distribution.

In most cases, the uncertainty is much smaller than the average value of the measurement.
For example, you typically can say that a table is long 150 cm, plus or minus 1 cm.
You are not going to say that a table is long 150 cm, plus or minus 100 cm.

In such cases, the probability distribution can be well represented by a bell-shaped _normal_ distribution.
Such kind of distributions can be characterized by two numbers: the mean (μ) and the standard deviation (σ).

So, it is quite typical to express a length as `170 ± 1.5 cm`, or a mass as `87.3 ± 0.2 kg`.

We want to be able to represent such measures in a variable.
Of course, our variable will need to store two numbers.

A problem regarding uncertainty is its propagation.

For example, let's consider a physical object, for which we measured its mass as `87.3 ± 0.2 kg` and its volume as `0.3 ± 0.012 m³`.
We can say that, for its mass density, the most likely value is `87.3 / 0.3 = 291 kg/m³`.
But what about the uncertainty of such derived measure?

The library `measures` allows to define measures with an associated variance, that is the square of the standard deviation.
In addition, the library allows to perform arithmetic operations on such measures, propagating the uncertainty to the results.

The propagation of the uncertainty of measures follows rules depending on the statistical correlation of the measures involved in the operations.
The usual infix operators (`+`, `-`, `*`, `/`) assume that their operands are statistically independent (i.e. they have correlation equal to zero).

Though, in some applications, a non-zero correlation is a better representation of reality, and so, for every mathematical operation involving two measures, an alternate operation is provided, in which the correlation can be specified.
