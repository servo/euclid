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

use num_traits::{Float, NumCast};
use std::fmt;
use std::ops::{Add, Neg, Mul, Sub, Div};

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
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

impl<T: Mul<T, Output=T> +
        Add<T, Output=T> +
        Sub<T, Output=T> +
        Copy> Point2D<T> {
    #[inline]
    pub fn dot(self, other: Point2D<T>) -> T {
        self.x * other.x +
        self.y * other.y
    }

    #[inline]
    pub fn cross(self, other: Point2D<T>) -> T {
        self.x * other.y - self.y * other.x
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

impl<T: Float> Point2D<T> {
    pub fn min(self, other: Point2D<T>) -> Point2D<T> {
         Point2D::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(self, other: Point2D<T>) -> Point2D<T> {
        Point2D::new(self.x.max(other.x), self.y.max(other.y))
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

impl<Unit, T: Clone> TypedPoint2D<Unit, T> {
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
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
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

impl<T: Mul<T, Output=T> +
        Add<T, Output=T> +
        Sub<T, Output=T> +
        Copy> Point3D<T> {
    #[inline]
    pub fn dot(self, other: Point3D<T>) -> T {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Point3D<T>) -> Point3D<T> {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

impl<T: Float> Point3D<T> {
    pub fn min(self, other: Point3D<T>) -> Point3D<T> {
         Point3D::new(self.x.min(other.x), self.y.min(other.y),
                      self.z.min(other.z))
    }

    pub fn max(self, other: Point3D<T>) -> Point3D<T> {
        Point3D::new(self.x.max(other.x), self.y.max(other.y),
                     self.z.max(other.z))
    }
}

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
pub struct Point4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Zero> Point4D<T> {
    #[inline]
    pub fn zero() -> Point4D<T> {
        Point4D {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
            w: Zero::zero()
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Point4D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?},{:?})", self.x, self.y, self.z, self.w)
    }
}

impl<T: fmt::Display> fmt::Display for Point4D<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}

impl<T> Point4D<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Point4D<T> {
        Point4D {x: x, y: y, z: z, w: w}
    }
}

impl<T:Clone + Add<T, Output=T>> Add for Point4D<T> {
    type Output = Point4D<T>;
    fn add(self, other: Point4D<T>) -> Point4D<T> {
        Point4D::new(self.x + other.x,
                     self.y + other.y,
                     self.z + other.z,
                     self.w + other.w)
    }
}

impl<T:Clone + Sub<T, Output=T>> Sub for Point4D<T> {
    type Output = Point4D<T>;
    fn sub(self, other: Point4D<T>) -> Point4D<T> {
        Point4D::new(self.x - other.x,
                     self.y - other.y,
                     self.z - other.z,
                     self.w - other.w)
    }
}

impl <T:Clone + Neg<Output=T>> Neg for Point4D<T> {
    type Output = Point4D<T>;
    #[inline]
    fn neg(self) -> Point4D<T> {
        Point4D::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T: Float> Point4D<T> {
    pub fn min(self, other: Point4D<T>) -> Point4D<T> {
         Point4D::new(self.x.min(other.x), self.y.min(other.y),
                      self.z.min(other.z), self.w.min(other.w))
    }

    pub fn max(self, other: Point4D<T>) -> Point4D<T> {
        Point4D::new(self.x.max(other.x), self.y.max(other.y),
                     self.z.max(other.z), self.w.max(other.w))
    }
}


#[cfg(test)]
mod point2d {
    use super::Point2D;

    #[test]
    pub fn test_scalar_mul() {
        let p1 = Point2D::new(3.0, 5.0);

        let result = p1 * 5.0;

        assert_eq!(result, Point2D::new(15.0, 25.0));
    }

    #[test]
    pub fn test_dot() {
        let p1 = Point2D::new(2.0, 7.0);
        let p2 = Point2D::new(13.0, 11.0);
        assert_eq!(p1.dot(p2), 103.0);
    }

    #[test]
    pub fn test_cross() {
        let p1 = Point2D::new(4.0, 7.0);
        let p2 = Point2D::new(13.0, 8.0);
        let r = p1.cross(p2);
        assert_eq!(r, -59.0);
    }

    #[test]
    pub fn test_min() {
        let p1 = Point2D::new(1.0, 3.0);
        let p2 = Point2D::new(2.0, 2.0);

        let result = p1.min(p2);

        assert_eq!(result, Point2D::new(1.0, 2.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point2D::new(1.0, 3.0);
        let p2 = Point2D::new(2.0, 2.0);

        let result = p1.max(p2);

        assert_eq!(result, Point2D::new(2.0, 3.0));
    }
}

#[cfg(test)]
mod typedpoint2d {
    use super::TypedPoint2D;
    use scale_factor::ScaleFactor;

    #[derive(Debug, Copy, Clone)]
    pub enum Mm {}
    #[derive(Debug, Copy, Clone)]
    pub enum Cm {}

    pub type Point2DMm<T> = TypedPoint2D<Mm, T>;
    pub type Point2DCm<T> = TypedPoint2D<Cm, T>;

    #[test]
    pub fn test_add() {
        let p1 = Point2DMm::typed(1.0, 2.0);
        let p2 = Point2DMm::typed(3.0, 4.0);

        let result = p1 + p2;

        assert_eq!(result, Point2DMm::typed(4.0, 6.0));
    }

    #[test]
    pub fn test_scalar_mul() {
        let p1 = Point2DMm::typed(1.0, 2.0);
        let cm_per_mm: ScaleFactor<Mm, Cm, f32> = ScaleFactor::new(0.1);

        let result = p1 * cm_per_mm;

        assert_eq!(result, Point2DCm::typed(0.1, 0.2));
    }
}

#[cfg(test)]
mod point3d {
    use super::Point3D;

    #[test]
    pub fn test_dot() {
        let p1 = Point3D::new(7.0, 21.0, 32.0);
        let p2 = Point3D::new(43.0, 5.0, 16.0);
        assert_eq!(p1.dot(p2), 918.0);
    }

    #[test]
    pub fn test_cross() {
        let p1 = Point3D::new(4.0, 7.0, 9.0);
        let p2 = Point3D::new(13.0, 8.0, 3.0);
        let p3 = p1.cross(p2);
        assert_eq!(p3, Point3D::new(-51.0, 105.0, -59.0));
    }

    #[test]
    pub fn test_min() {
        let p1 = Point3D::new(1.0, 3.0, 5.0);
        let p2 = Point3D::new(2.0, 2.0, -1.0);

        let result = p1.min(p2);

        assert_eq!(result, Point3D::new(1.0, 2.0, -1.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point3D::new(1.0, 3.0, 5.0);
        let p2 = Point3D::new(2.0, 2.0, -1.0);

        let result = p1.max(p2);

        assert_eq!(result, Point3D::new(2.0, 3.0, 5.0));
    }
}

#[cfg(test)]
mod point4d {
    use super::Point4D;

    #[test]
    pub fn test_add() {
        let p1 = Point4D::new(7.0, 21.0, 32.0, 1.0);
        let p2 = Point4D::new(43.0, 5.0, 16.0, 2.0);

        let result = p1 + p2;

        assert_eq!(result, Point4D::new(50.0, 26.0, 48.0, 3.0));
    }

    #[test]
    pub fn test_sub() {
        let p1 = Point4D::new(7.0, 21.0, 32.0, 1.0);
        let p2 = Point4D::new(43.0, 5.0, 16.0, 2.0);

        let result = p1 - p2;

        assert_eq!(result, Point4D::new(-36.0, 16.0, 16.0, -1.0));
    }

    #[test]
    pub fn test_min() {
        let p1 = Point4D::new(1.0, 3.0, 5.0, 7.0);
        let p2 = Point4D::new(2.0, 2.0, -1.0, 10.0);

        let result = p1.min(p2);

        assert_eq!(result, Point4D::new(1.0, 2.0, -1.0, 7.0));
    }

    #[test]
    pub fn test_max() {
        let p1 = Point4D::new(1.0, 3.0, 5.0, 7.0);
        let p2 = Point4D::new(2.0, 2.0, -1.0, 10.0);

        let result = p1.max(p2);

        assert_eq!(result, Point4D::new(2.0, 3.0, 5.0, 10.0));
    }
}
