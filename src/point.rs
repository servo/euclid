// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::{Length, UnknownUnit};
use scale_factor::ScaleFactor;
use size::TypedSize2D;
use num::Zero;

use num_traits::{Float, NumCast};
use std::fmt;
use std::ops::{Add, Neg, Mul, Sub, Div};
use std::marker::PhantomData;
use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};

define_vector! {
    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TypedPoint2D<T, U> {
        pub x: T,
        pub y: T,
    }
}

pub type Point2D<T> = TypedPoint2D<T, UnknownUnit>;

impl<T: Copy, U> Copy for TypedPoint2D<T, U> {}

impl<T: Clone, U> Clone for TypedPoint2D<T, U> {
    fn clone(&self) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x.clone(), self.y.clone())
    }
}

impl<T: PartialEq, U> PartialEq<TypedPoint2D<T, U>> for TypedPoint2D<T, U> {
    fn eq(&self, other: &TypedPoint2D<T, U>) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl<T: Eq, U> Eq for TypedPoint2D<T, U> {}

impl<T: Hash, U> Hash for TypedPoint2D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

impl<T: Zero, U> TypedPoint2D<T, U> {
    pub fn zero() -> TypedPoint2D<T, U> {
        TypedPoint2D::new(Zero::zero(), Zero::zero())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for TypedPoint2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedPoint2D<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.x, self.y)
    }
}

impl<T, U> TypedPoint2D<T, U> {
    pub fn new(x: T, y: T) -> TypedPoint2D<T, U> {
        TypedPoint2D { x: x, y: y, _unit: PhantomData }
    }
}

impl<T: Clone, U> TypedPoint2D<T, U> {
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(x.get(), y.get())
    }
}

impl<T: Clone, U> TypedPoint2D<T, U> {
    pub fn x_typed(&self) -> Length<T, U> { Length::new(self.x.clone()) }
    pub fn y_typed(&self) -> Length<T, U> { Length::new(self.y.clone()) }
}

impl<T, U> TypedPoint2D<T, U>
where T: Copy + Mul<T, Output=T> + Add<T, Output=T> + Sub<T, Output=T> {
    #[inline]
    pub fn dot(self, other: TypedPoint2D<T, U>) -> T {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(self, other: TypedPoint2D<T, U>) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl<T: Clone + Add<T, Output=T>, U> Add for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    fn add(self, other: TypedPoint2D<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Clone + Add<T, Output=T>, U> Add<TypedSize2D<T, U>> for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    fn add(self, other: TypedSize2D<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x + other.width, self.y + other.height)
    }
}

impl<T: Copy + Add<T, Output=T>, U> TypedPoint2D<T, U> {
    pub fn add_size(&self, other: &TypedSize2D<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x + other.width, self.y + other.height)
    }
}

impl<T: Clone + Sub<T, Output=T>, U> Sub for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    fn sub(self, other: TypedPoint2D<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x - other.x, self.y - other.y)
    }
}

impl <T: Clone + Neg<Output=T>, U> Neg for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    #[inline]
    fn neg(self) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(-self.x, -self.y)
    }
}

impl<T: Float, U> TypedPoint2D<T, U> {
    pub fn min(self, other: TypedPoint2D<T, U>) -> TypedPoint2D<T, U> {
         TypedPoint2D::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(self, other: TypedPoint2D<T, U>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl<T: Copy + Mul<T, Output=T>, U> Mul<T> for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    #[inline]
    fn mul(self, scale: T) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x * scale, self.y * scale)
    }
}

impl<T: Copy + Div<T, Output=T>, U> Div<T> for TypedPoint2D<T, U> {
    type Output = TypedPoint2D<T, U>;
    #[inline]
    fn div(self, scale: T) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(self.x / scale, self.y / scale)
    }
}

impl<T: Copy + Mul<T, Output=T>, U1, U2> Mul<ScaleFactor<T, U1, U2>> for TypedPoint2D<T, U1> {
    type Output = TypedPoint2D<T, U2>;
    #[inline]
    fn mul(self, scale: ScaleFactor<T, U1, U2>) -> TypedPoint2D<T, U2> {
        TypedPoint2D::new(self.x * scale.get(), self.y * scale.get())
    }
}

impl<T: Copy + Div<T, Output=T>, U1, U2> Div<ScaleFactor<T, U1, U2>> for TypedPoint2D<T, U2> {
    type Output = TypedPoint2D<T, U1>;
    #[inline]
    fn div(self, scale: ScaleFactor<T, U1, U2>) -> TypedPoint2D<T, U1> {
        TypedPoint2D::new(self.x / scale.get(), self.y / scale.get())
    }
}

// Convenient aliases for TypedPoint2D with typed units

impl<T: Clone, U> TypedPoint2D<T, U> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Point2D<T> {
        TypedPoint2D::new(self.x.clone(), self.y.clone())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Point2D<T>) -> TypedPoint2D<T, U> {
        TypedPoint2D::new(p.x.clone(), p.y.clone())
    }
}

