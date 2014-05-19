// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A one-dimensional length, tagged with its units.

use scale_factor::ScaleFactor;

use std::num::{cast, Zero};

/// A one-dimensional distance, with value represented by `T` and unit of measurement `Unit`.
///
/// `T` can be any numerical type, for example a primitive type like uint64 or f32.
///
/// `Unit` is not used in the representation of a Length value. It is used only at compile-time
/// to ensure that a Length stored with one unit is converted explicitly before being used in an
/// expression that requires a different unit.  It may be a type without values, such as an empty
/// enum.
///
/// You can multiply a Length by a `scale_factor::ScaleFactor` to convert it from one unit to
/// another.  See the ScaleFactor docs for an example.
#[deriving(Decodable, Encodable, Show)]
pub struct Length<Unit, T>(pub T);

// *length
impl<Unit, T> Deref<T> for Length<Unit, T> {
    fn deref<'a>(&'a self) -> &'a T {
        match *self {
            Length(ref x) => x
        }
    }
}

// length + length
impl<U, T: Add<T, T>> Add<Length<U, T>, Length<U, T>> for Length<U, T> {
    fn add(&self, other: &Length<U, T>) -> Length<U, T> {
        Length(**self + **other)
    }
}

// length - length
impl<U, T: Sub<T, T>> Sub<Length<U, T>, Length<U, T>> for Length<U, T> {
    fn sub(&self, other: &Length<U, T>) -> Length<U, T> {
        Length(**self - **other)
    }
}

// length * scaleFactor
impl<Src, Dst, T: Mul<f32, T>> Mul<ScaleFactor<Src, Dst>, Length<Dst, T>> for Length<Src, T> {
    #[inline]
    fn mul(&self, scale: &ScaleFactor<Src, Dst>) -> Length<Dst, T> {
        Length(**self * **scale)
    }
}

// length / scaleFactor
impl<Src, Dst, T: Div<f32, T>> Div<ScaleFactor<Src, Dst>, Length<Src, T>> for Length<Dst, T> {
    #[inline]
    fn div(&self, scale: &ScaleFactor<Src, Dst>) -> Length<Src, T> {
        Length(**self / **scale)
    }
}

impl<Unit, T0: NumCast + Clone, T1: NumCast + Clone> Length<Unit, T0> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast(&self) -> Option<Length<Unit, T1>> {
        cast((**self).clone()).map(|x| Length(x))
    }
}

// FIXME: Switch to `deriving(Clone, Eq, Ord, Zero)` after this Rust issue is fixed:
// https://github.com/mozilla/rust/issues/7671

impl<Unit, T: Clone> Clone for Length<Unit, T> {
    fn clone(&self) -> Length<Unit, T> {
        Length((**self).clone())
    }
}

impl<Unit, T: Eq> Eq for Length<Unit, T> {
    fn eq(&self, other: &Length<Unit, T>) -> bool { (**self).eq(&**other) }
}

impl<Unit, T: Ord> Ord for Length<Unit, T> {
    fn lt(&self, other: &Length<Unit, T>) -> bool { (**self).lt(&**other) }
    fn le(&self, other: &Length<Unit, T>) -> bool { (**self).le(&**other) }
    fn gt(&self, other: &Length<Unit, T>) -> bool { (**self).gt(&**other) }
    fn ge(&self, other: &Length<Unit, T>) -> bool { (**self).ge(&**other) }
}

impl<Unit, T: TotalEq> TotalEq for Length<Unit, T> {}

impl<Unit, T: TotalOrd> TotalOrd for Length<Unit, T> {
    fn cmp(&self, other: &Length<Unit, T>) -> Ordering { (**self).cmp(&**other) }
}

impl<Unit, T: Zero> Zero for Length<Unit, T> {
    fn zero() -> Length<Unit, T> {
        Length(Zero::zero())
    }

    fn is_zero(&self) -> bool {
        (**self).is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::Length;
    use scale_factor::ScaleFactor;
    use std::num::Zero;

    #[deriving(Show)]
    enum Inch {}
    #[deriving(Show)]
    enum Mm {}

    #[test]
    fn test_length() {
        let mm_per_inch: ScaleFactor<Inch, Mm> = ScaleFactor(25.4);

        let one_foot: Length<Inch, f32> = Length(12.0);
        let two_feet = one_foot + one_foot;
        let zero_feet = one_foot - one_foot;

        assert_eq!(*one_foot, 12.0);
        assert_eq!(*two_feet, 24.0);
        assert!(zero_feet.is_zero());

        assert!(one_foot == one_foot);
        assert!(two_feet != one_foot);

        assert!(zero_feet <  one_foot);
        assert!(zero_feet <= one_foot);
        assert!(two_feet  >  one_foot);
        assert!(two_feet  >= one_foot);

        assert!(  two_feet <= two_feet);
        assert!(  two_feet >= two_feet);
        assert!(!(two_feet >  two_feet));
        assert!(!(two_feet <  two_feet));

        let one_foot_in_mm: Length<Mm, f32> = one_foot * mm_per_inch;

        assert_eq!(*one_foot_in_mm, 304.8);

        let back_to_inches: Length<Inch, f32> = one_foot_in_mm / mm_per_inch;
        assert_eq!(one_foot, back_to_inches);

        let int_foot: Length<Inch, int> = one_foot.cast().unwrap();
        assert_eq!(*int_foot, 12);
    }
}
