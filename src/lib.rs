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
pub use matrix2d::{Matrix2D, TypedMatrix2D};
pub use matrix4d::{Matrix4D, TypedMatrix4D};
pub use point::{
    Point2D, TypedPoint2D,
    Point3D, TypedPoint3D,
    Point4D, TypedPoint4D,
};
pub use rect::{Rect, TypedRect};
pub use side_offsets::{SideOffsets2D, TypedSideOffsets2D};
#[cfg(feature = "unstable")] pub use side_offsets::SideOffsets2DSimdI32;
pub use size::{Size2D, TypedSize2D};

pub mod approxeq;
pub mod length;
#[macro_use]
mod macros;
pub mod matrix;
pub mod matrix2d;
pub mod matrix4d;
pub mod num;
pub mod point;
pub mod rect;
pub mod scale_factor;
pub mod side_offsets;
pub mod size;
mod trig;
