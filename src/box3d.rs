// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::UnknownUnit;
use length::Length;
use scale::TypedScale;
use num::*;
use point::TypedPoint3D;
use vector::TypedVector3D;
use side_offsets::TypedSideOffsets3D;
use size::TypedSize3D;
use approxord::{min, max};

use num_traits::NumCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use core::borrow::Borrow;
use core::cmp::PartialOrd;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Add, Div, Mul, Sub, Neg};


/// A 3d box optionally tagged with a unit. 
/// 
/// a is the upper left front corner position.
/// b is the lower right back corner position.
/// 
/// Axis directions: x axis positive going left to right.
///                  y axis positive going bottom to top.
///                  z axis positive going from back to front (out of the page).
#[repr(C)]
pub struct TypedBox3D<T, U = UnknownUnit> {
    pub a: TypedPoint3D<T, U>, 
    pub b: TypedPoint3D<T, U>,
}

/// The default box 3d type with no unit.
pub type Box3D<T> = TypedBox3D<T, UnknownUnit>;

#[cfg(feature = "serde")]
impl<'de, T: Copy + Deserialize<'de>, U> Deserialize<'de> for TypedBox3D<T, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (a, b) = try!(Deserialize::deserialize(deserializer));
        Ok(TypedBox3D::new(a, b))
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize, U> Serialize for TypedBox3D<T, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.a, &self.b).serialize(serializer)
    }
}

impl<T: Hash, U> Hash for TypedBox3D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.a.hash(h);
        self.b.hash(h);
    }
}

impl<T: Copy, U> Copy for TypedBox3D<T, U> {}

impl<T: Copy, U> Clone for TypedBox3D<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: PartialEq, U> PartialEq<TypedBox3D<T, U>> for TypedBox3D<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.b.eq(&other.b)
    }
}

impl<T: Eq, U> Eq for TypedBox3D<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for TypedBox3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TypedBox3D({:?}, {:?})", self.a, self.b)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedBox3D<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Box3D({}, {})", self.a, self.b)
    }
}

impl<T, U> TypedBox3D<T, U> {
    /// Constructor.
    pub fn new(a: TypedPoint3D<T, U>, b: TypedPoint3D<T, U>) -> Self {
        TypedBox3D {
            a,
            b,
        }
    }
}

