// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Eq;
use std::fmt;
use std::num::Zero;

#[deriving(Clone, Decodable, Encodable, Eq)]
pub struct Size2D<T> {
    pub width: T,
    pub height: T
}

impl<T: fmt::Show> fmt::Show for Size2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}×{}", self.width, self.height)
    }
}

pub fn Size2D<T: Clone>(width: T, height: T) -> Size2D<T> {
    Size2D {
        width: width,
        height: height
    }
}

impl<T:Clone + Mul<T,U>, U> Size2D<T> {
    pub fn area(&self) -> U { self.width * self.height }
}

impl<T:Clone + Zero> Size2D<T> {
    pub fn zero() -> Size2D<T> {
        Size2D {
            width: Zero::zero(),
            height: Zero::zero(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width.is_zero() || self.height.is_zero()
    }
}
