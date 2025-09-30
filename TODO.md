## Mixed-unit operations with unit `One`

Currently, the compilation of the following code allows the first three lines, but on the fourth it emits ``error[E0277]: cannot multiply `Measure<One>` by `Measure<CentiMetre>`` and then ``no implementation for `Measure<One> * Measure<CentiMetre>``:
```
    let mo = Measure::<measures::dimensionless::One>::new(2.3);
    let m = Measure::<CentiMetre>::new(3.6);
    let r1 = m * mo;
    let r2 = mo * m;
```

## Uncertainty

Let `m1` and `m2` be measures with absolute uncertainties `d1` and `d2`, and with correlation between such uncertainties `c`.
Let `k` be `2 - c^2`.
We can define the absolute uncertainty of their addition or subtraction `d(m1 +- m2) = (d1^k + d2^k)^(1/k)`.
In particular, in the case of `c = 0`, we get `d(m1 +- m2) = (d1^2 + d2^2)^(1/2)`, which is the Pythagorean/Euclidean formula.
By letting `v` be `d^2`, in this case we get `v(m1 +- m2) = v1 + v2`.
Instead, in the general case, we have `v(m1 +- m2) = (v1^(k/2) + v2^(k/2))^(2/k)`.
We have also `k/2 = (2 - c^2)/2 = 1 - c^2/2`
and then `2/k = 2/(1 - c^2/2)`.
So, if we let `h` be `1 - c^2/2`, we have `v(m1 +- m2) = (v1^h + v2^h)^(1/h)`.
This formula, in the case of `c = 0`, and so `h = 1`, becomes `v1 + v2`, and in the case of `c = 1`, and so `h = 1/2`, becomes `(v1^(1/2) + v2^(1/2))^2`, i.e. `(d1 + d2)^2`, and therefore it implies that `d(m1 +- m2) = d1 + d2`.

Therefore, the addition and subtraction of two uncertain measures can be done only by specifying a third argument, that is the correlation coefficient.
The `Add::add` method simply assumes such a coefficient is `0`.
The more complete methods are instead:
```
fn add_with_correlation(&self, other: &Self, correlation: Number) -> Self {
    let h = Number::ONE - correlation * correlation * Number::HALF;
    Self::with_variance(self.value + other.value, (self.variance.powf(h) + other.variance.powf(h)).powf(Number::ONE / h))
}
```
and
```
fn subtract_with_correlation(&self, other: &Self, correlation: Number) -> Self {

}
```

Similar considerations apply for `Sub::sub`.

Instead, for multiplication and division, different formulas apply.

For `m1` and `m2`, the relative uncertainties are, respectively, `r1 = d1 / m1` and `r2 = d1 / m1`.
We can define the relative uncertainty of their multiplication or division `r(m1 */ m2) = ?`.

## Implemented traits

Consider whether the following traits should be implemented for the various defined generic types: `Copy`, `Clone`, `Default`, `Debug`, `Display`, `Send`, `Sync`, `PartialEq`, `PartialOrd`, `Hash`, `Eq`, `Ord`, `Serialize`, `Deserialize`.
In case, add the unit-tests which check the existence and behavior of such implementations.
