// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A one-dimensional length, tagged with its units.

use std::num::Int;


pub trait Zero {
    fn zero() -> Self;
}

impl<T: Int> Zero for T {
    #[inline]
    fn zero() -> T { Int::zero() }
}

impl Zero for f32 {
    #[inline]
    fn zero() -> f32 { 0. }
}

impl Zero for f64 {
    #[inline]
    fn zero() -> f64 { 0. }
}

pub trait One {
    fn one() -> Self;
}

impl<T: Int> One for T {
    #[inline]
    fn one() -> T { Int::one() }
}

impl One for f32 {
    #[inline]
    fn one() -> f32 { 1. }
}

impl One for f64 {
    #[inline]
    fn one() -> f64 { 1. }
}
