// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A type-checked scaling factor between units.

/// A scaling factor between two different units of measurement.
///
/// This is effectively a type-safe float, intended to be used in combination with other types like
/// `length::Length` to enforce conversion between systems of measurement at compile time.
///
/// `Src` and `Dst` represent the units before and after multiplying a value by a ScaleFactor.  They
/// may be types without values, such as empty enums.  For example:
///
/// ```rust
/// enum Mm {};
/// enum Inch {};
///
/// let mm_per_inch: ScaleFactor<Inch, Mm> = ScaleFactor(25.4);
///
/// let one_foot: Length<Inch, f32> = Length(12.0);
/// let one_foot_in_mm: Length<Mm, f32> = one_foot * mm_per_inch;
/// ```
#[deriving(Clone, Decodable, Encodable)]
pub struct ScaleFactor<Src, Dst>(pub f32);

// *scale
impl<Src, Dst> Deref<f32> for ScaleFactor<Src, Dst> {
    fn deref<'a>(&'a self) -> &'a f32 {
        match *self {
            ScaleFactor(ref x) => x
        }
    }
}

impl<Src, Dst> ScaleFactor<Src, Dst> {
    /// The inverse ScaleFactor (1.0 / self).
    pub fn inv(&self) -> ScaleFactor<Dst, Src> {
        ScaleFactor(1.0 / **self)
    }
}

// scale0 * scale1
impl<A,B,C> Mul<ScaleFactor<B, C>, ScaleFactor<A, C>> for ScaleFactor<A, B> {
    #[inline]
    fn mul(&self, other: &ScaleFactor<B, C>) -> ScaleFactor<A, C> {
        ScaleFactor(**self * **other)
    }
}

// scale0 + scale1
impl<Src, Dst> Add<ScaleFactor<Src, Dst>, ScaleFactor<Src, Dst>> for ScaleFactor<Src, Dst> {
    #[inline]
    fn add(&self, other: &ScaleFactor<Src, Dst>) -> ScaleFactor<Src, Dst> {
        ScaleFactor(**self + **other)
    }
}

// scale0 - scale1
impl<Src, Dst> Sub<ScaleFactor<Src, Dst>, ScaleFactor<Src, Dst>> for ScaleFactor<Src, Dst> {
    #[inline]
    fn sub(&self, other: &ScaleFactor<Src, Dst>) -> ScaleFactor<Src, Dst> {
        ScaleFactor(**self - **other)
    }
}

// FIXME: Switch to `deriving(Eq)` after this Rust issue is fixed:
// https://github.com/mozilla/rust/issues/7671

impl<Src, Dst> Eq for ScaleFactor<Src, Dst> {
    fn eq(&self, other: &ScaleFactor<Src, Dst>) -> bool { (**self).eq(&**other) }
}
