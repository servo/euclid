// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::Length;

use std::cmp::PartialEq;
use std::fmt;
use std::num::Zero;

#[deriving(Clone, Decodable, Encodable, PartialEq)]
pub struct Size2D<T> {
    pub width: T,
    pub height: T
}

impl<T: fmt::Show> fmt::Show for Size2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

pub fn Size2D<T: Clone>(width: T, height: T) -> Size2D<T> {
    Size2D {
        width: width,
        height: height
    }
}

impl<T:Clone + Mul<T,U>, U> Size2D<T> {
    pub fn area(&self) -> U { self.width * self.height }
}

impl<T:Clone + Zero> Size2D<T> {
    pub fn zero() -> Size2D<T> {
        Size2D {
            width: Zero::zero(),
            height: Zero::zero(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width.is_zero() || self.height.is_zero()
    }
}

impl<Scale, T0: Mul<Scale, T1>, T1: Clone> Mul<Scale, Size2D<T1>> for Size2D<T0> {
    #[inline]
    fn mul(&self, scale: &Scale) -> Size2D<T1> {
        Size2D(self.width * *scale, self.height * *scale)
    }
}

impl<Scale, T0: Div<Scale, T1>, T1: Clone> Div<Scale, Size2D<T1>> for Size2D<T0> {
    #[inline]
    fn div(&self, scale: &Scale) -> Size2D<T1> {
        Size2D(self.width / *scale, self.height / *scale)
    }
}

// Convenient aliases for Size2D with typed units

pub type TypedSize2D<Unit, T> = Size2D<Length<Unit, T>>;

pub fn TypedSize2D<Unit, T: Clone>(width: T, height: T) -> TypedSize2D<Unit, T> {
    Size2D(Length(width), Length(height))
}

impl<Unit, T: Clone> Size2D<Length<Unit, T>> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Size2D<T> {
        Size2D(self.width.get(), self.height.get())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Size2D<T>) -> TypedSize2D<Unit, T> {
        Size2D(Length(p.width.clone()), Length(p.height.clone()))
    }
}

impl<Unit, T0: NumCast + Clone, T1: NumCast + Clone> Size2D<Length<Unit, T0>> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast(&self) -> Option<Size2D<Length<Unit, T1>>> {
        match (self.width.cast(), self.height.cast()) {
            (Some(w), Some(h)) => Some(Size2D(w, h)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<Unit, T: NumCast + Clone> Size2D<Length<Unit, T>> {
    pub fn as_f32(&self) -> Size2D<Length<Unit, f32>> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> Size2D<Length<Unit, uint>> {
        self.cast().unwrap()
    }
}
