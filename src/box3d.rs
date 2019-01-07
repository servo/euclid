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
/// The axis directions follows those used by Microsoft:
/// https://docs.microsoft.com/en-us/dotnet/framework/wpf/graphics-multimedia/3-d-transformations-overview
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
    T: Copy + Div<T, Output = T> + Neg<Output = T> + Add<T, Output = T> + One,
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
    T: Copy,
{
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
    pub fn top_left_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.min_x(), self.max_y(), self.max_z())
    }

    #[inline]
    pub fn top_right_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.max_z())
    }

    #[inline]
    pub fn bottom_left_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.min_x(), self.min_y(), self.max_z())
    }

    #[inline]
    pub fn bottom_right_front(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.min_y(), self.max_z())
    }

    #[inline]
    pub fn top_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.min_x(), self.max_y(), self.min_z())
    }

    #[inline]
    pub fn top_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.max_y(), self.min_z())
    }

    #[inline]
    pub fn bottom_left_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.min_x(), self.min_y(), self.min_z())
    }

    #[inline]
    pub fn bottom_right_back(&self) -> TypedPoint3D<T, U> {
        TypedPoint3D::new(self.max_x(), self.min_y(), self.min_z())
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
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + PartialOrd,
{
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_x() < other.max_x()
            && self.max_x() > other.min_x()
            && self.min_y() < other.max_y()
            && self.max_y() > other.min_y()
            && self.min_z() < other.max_z()
            && self.max_z() > other.min_z()
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
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    /// Returns the same box3d, translated by a vector.
    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate(&self, by: &TypedVector3D<T, U>) -> Self {
        Self::new(self.a + *by, self.b + *by)
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn translate_by_size(&self, b: &TypedPoint3D<T, U>) -> Self {
        self.translate(&b.to_vector())
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + PartialOrd + Zero,
{
    /// Returns true if this box3d contains the point. Points are considered
    /// in the box3d if they are on the front, left or top faces, but outside if they
    /// are on the back, right or bottom faces.
    #[inline]
    pub fn contains(&self, other: &TypedPoint3D<T, U>) -> bool {
        self.a.x <= other.x && other.x < self.b.x
            && self.a.y >= other.y && other.y > self.b.y
            && self.a.z >= other.z && other.z > self.b.z
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + PartialOrd + Zero + Sub<T, Output = T>,
{
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
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Sub<T, Output = T>,
{
    #[inline]
    pub fn size(&self)-> TypedSize3D<T, U> {
        TypedSize3D::new(
            self.max_x() - self.min_x(),
            self.max_y() - self.min_y(),
            self.max_z() - self.min_z(),
        )
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + PartialEq + Add<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + One,
{
    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn inflate(&self, width: T, height: T, depth: T) -> Self {
        let two = T::one() + T::one();
        TypedBox3D::new(
            TypedPoint3D::new(self.a.x - width / two, self.a.y + height / two, self.a.z + depth / two),
            TypedPoint3D::new(self.b.x + width / two, self.b.y - height / two, self.b.z - depth / two),
        )
    }

    #[inline]
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn inflate_typed(&self, width: Length<T, U>, height: Length<T, U>, depth: Length<T, U>) -> Self {
        self.inflate(width.get(), height.get(), depth.get())
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Zero + PartialOrd + Add<T, Output = T> + Sub<T, Output = T>,
{
    /// Calculate the size and position of an inner box3d.
    ///
    /// Subtracts the side offsets from all sides. The horizontal, vertical
    /// and applicate offsets must not be larger than the original side length.
    pub fn inner_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        let box3d = TypedBox3D::new(
            TypedPoint3D::new(
                self.a.x + offsets.left,
                self.a.y - offsets.top,
                self.a.z - offsets.front,
            ),
            TypedPoint3D::new(
                self.b.x - offsets.right,
                self.b.y + offsets.bottom,
                self.b.z + offsets.back,
            ),
        );
        debug_assert!(box3d.size().width >= T::zero());
        debug_assert!(box3d.size().height >= T::zero());
        debug_assert!(box3d.size().depth >= T::zero());
        box3d
    }

    /// Calculate the b and position of an outer box3d.
    ///
    /// Add the offsets to all sides. The expanded box3d is returned.
    pub fn outer_box(&self, offsets: TypedSideOffsets3D<T, U>) -> Self {
        TypedBox3D::new(
            TypedPoint3D::new(
                self.a.x - offsets.left,
                self.a.y + offsets.top,
                self.a.z + offsets.front,
            ),
            TypedPoint3D::new(
                self.b.x + offsets.right,
                self.b.y - offsets.bottom,
                self.b.z - offsets.back,
            ),
        )
    }
}


impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Zero + PartialOrd,
{
    /// Returns the smallest box containing all of the provided points.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<TypedPoint3D<T, U>>,
    {
        let mut points = points.into_iter();

        // Need at least 2 different points for a valid box3d (ie: volume > 0).
        let (mut min_x, mut min_y, mut min_z) = match points.next() {
            Some(first) => (first.borrow().x, first.borrow().y, first.borrow().z),
            None => return TypedBox3D::zero(),
        };
        let (mut max_x, mut max_y, mut max_z) = (min_x, min_y, min_z);

        {
            let mut assign_min_max = |point: I::Item| {
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
                if p.z > max_z {
                    max_z = p.z
                }
            };
                    
            match points.next() {
                Some(second) => assign_min_max(second),
                None => return TypedBox3D::zero(),
            }

            for point in points {
                assign_min_max(point);
            }
        }

        Self::from_min_max(min_x, min_y, min_z, max_x, max_y, max_z)
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + PartialOrd,
{
    pub fn from_min_max(min_x: T, min_y: T, min_z: T, max_x: T, max_y: T, max_z: T) -> Self {
        debug_assert!(min_x <= max_x);
        debug_assert!(min_y <= max_y);
        debug_assert!(min_z <= max_z);

        Self::new(
            TypedPoint3D::new(min_x, max_y, max_z),
            TypedPoint3D::new(max_x, min_y, min_z),
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
        (self.a + self.b.to_vector()) / two
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Clone + PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Zero,
{
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        TypedBox3D::from_min_max(
            min(self.min_x(), other.min_x()),
            min(self.min_y(), other.min_y()),
            min(self.min_z(), other.min_z()),
            max(self.max_x(), other.max_x()),
            max(self.max_y(), other.max_z()),
            max(self.max_y(), other.max_z()),
        )
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy,
{
    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S, z: S) -> Self
    where
        T: Mul<S, Output = T>
    {
        TypedBox3D::new(
            TypedPoint3D::new(self.a.x * x, self.a.y * y, self.a.z * z),
            TypedPoint3D::new(self.b.x * x, self.b.y * y, self.b.z * z),
        )
    }
}

impl<T, U> TypedBox3D<T, U>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    #[inline]
    pub fn volume(&self) -> T {
        let size = self.size();
        size.width * size.height * size.depth
    }

    #[inline]
    pub fn front_area(&self) -> T {
        let size = self.size();
        size.width * size.height
    }

    #[inline]
    pub fn left_area(&self) -> T {
        let size = self.size();
        size.depth * size.height
    }

    #[inline]
    pub fn top_area(&self) -> T {
        let size = self.size();
        size.depth * size.width
    }

    #[inline]
    pub fn back_area(&self) -> T {
        self.front_area()
    }

    #[inline]
    pub fn right_area(&self) -> T {
        self.left_area()
    }

    #[inline]
    pub fn bottom_area(&self) -> T {
        self.top_area()
    }
}

impl<T, U> TypedBox3D<T, U> 
where
    T: Copy + Zero,
{
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedBox3D::new(TypedPoint3D::zero(), TypedPoint3D::zero())
    }
}

impl<T, U> TypedBox3D<T, U> 
where
    T: Copy + PartialEq + Zero + Sub<T, Output = T>,
{
    /// Returns true if the size is zero, regardless of a or b's value.
    pub fn is_empty(&self) -> bool {
        let size = self.size();
        size.width == Zero::zero() || size.height == Zero::zero() || size.depth == Zero::zero()
    }
}

impl<T, U> Mul<T> for TypedBox3D<T, U> 
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Self;
    #[inline]
    fn mul(self, scale: T) -> Self {
        TypedBox3D::new(self.a * scale, self.b * scale)
    }
}

impl<T, U> Div<T> for TypedBox3D<T, U> 
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Self;
    #[inline]
    fn div(self, scale: T) -> Self {
        TypedBox3D::new(self.a / scale, self.b / scale)
    }
}

impl<T, U1, U2> Mul<TypedScale<T, U1, U2>> for TypedBox3D<T, U1> 
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = TypedBox3D<T, U2>;
    #[inline]
    fn mul(self, scale: TypedScale<T, U1, U2>) -> TypedBox3D<T, U2> {
        TypedBox3D::new(self.a * scale, self.b * scale)
    }
}

impl<T, U1, U2> Div<TypedScale<T, U1, U2>> for TypedBox3D<T, U2> 
where
    T: Copy + Div<T, Output = T>,
{
    type Output = TypedBox3D<T, U1>;
    #[inline]
    fn div(self, scale: TypedScale<T, U1, U2>) -> TypedBox3D<T, U1> {
        TypedBox3D::new(self.a / scale, self.b / scale)
    }
}

impl<T, Unit> TypedBox3D<T, Unit> 
where
    T: Copy,
{
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Box3D<T> {
        TypedBox3D::new(self.a.to_untyped(), self.b.to_untyped())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(c: &Box3D<T>) -> TypedBox3D<T, Unit> {
        TypedBox3D::new(
            TypedPoint3D::from_untyped(&c.a),
            TypedPoint3D::from_untyped(&c.b),
        )
    }
}

impl<T0, Unit> TypedBox3D<T0, Unit> 
where
    T0: NumCast + Copy,
{
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

impl<T, U> TypedBox3D<T, U> 
where
    T: Round,
{
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
        TypedBox3D::new(self.a.round(), self.b.round())
    }
}

impl<T, U> TypedBox3D<T, U> 
where
    T: Floor + Ceil,
{
    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d contains the resulting box3d.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_in(&self) -> Self {
        let a_x = self.a.x.ceil();
        let a_y = self.a.y.floor();
        let a_z = self.a.z.floor();
        let b_x = self.b.x.floor();
        let b_y = self.b.y.ceil();
        let b_z = self.b.z.ceil();
        TypedBox3D::new(
            TypedPoint3D::new(a_x, a_y, a_z), 
            TypedPoint3D::new(b_x, b_y, b_z),
        )
    }

    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d is contained in the resulting box3d.
    #[cfg_attr(feature = "unstable", must_use)]
    pub fn round_out(&self) -> Self {
        let a_x = self.a.x.floor();
        let a_y = self.a.y.ceil();
        let a_z = self.a.z.ceil();
        let b_x = self.b.x.ceil();
        let b_y = self.b.y.floor();
        let b_z = self.b.z.floor();
        TypedBox3D::new(
            TypedPoint3D::new(a_x, a_y, a_z), 
            TypedPoint3D::new(b_x, b_y, b_z),
        )
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
where 
    T: Copy + Div<T, Output = T> + Neg<Output = T> + Add<T, Output = T> + One,
{
    fn from(b: TypedSize3D<T, U>) -> Self {
        Self::from_size(b)
    }
}

/// Shorthand for `TypedBox3D::new(TypedPoint3D::new(x1, y1, z1), TypedPoint3D::new(x2, y2, z2))`.
pub fn box3d<T: Copy, U>(tlf_x: T, tlf_y: T, tlf_z: T, brb_x: T, brb_y: T, brb_z: T) -> TypedBox3D<T, U> {
    TypedBox3D::new(TypedPoint3D::new(tlf_x, tlf_y, tlf_z), TypedPoint3D::new(brb_x, brb_y, brb_z))
}

#[cfg(test)]
mod tests {
    use vector::vec3;
    use side_offsets::SideOffsets3D;
    use size::size3;
    use point::{point3, Point3D};
    use super::*;

    #[test]
    fn test_new() {
        let b = Box3D::new(point3(-1.0, 1.0, 1.0), point3(1.0, -1.0, -1.0));
        assert!(b.a.x == -1.0);
        assert!(b.a.y == 1.0);
        assert!(b.a.z == 1.0);
        assert!(b.b.x == 1.0);
        assert!(b.b.y == -1.0);
        assert!(b.b.z == -1.0);
    }

    #[test]
    fn test_size() {
        let b = Box3D::new(point3(-10.0, 10.0, 10.0), point3(10.0, -10.0, -10.0));
        assert!(b.size().width == 20.0);
        assert!(b.size().height == 20.0);
        assert!(b.size().depth == 20.0);
    }

    #[test]
    fn test_center() {
        let b = Box3D::new(point3(-10.0, 10.0, 10.0), point3(10.0, -10.0, -10.0));
        assert!(b.center() == Point3D::zero());
    }

    #[test]
    fn test_volume() {
        let b = Box3D::new(point3(-10.0, 10.0, 10.0), point3(10.0, -10.0, -10.0));
        assert!(b.volume() == 8000.0);
    }

    #[test]
    fn test_area() {
        let b = Box3D::new(point3(-10.0, 10.0, 10.0), point3(10.0, -10.0, -10.0));
        assert!(b.front_area() == 400.0);
        assert!(b.left_area() == 400.0);
        assert!(b.right_area() == 400.0);
        assert!(b.back_area() == 400.0);
        assert!(b.top_area() == 400.0);
        assert!(b.bottom_area() == 400.0);
    }

    #[test]
    fn test_from_points() {
        let b = Box3D::from_points(&[point3(50.0, 25.0, 12.5), point3(100.0, 160.0, 200.0)]);
        assert!(b.a == point3(50.0, 160.0, 200.0));
        assert!(b.b == point3(100.0, 25.0, 12.5));
    }

    #[test]
    fn test_min_max() {
        let b = Box3D::from_points(&[point3(50.0, 25.0, 12.5), point3(100.0, 160.0, 200.0)]);
        assert!(b.min_x() == 50.0);
        assert!(b.min_y() == 25.0);
        assert!(b.min_z() == 12.5);
        assert!(b.max_x() == 100.0);
        assert!(b.max_y() == 160.0);
        assert!(b.max_z() == 200.0);
    }

    #[test]
    fn test_from_min_max() {
        let b = Box3D::from_min_max(10.0, 20.0, 30.0, 40.0, 50.0, 60.0);
        assert!(b.a == point3(10.0, 50.0, 60.0));
        assert!(b.b == point3(40.0, 20.0, 30.0));
    }

    #[test]
    fn test_round_in() {
        let b = Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)]).round_in();
        assert!(b.min_x() == -25.0);
        assert!(b.min_y() == -40.0);
        assert!(b.min_z() == -70.0);
        assert!(b.max_x() == 60.0);
        assert!(b.max_y() == 36.0);
        assert!(b.max_z() == 89.0);
    }

    #[test]
    fn test_round_out() {
        let b = Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)]).round_out();
        assert!(b.min_x() == -26.0);
        assert!(b.min_y() == -41.0);
        assert!(b.min_z() == -71.0);
        assert!(b.max_x() == 61.0);
        assert!(b.max_y() == 37.0);
        assert!(b.max_z() == 90.0);
    }

    #[test]
    fn test_round() {
        let b = Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)]).round();
        assert!(b.min_x() == -26.0);
        assert!(b.min_y() == -40.0);
        assert!(b.min_z() == -71.0);
        assert!(b.max_x() == 60.0);
        assert!(b.max_y() == 37.0);
        assert!(b.max_z() == 90.0);
    }

    #[test]
    fn test_from_size() {
        let b = Box3D::from_size(size3(30.0, 40.0, 50.0));
        assert!(b.center() == Point3D::zero());
        assert!(b.size().width == 30.0);
        assert!(b.size().height == 40.0);
        assert!(b.size().depth == 50.0);
    }

    #[test]
    fn test_inner_box() {
        let b = Box3D::from_points(&[point3(50.0, 25.0, 12.5), point3(100.0, 160.0, 200.0)]);
        let b = b.inner_box(SideOffsets3D::new(10.0, 20.0, 5.0, 10.0, 20.0, 5.0));
        assert!(b.max_x() == 80.0);
        assert!(b.max_y() == 150.0);
        assert!(b.max_z() == 180.0);
        assert!(b.min_x() == 60.0);
        assert!(b.min_y() == 30.0);
        assert!(b.min_z() == 17.5);
    }

    #[test]
    fn test_outer_box() {
        let b = Box3D::from_points(&[point3(50.0, 25.0, 12.5), point3(100.0, 160.0, 200.0)]);
        let b = b.outer_box(SideOffsets3D::new(10.0, 20.0, 5.0, 10.0, 20.0, 5.0));
        assert!(b.max_x() == 120.0);
        assert!(b.max_y() == 170.0);
        assert!(b.max_z() == 220.0);
        assert!(b.min_x() == 40.0);
        assert!(b.min_y() == 20.0);
        assert!(b.min_z() == 7.5);
    }

    #[test]
    fn test_translate() {
        let b = Box3D::from_size(size3(15.0, 15.0, 200.0));
        assert!(b.center() == Point3D::zero());
        let b = b.translate(&vec3(10.0, 0.0, 0.0));
        assert!(b.center() == point3(10.0, 0.0, 0.0));
        assert!(b.max_x() == 17.5);
        assert!(b.max_y() == 7.5);
        assert!(b.max_z() == 100.0);
        assert!(b.min_x() == 2.5);
        assert!(b.min_y() == -7.5);
        assert!(b.min_z() == -100.0);
    }

    #[test]
    fn test_union() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(0.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(0.0, 20.0, 20.0), point3(20.0, -20.0, -20.0)]);
        let b = b1.union(&b2);
        assert!(b.max_x() == 20.0);
        assert!(b.max_y() == 20.0);
        assert!(b.max_z() == 20.0);
        assert!(b.min_x() == -20.0);
        assert!(b.min_y() == -20.0);
        assert!(b.min_z() == -20.0);
        assert!(b.volume() == (40.0 * 40.0 * 40.0));
    }

    #[test]
    fn test_intersection() {
        let b1 = Box3D::from_points(&[point3(-15.0, -20.0, -20.0), point3(10.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-10.0, 20.0, 20.0), point3(15.0, -20.0, -20.0)]);
        let b = b1.intersection(&b2).unwrap();
        assert!(b.max_x() == 10.0);
        assert!(b.max_y() == 20.0);
        assert!(b.max_z() == 20.0);
        assert!(b.min_x() == -10.0);
        assert!(b.min_y() == -20.0);
        assert!(b.min_z() == -20.0);
        assert!(b.volume() == (20.0 * 40.0 * 40.0));
    }

    #[test]
    fn test_scale() {
        let b = Box3D::from_points(&[point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0)]);
        let b = b.scale(0.5, 0.5, 0.5);
        assert!(b.max_x() == 5.0);
        assert!(b.max_y() == 5.0);
        assert!(b.max_z() == 5.0);
        assert!(b.min_x() == -5.0);
        assert!(b.min_y() == -5.0);
        assert!(b.min_z() == -5.0);
    }

    #[test]
    fn test_zero() {
        let b = Box3D::<f64>::zero();
        assert!(b.max_x() == 0.0);
        assert!(b.max_y() == 0.0);
        assert!(b.max_z() == 0.0);
        assert!(b.min_x() == 0.0);
        assert!(b.min_y() == 0.0);
        assert!(b.min_z() == 0.0);
    }

    #[test]
    fn test_lerp() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(-10.0, -10.0, -10.0)]);
        let b2 = Box3D::from_points(&[point3(10.0, 10.0, 10.0), point3(20.0, 20.0, 20.0)]);
        let b = b1.lerp(b2, 0.5);
        assert!(b.center() == Point3D::zero());
        assert!(b.size().width == 10.0);
        assert!(b.size().height == 10.0);
        assert!(b.size().depth == 10.0);
    }

    #[test]
    fn test_contains() {
        let b = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        assert!(b.contains(&point3(-15.3, 10.5, 18.4)));
    }

    #[test]
    fn test_contains_box() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-14.3, -16.5, -19.3), point3(6.7, 17.6, 2.5)]);
        assert!(b1.contains_box(&b2));
    }

    #[test]
    fn test_inflate() {
        let b = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        let b = b.inflate(10.0, 5.0, 2.0);
        assert!(b.size().width == 50.0);
        assert!(b.size().height == 45.0);
        assert!(b.size().depth == 42.0);
        assert!(b.center() == Point3D::zero());
    }

    #[test]
    fn test_is_empty() {
        for i in 0..3 {
            let mut coords_neg = [-20.0, -20.0, -20.0];
            let mut coords_pos = [20.0, 20.0, 20.0];
            coords_neg[i] = 0.0;
            coords_pos[i] = 0.0;
            let b = Box3D::from_points(&[Point3D::from(coords_neg), Point3D::from(coords_pos)]);
            assert!(b.is_empty());
        }
    }
}
