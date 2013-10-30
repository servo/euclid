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

#[deriving(Eq, Clone)]
pub struct Size2D<T> {
    width: T,
    height: T
}

impl<T: fmt::Default> fmt::Default for Size2D<T> {
    fn fmt(obj: &Size2D<T>, f: &mut fmt::Formatter) {
        write!(f.buf, "{}×{}", obj.width, obj.height);
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