impl<T, U> TypedBox3D<T, U> 
where
    T: Copy + Clone + PartialOrd 
{
    /// Fix coordinates so that a and b represent the corners described.
    pub fn fix(&self) -> Self {
        let true_a = TypedPoint3D::new(
            min(self.a.x, self.b.x),
            max(self.a.y, self.b.y),
            max(self.a.z, self.b.z),
        );

        let true_b = TypedPoint3D::new(
            min(self.a.x, self.b.x),
            max(self.a.y, self.b.y),
            max(self.a.z, self.b.z),
        );

        TypedBox3D {
            a: true_a,
            b: true_b,
        }
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Div<T, Output = T> + Neg<Output = T> + Add<T, Output = T> + One
{
    /// Creates a box3d of the given size, so that the centroid is at zero origin.
    pub fn from_size(size: TypedSize3D<T, U>) -> Self {
        let two = T::one() + T::one();
        TypedBox3D {
            a: TypedPoint3D::new(-size.width / two, size.height / two, size.depth / two),
            b: TypedPoint3D::new(size.width / two, -size.height / two, -size.depth / two)
        }
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Clone + Zero + PartialOrd + PartialEq + Add<T, Output = T> + Sub<T, Output = T>,
{
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.a.x < other.b.x
            && self.b.x > other.a.x
            && self.a.y < other.b.y
            && self.b.y > other.a.y
            && self.a.z < other.b.z
            && self.b.z > other.a.z
    }

    #[inline]
    pub fn max_x(&self) -> T {
        self.b.x
    }

    #[inline]
    pub fn min_x(&self) -> T {
        self.a.x
    }

    #[inline]
    pub fn max_y(&self) -> T {
        self.a.y
    }

    #[inline]
    pub fn min_y(&self) -> T {
        self.b.y
    }

    #[inline]
    pub fn max_z(&self) -> T {
        self.a.z
    }

    #[inline]
    pub fn min_z(&self) -> T {
        self.b.z
    }

    #[inline]
    pub fn max_x_typed(&self) -> Length<T, U> {
        Length::new(self.max_x())
    }

    #[inline]
    pub fn min_x_typed(&self) -> Length<T, U> {
        Length::new(self.min_x())
    }

    #[inline]
    pub fn max_y_typed(&self) -> Length<T, U> {
        Length::new(self.max_y())
    }

    #[inline]
    pub fn min_y_typed(&self) -> Length<T, U> {
        Length::new(self.min_y())
    }

    #[inline]
    pub fn max_z_typed(&self) -> Length<T, U> {
        Length::new(self.max_z())
    }

    #[inline]
    pub fn min_z_typed(&self) -> Length<T, U> {
        Length::new(self.min_z())
    }

    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        let intersection_a = TypedPoint3D::new(
            max(self.min_x(), other.min_x()),
            min(self.max_y(), other.max_y()),
            min(self.max_z(), other.max_z()),
        );

        let intersection_b = TypedPoint3D::new(
            min(self.max_x(), other.max_x()),
            max(self.min_y(), other.min_y()),
            max(self.min_z(), other.min_z()),
        );

        Some(TypedBox3D::new(
            intersection_a, 
            intersection_b,
        ))
    }

    /// Returns the same box3d, translated by a vector.
    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate(&self, by: &TypedVector3D<T, U>) -> Self {
        Self::new(self.a + *by, self.b + *by)
    }

    /// Returns true if this box3d contains the point. Points are considered
    /// in the box3d if they are on the front, left or top faces, but outside if they
    /// are on the back, right or bottom faces.
    #[inline]
    pub fn contains(&self, other: &TypedPoint3D<T, U>) -> bool {
        self.a.x <= other.x && other.x < self.b.x
            && self.a.y >= other.y && other.y > self.b.y
            && self.a.z >= other.z && other.z > self.b.z
    }

    /// Returns true if this box3d contains the interior of the other box3d. Always
    /// returns true if other is empty, and always returns false if other is
    /// nonempty but this box3d is empty.
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        other.is_empty()
            || (self.min_x() <= other.min_x() && other.max_x() <= self.max_x()
                && self.min_y() <= other.min_y() && other.max_y() <= self.max_y()
                && self.min_z() <= other.min_z() && other.max_z() <= self.max_z())
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn inflate(&self, width: T, height: T, depth: T) -> Self {
        TypedBox3D::new(
            TypedPoint3D::new(self.a.x - width, self.a.y - height, self.a.z - depth),
            TypedSize3D::new(
                self.b.width + width + width,
                self.b.height + height + height,
                self.b.depth + depth + depth,
            ),
        )
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn inflate_typed(&self, width: Length<T, U>, height: Length<T, U>, depth: Length<T, U>) -> Self {
        self.inflate(width.get(), height.get(), depth.get())
    }

    #[inline]
    pub fn top_right_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.a.y, self.a.z)
    }

    #[inline]
    pub fn bottom_left_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.a.x, self.max_y(), self.a.z)
    }

    #[inline]
    pub fn bottom_right_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.a.z)
    }

    #[inline]
    pub fn top_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.a.x, self.a.y, self.max_z())
    }

    #[inline]
    pub fn top_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.a.y, self.max_z())
    }

    #[inline]
    pub fn bottom_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.a.x, self.max_y(), self.max_z())
    }

    #[inline]
    pub fn bottom_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.max_z())
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate_by_size(&self, b: &TypedPoint3D<T, U>) -> Self {
        self.translate(&b.to_vector())
    }

    /// Calculate the b and position of an inner box3d.
    ///
    /// Subtracts the side offsets from all sides. The horizontal and vertical
    /// offsets must not be larger than the original side length.
    pub fn inner_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        let box3d = TypedBox3D::new(
            TypedPoint3D::new(
                self.a.x + offsets.left,
                self.a.y + offsets.top,
                self.a.z + offsets.front,
            ),
            TypedSize3D::new(
                self.b.width - offsets.horizontal(),
                self.b.height - offsets.vertical(),
                self.b.depth - offsets.applicate(),
            )
        );
        debug_assert!(box3d.b.width >= Zero::zero());
        debug_assert!(box3d.b.height >= Zero::zero());
        debug_assert!(box3d.b.depth >= Zero::zero());
        box3d
    }

    /// Calculate the b and position of an outer box3d.
    ///
    /// Add the offsets to all sides. The expanded box3d is returned.
    pub fn outer_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        TypedBox3D::new(
            TypedPoint3D::new(
                self.a.x - offsets.left,
                self.a.y - offsets.top,
                self.a.z - offsets.front,
            ),
            TypedSize3D::new(
                self.b.width + offsets.horizontal(),
                self.b.height + offsets.vertical(),
                self.b.depth + offsets.applicate(),
            )
        )
    }

    /// Returns the largest box3d defined by the outer-most
    /// points provided.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<TypedPoint3D<T, U>>,
    {
        let mut points = points.into_iter();

        let (mut min_x, mut min_y, mut min_z) = match points.next() {
            Some(first) => (first.borrow().x, first.borrow().y, first.borrow().z),
            None => return TypedBox3D::zero(),
        };

        let (mut max_x, mut max_y, mut max_z) = (min_x, min_y, min_z);
        for point in points {
            let p = point.borrow();
            if p.x < min_x {
                min_x = p.x
            }
            if p.x > max_x {
                max_x = p.x
            }
            if p.y < min_y {
                min_y = p.y
            }
            if p.y > max_y {
                max_y = p.y
            }
            if p.z < min_z {
                min_z = p.z
            }
            if p.y > max_y {
                max_z = p.z
            }
        }
        TypedBox3D::new(
            TypedPoint3D::new(min_x, min_y, min_z),
            TypedSize3D::new(max_x - min_x, max_y - min_y, max_z - min_z),
        )
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this box3d and another box3d.
    ///
    /// `t` is expected to be between zero and one.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        Self::new(
            self.a.lerp(other.a, t),
            self.b.lerp(other.b, t),
        )
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + One + Add<Output = T> + Div<Output = T>,
{
    pub fn center(&self) -> TypedPoint3D<T, U> {
        let two = T::one() + T::one();
        self.a + self.b.to_vector() / two
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Clone + PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Zero,
{
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        if self.b == Zero::zero() {
            return *other;
        }
        if other.b == Zero::zero() {
            return *self;
        }

        let upper_left_front = TypedPoint3D::new(
            min(self.min_x(), other.min_x()),
            min(self.min_y(), other.min_y()),
            min(self.min_z(), other.min_z()),
        );

        let lower_right_back_x = max(self.max_x(), other.max_x());
        let lower_right_back_y = max(self.max_y(), other.max_y());
        let lower_right_back_z = max(self.max_z(), other.max_z());

        TypedBox3D::new(
            upper_left_front,
            TypedSize3D::new(lower_right_back_x - upper_left_front.x, lower_right_back_y - upper_left_front.y, lower_right_back_z - upper_left_front.z),
        )
    }
}

impl<T, U> TypedBox3D<T, U> {
    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S, z: S) -> Self
    where
        T: Copy + Clone + Mul<S, Output = T>,
    {
        TypedBox3D::new(
            TypedPoint3D::new(self.a.x * x, self.a.y * y, self.a.z * z),
            TypedSize3D::new(self.b.width * x, self.b.height * y, self.b.depth * z),
        )
    }
}

