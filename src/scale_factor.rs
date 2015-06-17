// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A type-checked scaling factor between units.

use num::One;

use num_lib::NumCast;
use std::ops::{Add, Mul, Sub, Div};
use std::marker::PhantomData;

/// A scaling factor between two different units of measurement.
///
/// This is effectively a type-safe float, intended to be used in combination with other types like
/// `length::Length` to enforce conversion between systems of measurement at compile time.
///
/// `Src` and `Dst` represent the units before and after multiplying a value by a ScaleFactor.  They
/// may be types without values, such as empty enums.  For example:
///
/// ```rust
/// use euclid::scale_factor::ScaleFactor;
/// use euclid::length::Length;
/// enum Mm {};
/// enum Inch {};
///
/// let mm_per_inch: ScaleFactor<Inch, Mm, f32> = ScaleFactor::new(25.4);
///
/// let one_foot: Length<Inch, f32> = Length::new(12.0);
/// let one_foot_in_mm: Length<Mm, f32> = one_foot * mm_per_inch;
/// ```
#[derive(Copy, RustcDecodable, RustcEncodable, Debug)]
pub struct ScaleFactor<Src, Dst, T>(pub T, PhantomData<(Src, Dst)>);

impl<Src, Dst, T> ScaleFactor<Src, Dst, T> {
    pub fn new(x: T) -> ScaleFactor<Src, Dst, T> {
        ScaleFactor(x, PhantomData)
    }
}

impl<Src, Dst, T: Clone> ScaleFactor<Src, Dst, T> {
    pub fn get(&self) -> T {
        self.0.clone()
    }
}

impl<Src, Dst, T: Clone + One + Div<T, Output=T>> ScaleFactor<Src, Dst, T> {
    /// The inverse ScaleFactor (1.0 / self).
    pub fn inv(&self) -> ScaleFactor<Dst, Src, T> {
        let one: T = One::one();
        ScaleFactor::new(one / self.get())
    }
}

// scale0 * scale1
impl<A, B, C, T: Clone + Mul<T, Output=T>>
Mul<ScaleFactor<B, C, T>> for ScaleFactor<A, B, T> {
    type Output = ScaleFactor<A, C, T>;
    #[inline]
    fn mul(self, other: ScaleFactor<B, C, T>) -> ScaleFactor<A, C, T> {
        ScaleFactor::new(self.get() * other.get())
    }
}

// scale0 + scale1
impl<Src, Dst, T: Clone + Add<T, Output=T>> Add for ScaleFactor<Src, Dst, T> {
    type Output = ScaleFactor<Src, Dst, T>;
    #[inline]
    fn add(self, other: ScaleFactor<Src, Dst, T>) -> ScaleFactor<Src, Dst, T> {
        ScaleFactor::new(self.get() + other.get())
    }
}

// scale0 - scale1
impl<Src, Dst, T: Clone + Sub<T, Output=T>> Sub for ScaleFactor<Src, Dst, T> {
    type Output = ScaleFactor<Src, Dst, T>;
    #[inline]
    fn sub(self, other: ScaleFactor<Src, Dst, T>) -> ScaleFactor<Src, Dst, T> {
        ScaleFactor::new(self.get() - other.get())
    }
}

impl<Src, Dst, T0: NumCast + Clone> ScaleFactor<Src, Dst, T0> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast<T1: NumCast + Clone>(&self) -> Option<ScaleFactor<Src, Dst, T1>> {
        NumCast::from(self.get()).map(ScaleFactor::new)
    }
}

// FIXME: Switch to `derive(PartialEq, Clone)` after this Rust issue is fixed:
// https://github.com/mozilla/rust/issues/7671

impl<Src, Dst, T: Clone + PartialEq> PartialEq for ScaleFactor<Src, Dst, T> {
    fn eq(&self, other: &ScaleFactor<Src, Dst, T>) -> bool {
        self.get().eq(&other.get())
    }
}

impl<Src, Dst, T: Clone> Clone for ScaleFactor<Src, Dst, T> {
    fn clone(&self) -> ScaleFactor<Src, Dst, T> {
        ScaleFactor::new(self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::ScaleFactor;

    #[derive(Debug)]
    enum Inch {}
    #[derive(Debug)]
    enum Cm {}
    #[derive(Debug)]
    enum Mm {}

    #[test]
    fn test_scale_factor() {
        let mm_per_inch: ScaleFactor<Inch, Mm, f32> = ScaleFactor::new(25.4);
        let cm_per_mm: ScaleFactor<Mm, Cm, f32> = ScaleFactor::new(0.1);

        let mm_per_cm: ScaleFactor<Cm, Mm, f32> = cm_per_mm.inv();
        assert_eq!(mm_per_cm.get(), 10.0);

        let cm_per_inch: ScaleFactor<Inch, Cm, f32> = mm_per_inch * cm_per_mm;
        assert_eq!(cm_per_inch, ScaleFactor::new(2.54));

        let a: ScaleFactor<Inch, Inch, isize> = ScaleFactor::new(2);
        let b: ScaleFactor<Inch, Inch, isize> = ScaleFactor::new(3);
        assert!(a != b);
        assert_eq!(a, a.clone());
        assert_eq!(a.clone() + b.clone(), ScaleFactor::new(5));
        assert_eq!(a - b, ScaleFactor::new(-1));
    }
}
