// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use matrix4d::TypedMatrix4D;
use length::Untyped;

#[cfg_attr(feature = "unstable", deprecated(note = "Use matrix4d::Matrix4D instead"))]
pub type Matrix4 = TypedMatrix4D<f32, Untyped, Untyped>;
