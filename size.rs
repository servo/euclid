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

#[deriving(Clone, Decodable, Default, Encodable, Eq)]
pub struct Size2D<T> {
    pub width: T,
    pub height: T
}

impl<T: fmt::Show> fmt::Show for Size2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}×{}", self.width, self.height)
    }
}

impl<T:Clone + Add<T,T>> Add<Size2D<T>, Size2D<T>> for Size2D<T> {
    fn add(&self, other: &Size2D<T>) -> Size2D<T> {
        Size2D(self.width + other.width, self.height + other.height)
    }
}

pub fn Size2D<T: Clone>(width: T, height: T) -> Size2D<T> {
    return Size2D {
        width: width,
        height: height
    }
}

impl<T:Clone + Mul<T,T>> Size2D<T> {
    pub fn area(&self) -> T { self.width * self.height }
}

