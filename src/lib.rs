// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "unstable", feature(asm, repr_simd, test))]

extern crate heapsize;

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate serde;

#[cfg(test)]
extern crate rand;
#[cfg(feature = "unstable")]
extern crate test;
extern crate num_traits;

pub use matrix::Matrix4;
pub use matrix2d::Matrix2D;
pub use matrix4d::Matrix4D;
pub use point::{Point2D, Point3D, Point4D};
pub use ray::Ray3D;
pub use rect::Rect;
pub use side_offsets::SideOffsets2D;
#[cfg(feature = "unstable")] pub use side_offsets::SideOffsets2DSimdI32;
pub use size::Size2D;

pub mod approxeq;
pub mod length;
#[macro_use]
mod macros;
pub mod matrix;
pub mod matrix2d;
pub mod matrix4d;
pub mod num;
pub mod point;
pub mod ray;
pub mod rect;
pub mod scale_factor;
pub mod side_offsets;
pub mod size;
mod trig;
