// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(test)]
use point::{Point2D};
use matrix4d::Matrix4D;

#[cfg_attr(feature = "unstable", deprecated(note = "Use matrix4d::Matrix4D instead"))]
pub type Matrix4 = Matrix4D<f32>;

#[test]
pub fn test_ortho() {
    let (left, right, bottom, top) = (0.0f32, 1.0f32, 0.1f32, 1.0f32);
    let (near, far) = (-1.0f32, 1.0f32);
    let result = Matrix4::ortho(left, right, bottom, top, near, far);
    let expected = Matrix4::new(2.0,  0.0,         0.0,  0.0,
                                0.0,  2.22222222,  0.0,  0.0,
                                0.0,  0.0,         -1.0, 0.0,
                                -1.0, -1.22222222, -0.0, 1.0);
    debug!("result={:?} expected={:?}", result, expected);
    assert!(result.approx_eq(&expected));
}

#[test]
pub fn test_is_2d() {
    assert!(Matrix4::identity().is_2d());
    assert!(Matrix4::create_rotation(0.0, 0.0, 1.0, 0.7854).is_2d());
    assert!(!Matrix4::create_rotation(0.0, 1.0, 0.0, 0.7854).is_2d());
}

#[test]
pub fn test_new_2d() {
    let m1 = Matrix4::new_2d(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let m2 = Matrix4::new(1.0, 2.0, 0.0, 0.0,
                          3.0, 4.0, 0.0, 0.0,
                          0.0, 0.0, 1.0, 0.0,
                          5.0, 6.0, 0.0, 1.0);
    assert_eq!(m1, m2);
}

#[test]
pub fn test_invert_simple() {
    let m1 = Matrix4::identity();
    let m2 = m1.invert();
    assert!(m1.approx_eq(&m2));
}

#[test]
pub fn test_invert_scale() {
    let m1 = Matrix4::create_scale(1.5, 0.3, 2.1);
    let m2 = m1.invert();
    assert!(m1.mul(&m2).approx_eq(&Matrix4::identity()));
}

#[test]
pub fn test_invert_translate() {
    let m1 = Matrix4::create_translation(-132.0, 0.3, 493.0);
    let m2 = m1.invert();
    assert!(m1.mul(&m2).approx_eq(&Matrix4::identity()));
}

#[test]
pub fn test_invert_rotate() {
    let m1 = Matrix4::create_rotation(0.0, 1.0, 0.0, 1.57);
    let m2 = m1.invert();
    assert!(m1.mul(&m2).approx_eq(&Matrix4::identity()));
}

#[test]
pub fn test_invert_transform_point_2d() {
    let m1 = Matrix4::create_translation(100.0, 200.0, 0.0);
    let m2 = m1.invert();
    assert!(m1.mul(&m2).approx_eq(&Matrix4::identity()));

    let p1 = Point2D::new(1000.0, 2000.0);
    let p2 = m1.transform_point(&p1);
    assert!(p2.eq(&Point2D::new(1100.0, 2200.0)));

    let p3 = m2.transform_point(&p2);
    assert!(p3.eq(&p1));
}
