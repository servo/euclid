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
#[deriving(Clone, PartialEq)]
pub struct SideOffsets2D<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
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

impl<T:Num> Add<SideOffsets2D<T>, SideOffsets2D<T>> for SideOffsets2D<T> {
    fn add(&self, other: &SideOffsets2D<T>) -> SideOffsets2D<T> {
        SideOffsets2D {
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
            left: self.left + other.left,
        }
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

/// A SIMD enabled version of SideOffsets2D specialized for i32.
#[deriving(Clone, PartialEq, Rand)]
#[simd]
pub struct SideOffsets2DSimdI32 {
    pub top: i32,
    pub bottom: i32,
    pub right: i32,
    pub left: i32,
}

impl SideOffsets2DSimdI32 {
    #[inline]
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> SideOffsets2DSimdI32 {
        SideOffsets2DSimdI32 {
            top: top,
            bottom: bottom,
            right: right,
            left: left,
        }
    }
}

impl SideOffsets2DSimdI32 {
    #[inline]
    pub fn new_all_same(all: i32) -> SideOffsets2DSimdI32 {
        SideOffsets2DSimdI32::new(all.clone(), all.clone(), all.clone(), all.clone())
    }
}

impl SideOffsets2DSimdI32 {
    #[inline]
    pub fn horizontal(&self) -> i32 {
        self.left + self.right
    }

    #[inline]
    pub fn vertical(&self) -> i32 {
        self.top + self.bottom
    }
}

impl Add<SideOffsets2DSimdI32, SideOffsets2DSimdI32> for SideOffsets2DSimdI32 {
    #[inline]
    fn add(&self, other: &SideOffsets2DSimdI32) -> SideOffsets2DSimdI32 {
        *self + *other // Use SIMD addition
    }
}

impl Zero for SideOffsets2DSimdI32 {
    #[inline]
    fn zero() -> SideOffsets2DSimdI32 {
        SideOffsets2DSimdI32 {
            top: Zero::zero(),
            bottom: Zero::zero(),
            right: Zero::zero(),
            left: Zero::zero(),
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    #[inline]
    fn is_zero(&self) -> bool {
        self.top.is_zero() && self.right.is_zero() && self.bottom.is_zero() && self.left.is_zero()
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    fn is_zero(&self) -> bool {
        let is_zero: bool;
        unsafe {
            asm! {
                "ptest $1, $1
                 setz $0"
                : "=r"(is_zero)
                : "x"(*self)
                :
                : "intel"
            };
        }
        is_zero
    }
}

#[cfg(test)]
mod tests {
    use std::num::Zero;
    use super::SideOffsets2DSimdI32;

    #[test]
    fn test_is_zero() {
        assert!(SideOffsets2DSimdI32::new_all_same(0).is_zero());
        assert!(!SideOffsets2DSimdI32::new_all_same(1).is_zero());
        assert!(!SideOffsets2DSimdI32::new(1, 0, 0, 0).is_zero());
        assert!(!SideOffsets2DSimdI32::new(0, 1, 0, 0).is_zero());
        assert!(!SideOffsets2DSimdI32::new(0, 0, 1, 0).is_zero());
        assert!(!SideOffsets2DSimdI32::new(0, 0, 0, 1).is_zero());
    }
}

#[cfg(bench)]
mod bench {
    use test::BenchHarness;
    use std::num::Zero;
    use rand::{XorShiftRng, Rng};
    use super::SideOffsets2DSimdI32;

    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "x86_64")]
    #[bench]
    fn bench_naive_is_zero(bh: &mut BenchHarness) {
        fn is_zero(x: &SideOffsets2DSimdI32) -> bool {
            x.top.is_zero() && x.right.is_zero() && x.bottom.is_zero() && x.left.is_zero()
        }
        let mut rng = XorShiftRng::new().unwrap();
        bh.iter(|| is_zero(&rng.gen::<SideOffsets2DSimdI32>()))
    }

    #[bench]
    fn bench_is_zero(bh: &mut BenchHarness) {
        let mut rng = XorShiftRng::new().unwrap();
        bh.iter(|| rng.gen::<SideOffsets2DSimdI32>().is_zero())
    }

    #[bench]
    fn bench_naive_add(bh: &mut BenchHarness) {
        fn add(x: &SideOffsets2DSimdI32, y: &SideOffsets2DSimdI32) -> SideOffsets2DSimdI32 {
            SideOffsets2DSimdI32 {
                top: x.top + y.top,
                right: x.right + y.right,
                bottom: x.bottom + y.bottom,
                left: x.left + y.left,
            }
        }
        let mut rng = XorShiftRng::new().unwrap();
        bh.iter(|| add(&rng.gen::<SideOffsets2DSimdI32>(), &rng.gen::<SideOffsets2DSimdI32>()))
    }

    #[bench]
    fn bench_add(bh: &mut BenchHarness) {
        let mut rng = XorShiftRng::new().unwrap();
        bh.iter(|| rng.gen::<SideOffsets2DSimdI32>() + rng.gen::<SideOffsets2DSimdI32>())
    }
}
