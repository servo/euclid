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
use num::Zero;

use num_traits::NumCast;
#[cfg(feature = "plugins")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign};
use std::marker::PhantomData;

/// A one-dimensional distance, with value represented by `T` and unit of measurement `Unit`.
///
/// `T` can be any numeric type, for example a primitive type like u64 or f32.
///
/// `Unit` is not used in the representation of a Length value. It is used only at compile time
/// to ensure that a Length stored with one unit is converted explicitly before being used in an
/// expression that requires a different unit.  It may be a type without values, such as an empty
/// enum.
///
/// You can multiply a Length by a `scale_factor::ScaleFactor` to convert it from one unit to
/// another.  See the ScaleFactor docs for an example.
// Uncomment the derive, and remove the macro call, once heapsize gets
// PhantomData<T> support.
#[derive(Copy, RustcDecodable, RustcEncodable, Debug)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
pub struct Length<Unit, T>(pub T, PhantomData<Unit>);

#[cfg(feature = "plugins")]
impl<Unit,T> Deserialize for Length<Unit,T> where T: Deserialize {
    fn deserialize<D>(deserializer: &mut D) -> Result<Length<Unit,T>,D::Error>
                      where D: Deserializer {
        Ok(Length(try!(Deserialize::deserialize(deserializer)), PhantomData))
    }
}

#[cfg(feature = "plugins")]
impl<Unit,T> Serialize for Length<Unit,T> where T: Serialize {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(),S::Error> where S: Serializer {
        self.0.serialize(serializer)
    }
}

impl<Unit, T> Length<Unit, T> {
    pub fn new(x: T) -> Length<Unit, T> {
        Length(x, PhantomData)
    }
}

impl<Unit, T: Clone> Length<Unit, T> {
    pub fn get(&self) -> T {
        self.0.clone()
    }
}

// length + length
impl<U, T: Clone + Add<T, Output=T>> Add for Length<U, T> {
    type Output = Length<U, T>;
    fn add(self, other: Length<U, T>) -> Length<U, T> {
        Length::new(self.get() + other.get())
    }
}

// length += length
impl<U, T: Clone + AddAssign<T>> AddAssign for Length<U, T> {
    fn add_assign(&mut self, other: Length<U, T>) {
        self.0 += other.get();
    }
}

// length - length
impl<U, T: Clone + Sub<T, Output=T>> Sub<Length<U, T>> for Length<U, T> {
    type Output = Length<U, T>;
    fn sub(self, other: Length<U, T>) -> <Self as Sub>::Output {
        Length::new(self.get() - other.get())
    }
}

// length -= length
impl<U, T: Clone + SubAssign<T>> SubAssign for Length<U, T> {
    fn sub_assign(&mut self, other: Length<U, T>) {
        self.0 -= other.get();
    }
}

// length / length
impl<Src, Dst, T: Clone + Div<T, Output=T>> Div<Length<Src, T>> for Length<Dst, T> {
    type Output = ScaleFactor<Src, Dst, T>;
    #[inline]
    fn div(self, other: Length<Src, T>) -> ScaleFactor<Src, Dst, T> {
        ScaleFactor::new(self.get() / other.get())
    }
}

// length * scaleFactor
impl<Src, Dst, T: Clone + Mul<T, Output=T>> Mul<ScaleFactor<Src, Dst, T>> for Length<Src, T> {
    type Output = Length<Dst, T>;
    #[inline]
    fn mul(self, scale: ScaleFactor<Src, Dst, T>) -> Length<Dst, T> {
        Length::new(self.get() * scale.get())
    }
}

// length / scaleFactor
impl<Src, Dst, T: Clone + Div<T, Output=T>> Div<ScaleFactor<Src, Dst, T>> for Length<Dst, T> {
    type Output = Length<Src, T>;
    #[inline]
    fn div(self, scale: ScaleFactor<Src, Dst, T>) -> Length<Src, T> {
        Length::new(self.get() / scale.get())
    }
}

// -length
impl <U, T:Clone + Neg<Output=T>> Neg for Length<U, T> {
    type Output = Length<U, T>;
    #[inline]
    fn neg(self) -> Length<U, T> {
        Length::new(-self.get())
    }
}

impl<Unit, T0: NumCast + Clone> Length<Unit, T0> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast<T1: NumCast + Clone>(&self) -> Option<Length<Unit, T1>> {
        NumCast::from(self.get()).map(Length::new)
    }
}

// FIXME: Switch to `derive(Clone, PartialEq, PartialOrd, Zero)` after this Rust issue is fixed:
// https://github.com/mozilla/rust/issues/7671

impl<Unit, T: Clone> Clone for Length<Unit, T> {
    fn clone(&self) -> Length<Unit, T> {
        Length::new(self.get())
    }
}

impl<Unit, T: Clone + PartialEq> PartialEq for Length<Unit, T> {
    fn eq(&self, other: &Length<Unit, T>) -> bool { self.get().eq(&other.get()) }
}

impl<Unit, T: Clone + PartialOrd> PartialOrd for Length<Unit, T> {
    fn partial_cmp(&self, other: &Length<Unit, T>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<Unit, T: Clone + Eq> Eq for Length<Unit, T> {}

impl<Unit, T: Clone + Ord> Ord for Length<Unit, T> {
    fn cmp(&self, other: &Length<Unit, T>) -> Ordering { self.get().cmp(&other.get()) }
}

impl<Unit, T: Zero> Zero for Length<Unit, T> {
    fn zero() -> Length<Unit, T> {
        Length::new(Zero::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::Length;
    use scale_factor::ScaleFactor;

    #[derive(Debug, Copy, Clone)]
    enum Inch {}
    #[derive(Debug, Copy, Clone)]
    enum Mm {}

    #[test]
    fn test_length() {
        let mm_per_inch: ScaleFactor<Inch, Mm, f32> = ScaleFactor::new(25.4);

        let one_foot: Length<Inch, f32> = Length::new(12.0);
        let two_feet = one_foot.clone() + one_foot.clone();
        let zero_feet = one_foot.clone() - one_foot.clone();

        assert_eq!(one_foot.get(), 12.0);
        assert_eq!(two_feet.get(), 24.0);
        assert_eq!(zero_feet.get(), 0.0);

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

        assert_eq!(one_foot_in_mm, Length::new(304.8));
        assert_eq!(one_foot_in_mm / one_foot, mm_per_inch);

        let back_to_inches: Length<Inch, f32> = one_foot_in_mm / mm_per_inch;
        assert_eq!(one_foot, back_to_inches);

        let int_foot: Length<Inch, isize> = one_foot.cast().unwrap();
        assert_eq!(int_foot.get(), 12);

        let negative_one_foot = -one_foot;
        assert_eq!(negative_one_foot.get(), -12.0);

        let negative_two_feet = -two_feet;
        assert_eq!(negative_two_feet.get(), -24.0);

        let zero_feet: Length<Inch, f32> = Length::new(0.0);
        let negative_zero_feet = -zero_feet;
        assert_eq!(negative_zero_feet.get(), 0.0);
    }

    #[test]
    fn test_addassign() {
        let one_cm: Length<Mm, f32> = Length::new(10.0);
        let mut measurement: Length<Mm, f32> = Length::new(5.0);

        measurement += one_cm;

        assert_eq!(measurement.get(), 15.0);
    }

    #[test]
    fn test_subassign() {
        let one_cm: Length<Mm, f32> = Length::new(10.0);
        let mut measurement: Length<Mm, f32> = Length::new(5.0);

        measurement -= one_cm;

        assert_eq!(measurement.get(), -5.0);
    }
}
