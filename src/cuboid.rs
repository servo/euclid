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
use core::ops::{Add, Div, Mul, Sub};


/// A 3d Cuboid optionally tagged with a unit.
#[repr(C)]
pub struct TypedCuboid<T, U = UnknownUnit> {
    pub origin: TypedPoint3D<T, U>,
    pub size: TypedSize3D<T, U>,
}

/// The default cuboid type with no unit.
pub type Cuboid<T> = TypedCuboid<T, UnknownUnit>;

#[cfg(feature = "serde")]
impl<'de, T: Copy + Deserialize<'de>, U> Deserialize<'de> for TypedCuboid<T, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (origin, size) = try!(Deserialize::deserialize(deserializer));
        Ok(TypedCuboid::new(origin, size))
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize, U> Serialize for TypedCuboid<T, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.origin, &self.size).serialize(serializer)
    }
}

impl<T: Hash, U> Hash for TypedCuboid<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.origin.hash(h);
        self.size.hash(h);
    }
}

impl<T: Copy, U> Copy for TypedCuboid<T, U> {}

impl<T: Copy, U> Clone for TypedCuboid<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: PartialEq, U> PartialEq<TypedCuboid<T, U>> for TypedCuboid<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin) && self.size.eq(&other.size)
    }
}

impl<T: Eq, U> Eq for TypedCuboid<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for TypedCuboid<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TypedCuboid({:?} at {:?})", self.size, self.origin)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedCuboid<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Cuboid({} at {})", self.size, self.origin)
    }
}

impl<T, U> TypedCuboid<T, U> {
    /// Constructor.
    pub fn new(origin: TypedPoint3D<T, U>, size: TypedSize3D<T, U>) -> Self {
        TypedCuboid {
            origin,
            size,
        }
    }
}

impl<T, U> TypedCuboid<T, U>
where
    T: Copy + Zero
{
    /// Creates a cuboid of the given size, at offset zero.
    pub fn from_size(size: TypedSize3D<T, U>) -> Self {
        TypedCuboid {
            origin: TypedPoint3D::zero(),
            size,
        }
    }
}

impl<T, U> TypedCuboid<T, U>
where
    T: Copy + Clone + Zero + PartialOrd + PartialEq + Add<T, Output = T> + Sub<T, Output = T>,
{
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.origin.x < other.origin.x + other.size.width
            && other.origin.x < self.origin.x + self.size.width
            && self.origin.y < other.origin.y + other.size.height
            && other.origin.y < self.origin.y + self.size.height
            && self.origin.z < other.origin.z + other.size.depth
            && other.origin.z < self.origin.z + self.size.depth
    }

    #[inline]
    pub fn max_x(&self) -> T {
        self.origin.x + self.size.width
    }

    #[inline]
    pub fn min_x(&self) -> T {
        self.origin.x
    }

    #[inline]
    pub fn max_y(&self) -> T {
        self.origin.y + self.size.height
    }

    #[inline]
    pub fn min_y(&self) -> T {
        self.origin.y
    }

    #[inline]
    pub fn max_z(&self) -> T {
        self.origin.z + self.size.depth
    }

    #[inline]
    pub fn min_z(&self) -> T {
        self.origin.z
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

        let upper_left_front = TypedPoint3D::new(
            max(self.min_x(), other.min_x()),
            max(self.min_y(), other.min_y()),
            max(self.min_z(), other.min_z()),
        );
        let lower_right_back_x = min(self.max_x(), other.max_x());
        let lower_right_back_y = min(self.max_y(), other.max_y());
        let lower_right_back_z = min(self.max_z(), other.max_z());

        Some(TypedCuboid::new(
            upper_left_front,
            TypedSize3D::new(lower_right_back_x - upper_left_front.x, lower_right_back_y - upper_left_front.y, lower_right_back_z - upper_left_front.z),
        ))
    }

    /// Returns the same cuboid, translated by a vector.
    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate(&self, by: &TypedVector3D<T, U>) -> Self {
        Self::new(self.origin + *by, self.size)
    }

    /// Returns true if this cuboid contains the point. Points are considered
    /// in the cuboid if they are on the front, left or top faces, but outside if they
    /// are on the back, right or bottom faces.
    #[inline]
    pub fn contains(&self, other: &TypedPoint3D<T, U>) -> bool {
        self.origin.x <= other.x && other.x < self.origin.x + self.size.width
            && self.origin.y <= other.y && other.y < self.origin.y + self.size.height
            && self.origin.z <= other.z && other.z < self.origin.z + self.size.depth
    }

    /// Returns true if this cuboid contains the interior of the other cuboid. Always
    /// returns true if other is empty, and always returns false if other is
    /// nonempty but this cuboid is empty.
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
        TypedCuboid::new(
            TypedPoint3D::new(self.origin.x - width, self.origin.y - height, self.origin.z - depth),
            TypedSize3D::new(
                self.size.width + width + width,
                self.size.height + height + height,
                self.size.depth + depth + depth,
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
        TypedPoint3D::new(self.max_x(), self.origin.y, self.origin.z)
    }

    #[inline]
    pub fn bottom_left_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.origin.x, self.max_y(), self.origin.z)
    }

    #[inline]
    pub fn bottom_right_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.origin.z)
    }

    #[inline]
    pub fn top_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.origin.x, self.origin.y, self.max_z())
    }

    #[inline]
    pub fn top_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.origin.y, self.max_z())
    }

    #[inline]
    pub fn bottom_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.origin.x, self.max_y(), self.max_z())
    }

    #[inline]
    pub fn bottom_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.max_z())
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate_by_size(&self, size: &TypedPoint3D<T, U>) -> Self {
        self.translate(&size.to_vector())
    }

    /// Calculate the size and position of an inner cuboid.
    ///
    /// Subtracts the side offsets from all sides. The horizontal and vertical
    /// offsets must not be larger than the original side length.
    pub fn inner_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        let cuboid = TypedCuboid::new(
            TypedPoint3D::new(
                self.origin.x + offsets.left,
                self.origin.y + offsets.top,
                self.origin.z + offsets.front,
            ),
            TypedSize3D::new(
                self.size.width - offsets.horizontal(),
                self.size.height - offsets.vertical(),
                self.size.depth - offsets.applicate(),
            )
        );
        debug_assert!(cuboid.size.width >= Zero::zero());
        debug_assert!(cuboid.size.height >= Zero::zero());
        debug_assert!(cuboid.size.depth >= Zero::zero());
        cuboid
    }

    /// Calculate the size and position of an outer cuboid.
    ///
    /// Add the offsets to all sides. The expanded cuboid is returned.
    pub fn outer_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        TypedCuboid::new(
            TypedPoint3D::new(
                self.origin.x - offsets.left,
                self.origin.y - offsets.top,
                self.origin.z - offsets.front,
            ),
            TypedSize3D::new(
                self.size.width + offsets.horizontal(),
                self.size.height + offsets.vertical(),
                self.size.depth + offsets.applicate(),
            )
        )
    }

    /// Returns the largest cuboid defined by the outer-most
    /// points provided.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<TypedPoint3D<T, U>>,
    {
        let mut points = points.into_iter();

        let (mut min_x, mut min_y, mut min_z) = match points.next() {
            Some(first) => (first.borrow().x, first.borrow().y, first.borrow().z),
            None => return TypedCuboid::zero(),
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
        TypedCuboid::new(
            TypedPoint3D::new(min_x, min_y, min_z),
            TypedSize3D::new(max_x - min_x, max_y - min_y, max_z - min_z),
        )
    }
}

