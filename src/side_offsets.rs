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
use num::{Zero, ValueOrLength};
use std::fmt;
use std::ops::Add;
use std::marker::PhantomData;

/// A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
/// and margins in CSS, optionally tagged with a unit.
define_matrix! {
    pub struct TypedSideOffsets2D<T, U> {
        pub top: T,
        pub right: T,
        pub bottom: T,
        pub left: T,
    }
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

/// The default side offset type with no unit.
pub type SideOffsets2D<T> = TypedSideOffsets2D<T, UnknownUnit>;


impl<T, U> TypedSideOffsets2D<T, U> {
    /// Constructor taking a scalar or a `Length` for each side.
    pub fn new<N>(top: N, right: N, bottom: N, left: N) -> Self
    where N: ValueOrLength<T, U> {
        TypedSideOffsets2D {
            top: top.value(),
            right: right.value(),
            bottom: bottom.value(),
            left: left.value(),
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, U> TypedSideOffsets2D<T, U> {
    /// Access self.top as a typed Length instead of a scalar value.
    pub fn get_top(&self) -> Length<T, U> {
        Length::new(self.top)
    }

    /// Access self.right as a typed Length instead of a scalar value.
    pub fn get_right(&self) -> Length<T, U> {
        Length::new(self.right)
    }

    /// Access self.bottom as a typed Length instead of a scalar value.
    pub fn get_bottom(&self) -> Length<T, U> {
        Length::new(self.bottom)
    }

    /// Access self.left as a typed Length instead of a scalar value.
    pub fn get_left(&self) -> Length<T, U> {
        Length::new(self.left)
    }

    /// Constructor setting the same value to all sides, taking a scalar value directly.
    pub fn new_all_same<N>(all: N) -> Self
    where N: ValueOrLength<T, U> {
        let v = all.value();
        TypedSideOffsets2D::new(v, v, v, v)
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

    pub fn get_horizontal(&self) -> Length<T, U> {
        Length::new(self.horizontal())
    }

    pub fn get_vertical(&self) -> Length<T, U> {
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
        TypedSideOffsets2D::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}