impl<T: Copy + Clone + Mul<T, Output = T>, U> TypedBox3D<T, U> {
    #[inline]
    pub fn volume(&self) -> T {
        self.b.volume()
    }
}

impl<T: Copy + PartialEq + Zero, U> TypedBox3D<T, U> {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedBox3D::new(TypedPoint3D::a(), TypedSize3D::zero())
    }

    /// Returns true if the b is zero, regardless of the a's value.
    pub fn is_empty(&self) -> bool {
        self.b.width == Zero::zero() || self.b.height == Zero::zero() || self.b.depth == Zero::zero()
    }
}

impl<T: Copy + Mul<T, Output = T>, U> Mul<T> for TypedBox3D<T, U> {
    type Output = Self;
    #[inline]
    fn mul(self, scale: T) -> Self {
        TypedBox3D::new(self.a * scale, self.b * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U> Div<T> for TypedBox3D<T, U> {
    type Output = Self;
    #[inline]
    fn div(self, scale: T) -> Self {
        TypedBox3D::new(self.a / scale, self.b / scale)
    }
}

impl<T: Copy + Mul<T, Output = T>, U1, U2> Mul<TypedScale<T, U1, U2>> for TypedBox3D<T, U1> {
    type Output = TypedBox3D<T, U2>;
    #[inline]
    fn mul(self, scale: TypedScale<T, U1, U2>) -> TypedBox3D<T, U2> {
        TypedBox3D::new(self.a * scale, self.b * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U1, U2> Div<TypedScale<T, U1, U2>> for TypedBox3D<T, U2> {
    type Output = TypedBox3D<T, U1>;
    #[inline]
    fn div(self, scale: TypedScale<T, U1, U2>) -> TypedBox3D<T, U1> {
        TypedBox3D::new(self.a / scale, self.b / scale)
    }
}

impl<T: Copy, Unit> TypedBox3D<T, Unit> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Box3D<T> {
        TypedBox3D::new(self.a.to_untyped(), self.b.to_untyped())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(c: &Box3D<T>) -> TypedBox3D<T, Unit> {
        TypedBox3D::new(
            TypedPoint3D::from_untyped(&c.a),
            TypedSize3D::from_untyped(&c.b),
        )
    }
}

impl<T0: NumCast + Copy, Unit> TypedBox3D<T0, Unit> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using round(), round_in or round_out() before casting.
    pub fn cast<T1: NumCast + Copy>(&self) -> TypedBox3D<T1, Unit> {
        TypedBox3D::new(
            self.a.cast(),
            self.b.cast(),
        )
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using round(), round_in or round_out() before casting.
    pub fn try_cast<T1: NumCast + Copy>(&self) -> Option<TypedBox3D<T1, Unit>> {
        match (self.a.try_cast(), self.b.try_cast()) {
            (Some(a), Some(b)) => Some(TypedBox3D::new(a, b)),
            _ => None,
        }
    }
}

impl<T: Floor + Ceil + Round + Add<T, Output = T> + Sub<T, Output = T>, U> TypedBox3D<T, U> {
    /// Return a box3d with edges rounded to integer coordinates, such that
    /// the returned box3d has the same set of pixel centers as the original
    /// one.
    /// Values equal to 0.5 round up.
    /// Suitable for most places where integral device coordinates
    /// are needed, but note that any translation should be applied first to
    /// avoid pixel rounding errors.
    /// Note that this is *not* rounding to nearest integer if the values are negative.
    /// They are always rounding as floor(n + 0.5).
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round(&self) -> Self {
        let a = self.a.round();
        let b = self.a.add_size(&self.b).round() - a;
        TypedBox3D::new(a, TypedSize3D::new(b.x, b.y, b.z))
    }

    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d contains the resulting box3d.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_in(&self) -> Self {
        let a = self.a.ceil();
        let b = self.a.add_size(&self.b).floor() - a;
        TypedBox3D::new(a, TypedSize3D::new(b.x, b.y, b.z))
    }

    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d is contained in the resulting box3d.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_out(&self) -> Self {
        let a = self.a.floor();
        let b = self.a.add_size(&self.b).ceil() - a;
        TypedBox3D::new(a, TypedSize3D::new(b.x, b.y, b.z))
    }
}

// Convenience functions for common casts
impl<T: NumCast + Copy, Unit> TypedBox3D<T, Unit> {
    /// Cast into an `f32` box3d.
    pub fn to_f32(&self) -> TypedBox3D<f32, Unit> {
        self.cast()
    }

    /// Cast into an `f64` box3d.
    pub fn to_f64(&self) -> TypedBox3D<f64, Unit> {
        self.cast()
    }

    /// Cast into an `usize` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_usize(&self) -> TypedBox3D<usize, Unit> {
        self.cast()
    }

    /// Cast into an `u32` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_u32(&self) -> TypedBox3D<u32, Unit> {
        self.cast()
    }

    /// Cast into an `i32` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_i32(&self) -> TypedBox3D<i32, Unit> {
        self.cast()
    }

    /// Cast into an `i64` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_i64(&self) -> TypedBox3D<i64, Unit> {
        self.cast()
    }
}

impl<T, U> From<TypedSize3D<T, U>> for TypedBox3D<T, U>
where T: Copy + Zero
{
    fn from(b: TypedSize3D<T, U>) -> Self {
        Self::from_size(b)
    }
}

/// Shorthand for `TypedBox3D::new(TypedPoint3D::new(x, y, z), TypedSize3D::new(w, h, d))`.
pub fn box3d<T: Copy, U>(x: T, y: T, z: T, w: T, h: T, d: T) -> TypedBox3D<T, U> {
    TypedBox3D::new(TypedPoint3D::new(x, y, z), TypedSize3D::new(w, h, d))
}