impl<T0: NumCast + Clone, U> TypedPoint2D<T0, U> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast<T1: NumCast + Clone>(&self) -> Option<TypedPoint2D<T1, U>> {
        match (NumCast::from(self.x.clone()), NumCast::from(self.y.clone())) {
            (Some(x), Some(y)) => Some(TypedPoint2D::new(x, y)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<T: NumCast + Clone, U> TypedPoint2D<T, U> {
    pub fn as_f32(&self) -> TypedPoint2D<f32, U> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> TypedPoint2D<usize, U> {
        self.cast().unwrap()
    }
}

define_vector! {
    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TypedPoint3D<T, U> {
        pub x: T,
        pub y: T,
        pub z: T,
    }
}

pub type Point3D<T> = TypedPoint3D<T, UnknownUnit>;

impl<T: Hash, U> Hash for TypedPoint3D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
    }
}

impl<T: Zero, U> TypedPoint3D<T, U> {
    #[inline]
    pub fn zero() -> TypedPoint3D<T, U> {
        TypedPoint3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    }
}

impl<T: Copy, U> Copy for TypedPoint3D<T, U> {}

impl<T: Clone, U> Clone for TypedPoint3D<T, U> {
    fn clone(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: PartialEq, U> PartialEq<TypedPoint3D<T, U>> for TypedPoint3D<T, U> {
    fn eq(&self, other: &TypedPoint3D<T, U>) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
    }
}

impl<T: Eq, U> Eq for TypedPoint3D<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for TypedPoint3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedPoint3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T, U> TypedPoint3D<T, U> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> TypedPoint3D<T, U> {
        TypedPoint3D { x: x, y: y, z: z, _unit: PhantomData }
    }
}

impl<T: Clone, U> TypedPoint3D<T, U> {
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>, z: Length<T, U>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(x.get(), y.get(), z.get())
    }
}

impl<T: Clone, U> TypedPoint3D<T, U> {
    pub fn x_typed(&self) -> Length<T, U> { Length::new(self.x.clone()) }
    pub fn y_typed(&self) -> Length<T, U> { Length::new(self.y.clone()) }
    pub fn z_typed(&self) -> Length<T, U> { Length::new(self.z.clone()) }
}

impl<T: Mul<T, Output=T> +
        Add<T, Output=T> +
        Sub<T, Output=T> +
        Copy, U> TypedPoint3D<T, U> {
    #[inline]
    pub fn dot(self, other: TypedPoint3D<T, U>) -> T {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: TypedPoint3D<T, U>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.y * other.z - self.z * other.y,
                          self.z * other.x - self.x * other.z,
                          self.x * other.y - self.y * other.x)
    }
}

impl<T: Clone + Add<T, Output=T>, U> Add for TypedPoint3D<T, U> {
    type Output = TypedPoint3D<T, U>;
    fn add(self, other: TypedPoint3D<T, U>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.x + other.x,
                          self.y + other.y,
                          self.z + other.z)
    }
}

impl<T: Clone + Sub<T, Output=T>, U> Sub for TypedPoint3D<T, U> {
    type Output = TypedPoint3D<T, U>;
    fn sub(self, other: TypedPoint3D<T, U>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.x - other.x,
                          self.y - other.y,
                          self.z - other.z)
    }
}

impl <T: Clone + Neg<Output=T>, U> Neg for TypedPoint3D<T, U> {
    type Output = TypedPoint3D<T, U>;
    #[inline]
    fn neg(self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Float, U> TypedPoint3D<T, U> {
    pub fn min(self, other: TypedPoint3D<T, U>) -> TypedPoint3D<T, U> {
         TypedPoint3D::new(self.x.min(other.x),
                           self.y.min(other.y),
                           self.z.min(other.z))
    }

    pub fn max(self, other: TypedPoint3D<T, U>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.x.max(other.x), self.y.max(other.y),
                     self.z.max(other.z))
    }
}

impl<T: Clone, U> TypedPoint3D<T, U> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Point3D<T> {
        TypedPoint3D::new(self.x.clone(), self.y.clone(), self.z.clone())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Point3D<T>) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(p.x.clone(), p.y.clone(), p.z.clone())
    }
}

define_vector! {
    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TypedPoint4D<T, U> {
        pub x: T,
        pub y: T,
        pub z: T,
        pub w: T,
    }
}

pub type Point4D<T> = TypedPoint4D<T, UnknownUnit>;

impl<T: Copy, U> Copy for TypedPoint4D<T, U> {}

impl<T: Clone, U> Clone for TypedPoint4D<T, U> {
    fn clone(&self) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(self.x.clone(),
                          self.y.clone(),
                          self.z.clone(),
                          self.w.clone())
    }
}

