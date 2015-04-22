// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A one-dimensional length, tagged with its units.

use std::num;


pub trait Zero {
    fn zero() -> Self;
}

impl<T: num::Zero> Zero for T {
    fn zero() -> T { num::Zero::zero() }
}

pub trait One {
    fn one() -> Self;
}

impl<T: num::One> One for T {
    fn one() -> T { num::One::one() }
}

