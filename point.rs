// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::Length;

use std::fmt;
use std::num::Zero;

#[deriving(Clone, Decodable, Encodable, PartialEq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T
}

impl<T: Zero + Clone> Zero for Point2D<T> {
    fn zero() -> Point2D<T> {
        Point2D { x: Zero::zero(), y: Zero::zero() }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: fmt::Show> fmt::Show for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn Point2D<T:Clone>(x: T, y: T) -> Point2D<T> {
    Point2D {x: x, y: y}
}


impl<T:Clone + Add<T,T>> Add<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn add(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
}

impl<T:Clone + Sub<T,T>> Sub<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn sub(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

impl<Scale, T0: Mul<Scale, T1>, T1: Clone> Mul<Scale, Point2D<T1>> for Point2D<T0> {
    #[inline]
    fn mul(&self, scale: &Scale) -> Point2D<T1> {
        Point2D(self.x * *scale, self.y * *scale)
    }
}

impl<Scale, T0: Div<Scale, T1>, T1: Clone> Div<Scale, Point2D<T1>> for Point2D<T0> {
    #[inline]
    fn div(&self, scale: &Scale) -> Point2D<T1> {
        Point2D(self.x / *scale, self.y / *scale)
    }
}

// Convenient aliases for Point2D with typed units

pub type TypedPoint2D<Unit, T> = Point2D<Length<Unit, T>>;

pub fn TypedPoint2D<Unit, T: Clone>(x: T, y: T) -> TypedPoint2D<Unit, T> {
    Point2D(Length(x), Length(y))
}

impl<Unit, T: Clone> Point2D<Length<Unit, T>> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Point2D<T> {
        Point2D(self.x.get(), self.y.get())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Point2D<T>) -> TypedPoint2D<Unit, T> {
        Point2D(Length(p.x.clone()), Length(p.y.clone()))
    }
}

impl<Unit, T0: NumCast + Clone, T1: NumCast + Clone> Point2D<Length<Unit, T0>> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast(&self) -> Option<Point2D<Length<Unit, T1>>> {
        match (self.x.cast(), self.y.cast()) {
            (Some(x), Some(y)) => Some(Point2D(x, y)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<Unit, T: NumCast + Clone> Point2D<Length<Unit, T>> {
    pub fn as_f32(&self) -> Point2D<Length<Unit, f32>> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> Point2D<Length<Unit, uint>> {
        self.cast().unwrap()
    }
}
