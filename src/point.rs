// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::Length;
use size::Size2D;
use num::Zero;

use num_lib::NumCast;
use std::fmt::{self, Formatter};
use std::ops::{Add, Neg, Mul, Sub, Div};

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T
}

impl<T: Zero> Point2D<T> {
    pub fn zero() -> Point2D<T> {
        Point2D { x: Zero::zero(), y: Zero::zero() }
    }
}

impl<T: fmt::Debug> fmt::Debug for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

impl<T: fmt::Display> fmt::Display for Point2D<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.x, self.y)
    }
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D {x: x, y: y}
    }
}


impl<T:Clone + Add<T, Output=T>> Add for Point2D<T> {
    type Output = Point2D<T>;
    fn add(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T:Clone + Add<T, Output=T>> Add<Size2D<T>> for Point2D<T> {
    type Output = Point2D<T>;
    fn add(self, other: Size2D<T>) -> Point2D<T> {
        Point2D::new(self.x + other.width, self.y + other.height)
    }
}

impl<T: Copy + Add<T, Output=T>> Point2D<T> {
    pub fn add_size(&self, other: &Size2D<T>) -> Point2D<T> {
        Point2D { x: self.x + other.width, y: self.y + other.height }
    }
}

impl<T:Clone + Sub<T, Output=T>> Sub for Point2D<T> {
    type Output = Point2D<T>;
    fn sub(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

impl <T:Clone + Neg<Output=T>> Neg for Point2D<T> {
    type Output = Point2D<T>;
    #[inline]
    fn neg(self) -> Point2D<T> {
        Point2D::new(-self.x, -self.y)
    }
}

impl<Scale: Copy, T0: Mul<Scale, Output=T1>, T1: Clone> Mul<Scale> for Point2D<T0> {
    type Output = Point2D<T1>;
    #[inline]
    fn mul(self, scale: Scale) -> Point2D<T1> {
        Point2D::new(self.x * scale, self.y * scale)
    }
}

impl<Scale: Copy, T0: Div<Scale, Output=T1>, T1: Clone> Div<Scale> for Point2D<T0> {
    type Output = Point2D<T1>;
    #[inline]
    fn div(self, scale: Scale) -> Point2D<T1> {
        Point2D::new(self.x / scale, self.y / scale)
    }
}

// Convenient aliases for Point2D with typed units

pub type TypedPoint2D<Unit, T> = Point2D<Length<Unit, T>>;

impl<Unit, T: Clone> Point2D<Length<Unit, T>> {
    pub fn typed(x: T, y: T) -> TypedPoint2D<Unit, T> {
        Point2D::new(Length::new(x), Length::new(y))
    }

    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Point2D<T> {
        Point2D::new(self.x.get(), self.y.get())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Point2D<T>) -> TypedPoint2D<Unit, T> {
        Point2D::new(Length::new(p.x.clone()), Length::new(p.y.clone()))
    }
}

impl<Unit, T0: NumCast + Clone> Point2D<Length<Unit, T0>> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast<T1: NumCast + Clone>(&self) -> Option<Point2D<Length<Unit, T1>>> {
        match (self.x.cast(), self.y.cast()) {
            (Some(x), Some(y)) => Some(Point2D::new(x, y)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<Unit, T: NumCast + Clone> Point2D<Length<Unit, T>> {
    pub fn as_f32(&self) -> Point2D<Length<Unit, f32>> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> Point2D<Length<Unit, usize>> {
        self.cast().unwrap()
    }
}

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Zero> Point3D<T> {
    #[inline]
    pub fn zero() -> Point3D<T> {
        Point3D { x: Zero::zero(), y: Zero::zero(), z: Zero::zero() }
    }
}

impl<T: fmt::Debug> fmt::Debug for Point3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
    }
}

impl<T: fmt::Display> fmt::Display for Point3D<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T> Point3D<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D {x: x, y: y, z: z}
    }
}

impl<T:Clone + Add<T, Output=T>> Add for Point3D<T> {
    type Output = Point3D<T>;
    fn add(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(self.x + other.x,
                     self.y + other.y,
                     self.z + other.z)
    }
}

impl<T:Clone + Sub<T, Output=T>> Sub for Point3D<T> {
    type Output = Point3D<T>;
    fn sub(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(self.x - other.x,
                     self.y - other.y,
                     self.z - other.z)
    }
}

impl <T:Clone + Neg<Output=T>> Neg for Point3D<T> {
    type Output = Point3D<T>;
    #[inline]
    fn neg(self) -> Point3D<T> {
        Point3D::new(-self.x, -self.y, -self.z)
    }
}