impl<T, U> TypedCuboid<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this cuboid and another cuboid.
    ///
    /// `t` is expected to be between zero and one.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        Self::new(
            self.origin.lerp(other.origin, t),
            self.size.lerp(other.size, t),
        )
    }
}

impl<T, U> TypedCuboid<T, U>
where
    T: Copy + One + Add<Output = T> + Div<Output = T>,
{
    pub fn center(&self) -> TypedPoint3D<T, U> {
        let two = T::one() + T::one();
        self.origin + self.size.to_vector() / two
    }
}

impl<T, U> TypedCuboid<T, U>
where
    T: Copy + Clone + PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Zero,
{
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        if self.size == Zero::zero() {
            return *other;
        }
        if other.size == Zero::zero() {
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

        TypedCuboid::new(
            upper_left_front,
            TypedSize3D::new(lower_right_back_x - upper_left_front.x, lower_right_back_y - upper_left_front.y, lower_right_back_z - upper_left_front.z),
        )
    }
}

impl<T, U> TypedCuboid<T, U> {
    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S, z: S) -> Self
    where
        T: Copy + Clone + Mul<S, Output = T>,
    {
        TypedCuboid::new(
            TypedPoint3D::new(self.origin.x * x, self.origin.y * y, self.origin.z * z),
            TypedSize3D::new(self.size.width * x, self.size.height * y, self.size.depth * z),
        )
    }
}

impl<T: Copy + Clone + Mul<T, Output = T>, U> TypedCuboid<T, U> {
    #[inline]
    pub fn volume(&self) -> T {
        self.size.volume()
    }
}

impl<T: Copy + PartialEq + Zero, U> TypedCuboid<T, U> {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedCuboid::new(TypedPoint3D::origin(), TypedSize3D::zero())
    }

    /// Returns true if the size is zero, regardless of the origin's value.
    pub fn is_empty(&self) -> bool {
        self.size.width == Zero::zero() || self.size.height == Zero::zero() || self.size.depth == Zero::zero()
    }
}

