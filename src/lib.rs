// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature="rustc_stable"), feature(asm, custom_derive, plugin, repr_simd, zero_one, test))]
#![cfg_attr(not(feature="rustc_stable"), feature(augmented_assignments))]
#![cfg_attr(not(feature="rustc_stable"), feature(op_assign_traits))]

#![cfg_attr(feature = "plugins", plugin(heapsize_plugin))]
#![cfg_attr(feature = "plugins", plugin(serde_macros))]

#[cfg(feature = "plugins")]
#[macro_use]
extern crate heapsize;

#[macro_use]
extern crate log;
extern crate rustc_serialize;
#[cfg(feature = "plugins")]
extern crate serde;

extern crate rand;
#[cfg(not(feature="rustc_stable"))]
extern crate test;
extern crate num as num_lib;

pub use matrix::Matrix4;
pub use matrix2d::Matrix2D;
pub use point::{Point2D, Point3D, Point4D};
pub use rect::Rect;
#[cfg(not(feature="rustc_stable"))]
pub use side_offsets::SideOffsets2D;
#[cfg(not(feature="rustc_stable"))]
pub use side_offsets::SideOffsets2DSimdI32;
pub use size::Size2D;

pub mod approxeq;
pub mod length;
pub mod matrix;
pub mod matrix2d;
//pub mod num;
pub mod point;
pub mod rect;
pub mod scale_factor;
#[cfg(not(feature="rustc_stable"))]
pub mod side_offsets;
pub mod size;