impl<T: PartialEq, U> PartialEq<TypedPoint4D<T, U>> for TypedPoint4D<T, U> {
    fn eq(&self, other: &TypedPoint4D<T, U>) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z) && self.w.eq(&other.w)
    }
}

impl<T: Eq, U> Eq for TypedPoint4D<T, U> {}

impl<T: Hash, U> Hash for TypedPoint4D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
        self.w.hash(h);
    }
}

impl<T: Zero, U> TypedPoint4D<T, U> {
    #[inline]
    pub fn zero() -> TypedPoint4D<T, U> {
        TypedPoint4D::new(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for TypedPoint4D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?},{:?})", self.x, self.y, self.z, self.w)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedPoint4D<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}

impl<T, U> TypedPoint4D<T, U> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> TypedPoint4D<T, U> {
        TypedPoint4D { x: x, y: y, z: z, w: w, _unit: PhantomData }
    }
}

impl<T: Clone, U> TypedPoint4D<T, U> {
    pub fn from_lengths(x: Length<T, U>,
                        y: Length<T, U>,
                        z: Length<T, U>,
                        w: Length<T, U>) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(x.get(), y.get(), z.get(), w.get())
    }
}

impl<T: Clone, U> TypedPoint4D<T, U> {
    pub fn x_typed(&self) -> Length<T, U> { Length::new(self.x.clone()) }
    pub fn y_typed(&self) -> Length<T, U> { Length::new(self.y.clone()) }
    pub fn z_typed(&self) -> Length<T, U> { Length::new(self.z.clone()) }
    pub fn w_typed(&self) -> Length<T, U> { Length::new(self.w.clone()) }
}

impl<T: Clone + Add<T, Output=T>, U> Add for TypedPoint4D<T, U> {
    type Output = TypedPoint4D<T, U>;
    fn add(self, other: TypedPoint4D<T, U>) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(self.x + other.x,
                          self.y + other.y,
                          self.z + other.z,
                          self.w + other.w)
    }
}

impl<T: Clone + Sub<T, Output=T>, U> Sub for TypedPoint4D<T, U> {
    type Output = TypedPoint4D<T, U>;
    fn sub(self, other: TypedPoint4D<T, U>) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(self.x - other.x,
                          self.y - other.y,
                          self.z - other.z,
                          self.w - other.w)
    }
}

impl <T: Clone + Neg<Output=T>, U> Neg for TypedPoint4D<T, U> {
    type Output = TypedPoint4D<T, U>;
    #[inline]
    fn neg(self) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T: Float, U> TypedPoint4D<T, U> {
    pub fn min(self, other: TypedPoint4D<T, U>) -> TypedPoint4D<T, U> {
         TypedPoint4D::new(self.x.min(other.x), self.y.min(other.y),
                           self.z.min(other.z), self.w.min(other.w))
    }

    pub fn max(self, other: TypedPoint4D<T, U>) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(self.x.max(other.x), self.y.max(other.y),
                          self.z.max(other.z), self.w.max(other.w))
    }
}

impl<T: Clone, U> TypedPoint4D<T, U> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Point4D<T> {
        TypedPoint4D::new(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Point4D<T>) -> TypedPoint4D<T, U> {
        TypedPoint4D::new(p.x.clone(), p.y.clone(), p.z.clone(), p.w.clone())
    }
}

#[cfg(test)]
mod point2d {
    use super::Point2D;

    #[test]
    pub fn test_scalar_mul() {
        let p1: Point2D<f32> = Point2D::new(3.0, 5.0);

        let result = p1 * 5.0;

        assert_eq!(result, Point2D::new(15.0, 25.0));
    }

    #[test]
    pub fn test_dot() {
        let p1: Point2D<f32> = Point2D::new(2.0, 7.0);
        let p2: Point2D<f32> = Point2D::new(13.0, 11.0);
        assert_eq!(p1.dot(p2), 103.0);
    }

    #[test]
    pub fn test_cross() {
        let p1: Point2D<f32> = Point2D::new(4.0, 7.0);
        let p2: Point2D<f32> = Point2D::new(13.0, 8.0);
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

    pub type Point2DMm<T> = TypedPoint2D<T, Mm>;
    pub type Point2DCm<T> = TypedPoint2D<T, Cm>;

    #[test]
    pub fn test_add() {
        let p1 = Point2DMm::new(1.0, 2.0);
        let p2 = Point2DMm::new(3.0, 4.0);

        let result = p1 + p2;

        assert_eq!(result, Point2DMm::new(4.0, 6.0));
    }

    #[test]
    pub fn test_scalar_mul() {
        let p1 = Point2DMm::new(1.0, 2.0);
        let cm_per_mm: ScaleFactor<f32, Mm, Cm> = ScaleFactor::new(0.1);

        let result = p1 * cm_per_mm;

        assert_eq!(result, Point2DCm::new(0.1, 0.2));
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