impl<T: Copy + Mul<T, Output = T>, U> Mul<T> for TypedCuboid<T, U> {
    type Output = Self;
    #[inline]
    fn mul(self, scale: T) -> Self {
        TypedCuboid::new(self.origin * scale, self.size * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U> Div<T> for TypedCuboid<T, U> {
    type Output = Self;
    #[inline]
    fn div(self, scale: T) -> Self {
        TypedCuboid::new(self.origin / scale, self.size / scale)
    }
}

impl<T: Copy + Mul<T, Output = T>, U1, U2> Mul<TypedScale<T, U1, U2>> for TypedCuboid<T, U1> {
    type Output = TypedCuboid<T, U2>;
    #[inline]
    fn mul(self, scale: TypedScale<T, U1, U2>) -> TypedCuboid<T, U2> {
        TypedCuboid::new(self.origin * scale, self.size * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U1, U2> Div<TypedScale<T, U1, U2>> for TypedCuboid<T, U2> {
    type Output = TypedCuboid<T, U1>;
    #[inline]
    fn div(self, scale: TypedScale<T, U1, U2>) -> TypedCuboid<T, U1> {
        TypedCuboid::new(self.origin / scale, self.size / scale)
    }
}

impl<T: Copy, Unit> TypedCuboid<T, Unit> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Cuboid<T> {
        TypedCuboid::new(self.origin.to_untyped(), self.size.to_untyped())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(c: &Cuboid<T>) -> TypedCuboid<T, Unit> {
        TypedCuboid::new(
            TypedPoint3D::from_untyped(&c.origin),
            TypedSize3D::from_untyped(&c.size),
        )
    }
}

impl<T0: NumCast + Copy, Unit> TypedCuboid<T0, Unit> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using round(), round_in or round_out() before casting.
    pub fn cast<T1: NumCast + Copy>(&self) -> TypedCuboid<T1, Unit> {
        TypedCuboid::new(
            self.origin.cast(),
            self.size.cast(),
        )
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using round(), round_in or round_out() before casting.
    pub fn try_cast<T1: NumCast + Copy>(&self) -> Option<TypedCuboid<T1, Unit>> {
        match (self.origin.try_cast(), self.size.try_cast()) {
            (Some(origin), Some(size)) => Some(TypedCuboid::new(origin, size)),
            _ => None,
        }
    }
}

impl<T: Floor + Ceil + Round + Add<T, Output = T> + Sub<T, Output = T>, U> TypedCuboid<T, U> {
    /// Return a cuboid with edges rounded to integer coordinates, such that
    /// the returned cuboid has the same set of pixel centers as the original
    /// one.
    /// Values equal to 0.5 round up.
    /// Suitable for most places where integral device coordinates
    /// are needed, but note that any translation should be applied first to
    /// avoid pixel rounding errors.
    /// Note that this is *not* rounding to nearest integer if the values are negative.
    /// They are always rounding as floor(n + 0.5).
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round(&self) -> Self {
        let origin = self.origin.round();
        let size = self.origin.add_size(&self.size).round() - origin;
        TypedCuboid::new(origin, TypedSize3D::new(size.x, size.y, size.z))
    }

    /// Return a cuboid with faces/edges rounded to integer coordinates, such that
    /// the original cuboid contains the resulting cuboid.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_in(&self) -> Self {
        let origin = self.origin.ceil();
        let size = self.origin.add_size(&self.size).floor() - origin;
        TypedCuboid::new(origin, TypedSize3D::new(size.x, size.y, size.z))
    }

    /// Return a cuboid with faces/edges rounded to integer coordinates, such that
    /// the original cuboid is contained in the resulting cuboid.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_out(&self) -> Self {
        let origin = self.origin.floor();
        let size = self.origin.add_size(&self.size).ceil() - origin;
        TypedCuboid::new(origin, TypedSize3D::new(size.x, size.y, size.z))
    }
}

// Convenience functions for common casts
impl<T: NumCast + Copy, Unit> TypedCuboid<T, Unit> {
    /// Cast into an `f32` cuboid.
    pub fn to_f32(&self) -> TypedCuboid<f32, Unit> {
        self.cast()
    }

    /// Cast into an `f64` cuboid.
    pub fn to_f64(&self) -> TypedCuboid<f64, Unit> {
        self.cast()
    }

    /// Cast into an `usize` cuboid, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_usize(&self) -> TypedCuboid<usize, Unit> {
        self.cast()
    }

    /// Cast into an `u32` cuboid, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_u32(&self) -> TypedCuboid<u32, Unit> {
        self.cast()
    }

    /// Cast into an `i32` cuboid, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_i32(&self) -> TypedCuboid<i32, Unit> {
        self.cast()
    }

    /// Cast into an `i64` cuboid, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    pub fn to_i64(&self) -> TypedCuboid<i64, Unit> {
        self.cast()
    }
}

impl<T, U> From<TypedSize3D<T, U>> for TypedCuboid<T, U>
where T: Copy + Zero
{
    fn from(size: TypedSize3D<T, U>) -> Self {
        Self::from_size(size)
    }
}

/// Shorthand for `TypedCuboid::new(TypedPoint3D::new(x, y, z), TypedSize3D::new(w, h, d))`.
pub fn cuboid<T: Copy, U>(x: T, y: T, z: T, w: T, h: T, d: T) -> TypedCuboid<T, U> {
    TypedCuboid::new(TypedPoint3D::new(x, y, z), TypedSize3D::new(w, h, d))
}
