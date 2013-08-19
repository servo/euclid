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

use std::num::Zero;

/// A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
/// and margins in CSS.
#[deriving(Eq)]
pub struct SideOffsets2D<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T> SideOffsets2D<T> {
    pub fn new(top: T, right: T, bottom: T, left: T) -> SideOffsets2D<T> {
        SideOffsets2D {
            top: top,
            right: right,
            bottom: bottom,
            left: left,
        }
    }
}

impl<T:Clone> SideOffsets2D<T> {
    pub fn new_all_same(all: T) -> SideOffsets2D<T> {
        SideOffsets2D::new(all.clone(), all.clone(), all.clone(), all.clone())
    }
}

impl<T:Num> SideOffsets2D<T> {
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }
}

impl<T:Num> Zero for SideOffsets2D<T> {
    fn zero() -> SideOffsets2D<T> {
        SideOffsets2D {
            top: Zero::zero(),
            right: Zero::zero(),
            bottom: Zero::zero(),
            left: Zero::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.top.is_zero() && self.right.is_zero() && self.bottom.is_zero() && self.left.is_zero()
    }
}

