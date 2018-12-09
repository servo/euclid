// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
//! and margins in CSS.

use super::UnknownUnit;
use length::Length;
use num::Zero;
use core::fmt;
use core::ops::Add;
use core::marker::PhantomData;

/// A group of 2D side offsets, which correspond to top/left/bottom/right for borders, padding,
/// and margins in CSS, optionally tagged with a unit.
#[derive(EuclidMatrix)]
#[repr(C)]
pub struct TypedSideOffsets2D<T, U> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T: fmt::Debug, U> fmt::Debug for TypedSideOffsets2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?},{:?},{:?},{:?})",
            self.top, self.right, self.bottom, self.left
        )
    }
}

/// The default 2D side offset type with no unit.
pub type SideOffsets2D<T> = TypedSideOffsets2D<T, UnknownUnit>;

impl<T: Copy, U> TypedSideOffsets2D<T, U> {
    /// Constructor taking a scalar for each side.
    pub fn new(top: T, right: T, bottom: T, left: T) -> Self {
        TypedSideOffsets2D {
            top,
            right,
            bottom,
            left,
            _unit: PhantomData,
        }
    }

    /// Constructor taking a typed Length for each side.
    pub fn from_lengths(
        top: Length<T, U>,
        right: Length<T, U>,
        bottom: Length<T, U>,
        left: Length<T, U>,
    ) -> Self {
        TypedSideOffsets2D::new(top.0, right.0, bottom.0, left.0)
    }

    /// Access self.top as a typed Length instead of a scalar value.
    pub fn top_typed(&self) -> Length<T, U> {
        Length::new(self.top)
    }

    /// Access self.right as a typed Length instead of a scalar value.
    pub fn right_typed(&self) -> Length<T, U> {
        Length::new(self.right)
    }

    /// Access self.bottom as a typed Length instead of a scalar value.
    pub fn bottom_typed(&self) -> Length<T, U> {
        Length::new(self.bottom)
    }

    /// Access self.left as a typed Length instead of a scalar value.
    pub fn left_typed(&self) -> Length<T, U> {
        Length::new(self.left)
    }

    /// Constructor setting the same value to all sides, taking a scalar value directly.
    pub fn new_all_same(all: T) -> Self {
        TypedSideOffsets2D::new(all, all, all, all)
    }

    /// Constructor setting the same value to all sides, taking a typed Length.
    pub fn from_length_all_same(all: Length<T, U>) -> Self {
        TypedSideOffsets2D::new_all_same(all.0)
    }
}

impl<T, U> TypedSideOffsets2D<T, U>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }

    pub fn horizontal_typed(&self) -> Length<T, U> {
        Length::new(self.horizontal())
    }

    pub fn vertical_typed(&self) -> Length<T, U> {
        Length::new(self.vertical())
    }
}

impl<T, U> Add for TypedSideOffsets2D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        TypedSideOffsets2D::new(
            self.top + other.top,
            self.right + other.right,
            self.bottom + other.bottom,
            self.left + other.left,
        )
    }
}

impl<T: Copy + Zero, U> TypedSideOffsets2D<T, U> {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedSideOffsets2D::new(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }
}

/// 3D offsets for a cuboid: front, left, right, back, top, bottom.
#[derive(EuclidMatrix)]
#[repr(C)]
pub struct TypedSideOffsets3D<T, U> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
    pub front: T,
    pub back: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T: fmt::Debug, U> fmt::Debug for TypedSideOffsets3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?},{:?},{:?},{:?},{:?},{:?})",
            self.top, self.right, self.bottom, self.left, self.front, self.back
        )
    }
}

/// The default 3D side offset type with no unit.
pub type SideOffsets3D<T> = TypedSideOffsets3D<T, UnknownUnit>;

impl<T: Copy, U> TypedSideOffsets3D<T, U> {
    /// Constructor taking a scalar for each side.
    pub fn new(top: T, right: T, bottom: T, left: T, front: T, back: T) -> Self {
        TypedSideOffsets3D {
            top,
            right,
            bottom,
            left,
            front,
            back,
            _unit: PhantomData,
        }
    }

    /// Constructor taking a typed Length for each side.
    pub fn from_lengths(
        top: Length<T, U>,
        right: Length<T, U>,
        bottom: Length<T, U>,
        left: Length<T, U>,
        front: Length<T, U>,
        back: Length<T, U>,
    ) -> Self {
        TypedSideOffsets3D::new(top.0, right.0, bottom.0, left.0, front.0, back.0)
    }

    /// Access self.top as a typed Length instead of a scalar value.
    pub fn top_typed(&self) -> Length<T, U> {
        Length::new(self.top)
    }

    /// Access self.right as a typed Length instead of a scalar value.
    pub fn right_typed(&self) -> Length<T, U> {
        Length::new(self.right)
    }

    /// Access self.bottom as a typed Length instead of a scalar value.
    pub fn bottom_typed(&self) -> Length<T, U> {
        Length::new(self.bottom)
    }

    /// Access self.left as a typed Length instead of a scalar value.
    pub fn left_typed(&self) -> Length<T, U> {
        Length::new(self.left)
    }

    /// Access self.front as a typed Length instead of a scalar value.
    pub fn front_typed(&self) -> Length<T, U> {
        Length::new(self.front)
    }

    /// Access self.back as a typed Length instead of a scalar value.
    pub fn back_typed(&self) -> Length<T, U> {
        Length::new(self.back)
    }

    /// Constructor setting the same value to all sides, taking a scalar value directly.
    pub fn new_all_same(all: T) -> Self {
        TypedSideOffsets3D::new(all, all, all, all, all, all)
    }

    /// Constructor setting the same value to all sides, taking a typed Length.
    pub fn from_length_all_same(all: Length<T, U>) -> Self {
        TypedSideOffsets3D::new_all_same(all.0)
    }
}

impl<T, U> TypedSideOffsets3D<T, U>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }

    pub fn applicate(&self) -> T {
        self.front + self.back
    }

    pub fn horizontal_typed(&self) -> Length<T, U> {
        Length::new(self.horizontal())
    }

    pub fn vertical_typed(&self) -> Length<T, U> {
        Length::new(self.vertical())
    }

    pub fn applicate_typed(&self) -> Length<T, U> {
        Length::new(self.applicate())
    }
}

impl<T, U> Add for TypedSideOffsets3D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        TypedSideOffsets3D::new(
            self.top + other.top,
            self.right + other.right,
            self.bottom + other.bottom,
            self.left + other.left,
            self.front + other.front,
            self.back + other.back,
        )
    }
}

impl<T: Copy + Zero, U> TypedSideOffsets3D<T, U> {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedSideOffsets3D::new(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }
}
