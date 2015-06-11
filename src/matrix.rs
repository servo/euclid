// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use approxeq::ApproxEq;
use point::{Point2D, Point3D};

pub fn Matrix4(
        m11: f32, m12: f32, m13: f32, m14: f32,
        m21: f32, m22: f32, m23: f32, m24: f32,
        m31: f32, m32: f32, m33: f32, m34: f32,
        m41: f32, m42: f32, m43: f32, m44: f32)
     -> Matrix4 {
    Matrix4 {
        m11: m11, m12: m12, m13: m13, m14: m14,
        m21: m21, m22: m22, m23: m23, m24: m24,
        m31: m31, m32: m32, m33: m33, m34: m34,
        m41: m41, m42: m42, m43: m43, m44: m44
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32,
}

impl Matrix4 {
    pub fn approx_eq(&self, other: &Matrix4) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m13.approx_eq(&other.m13) && self.m14.approx_eq(&other.m14) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m23.approx_eq(&other.m23) && self.m24.approx_eq(&other.m24) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32) &&
        self.m33.approx_eq(&other.m33) && self.m34.approx_eq(&other.m34) &&
        self.m41.approx_eq(&other.m41) && self.m42.approx_eq(&other.m42) &&
        self.m43.approx_eq(&other.m43) && self.m44.approx_eq(&other.m44)
    }

    pub fn mul(&self, m: &Matrix4) -> Matrix4 {
        Matrix4(m.m11*self.m11 + m.m12*self.m21 + m.m13*self.m31 + m.m14*self.m41,
                m.m11*self.m12 + m.m12*self.m22 + m.m13*self.m32 + m.m14*self.m42,
                m.m11*self.m13 + m.m12*self.m23 + m.m13*self.m33 + m.m14*self.m43,
                m.m11*self.m14 + m.m12*self.m24 + m.m13*self.m34 + m.m14*self.m44,
                m.m21*self.m11 + m.m22*self.m21 + m.m23*self.m31 + m.m24*self.m41,
                m.m21*self.m12 + m.m22*self.m22 + m.m23*self.m32 + m.m24*self.m42,
                m.m21*self.m13 + m.m22*self.m23 + m.m23*self.m33 + m.m24*self.m43,
                m.m21*self.m14 + m.m22*self.m24 + m.m23*self.m34 + m.m24*self.m44,
                m.m31*self.m11 + m.m32*self.m21 + m.m33*self.m31 + m.m34*self.m41,
                m.m31*self.m12 + m.m32*self.m22 + m.m33*self.m32 + m.m34*self.m42,
                m.m31*self.m13 + m.m32*self.m23 + m.m33*self.m33 + m.m34*self.m43,
                m.m31*self.m14 + m.m32*self.m24 + m.m33*self.m34 + m.m34*self.m44,
                m.m41*self.m11 + m.m42*self.m21 + m.m43*self.m31 + m.m44*self.m41,
                m.m41*self.m12 + m.m42*self.m22 + m.m43*self.m32 + m.m44*self.m42,
                m.m41*self.m13 + m.m42*self.m23 + m.m43*self.m33 + m.m44*self.m43,
                m.m41*self.m14 + m.m42*self.m24 + m.m43*self.m34 + m.m44*self.m44)
    }

    pub fn mul_s(&self, x: f32) -> Matrix4 {
        Matrix4(self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
                self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
                self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
                self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x)
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4(self.m11 * x, self.m12,     self.m13,     self.m14,
                self.m21    , self.m22 * y, self.m23,     self.m24,
                self.m31    , self.m32,     self.m33 * z, self.m34,
                self.m41    , self.m42,     self.m43,     self.m44)
    }

    /// Returns the given point transformed by this matrix.
    #[inline]
    pub fn transform_point(&self, p: &Point2D<f32>) -> Point2D<f32> {
        Point2D(p.x * self.m11 + p.y * self.m21 + self.m41,
                p.x * self.m12 + p.y * self.m22 + self.m42)
    }

    /// Transform the 3d point, and perform perspective division.
    #[inline]
    pub fn transform_homogenous(&self, p: &Point3D<f32>) -> Point3D<f32> {
        let x = p.x * self.m11 + p.y * self.m21 + p.z * self.m31 + self.m41;
        let y = p.x * self.m12 + p.y * self.m22 + p.z * self.m32 + self.m42;
        let z = p.x * self.m13 + p.y * self.m23 + p.z * self.m33 + self.m43;
        let w = p.x * self.m14 + p.y * self.m24 + p.z * self.m34 + self.m44;
        let inv_w = 1.0 / w;
        Point3D::new(x * inv_w, y * inv_w, z * inv_w)
    }

    pub fn to_array(&self) -> [f32; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Matrix4 {
        let matrix = Matrix4(1.0, 0.0, 0.0, 0.0,
                             0.0, 1.0, 0.0, 0.0,
                             0.0, 0.0, 1.0, 0.0,
                               x,   y,   z, 1.0);

        return self.mul(&matrix);
    }

    /// Create a 3d translation matrix
    pub fn create_translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4(1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                  x,   y,   z, 1.0)
    }

    /// Create a 3d scale matrix
    pub fn create_scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4(  x, 0.0, 0.0, 0.0,
                0.0,   y, 0.0, 0.0,
                0.0, 0.0,   z, 0.0,
                0.0, 0.0, 0.0, 1.0)
    }

    /// Create a 3d rotation matrix from an angle / axis.
    /// The supplied axis must be normalized.
    pub fn create_rotation(x: f32, y: f32, z: f32, theta: f32) -> Matrix4 {
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta * 0.5;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        Matrix4(
            1.0 - 2.0 * (yy + zz) * sq,
            2.0 * (x * y * sq - z * sc),
            2.0 * (x * z * sq + y * sc),
            0.0,

            2.0 * (x * y * sq + z * sc),
            1.0 - 2.0 * (xx + zz) * sq,
            2.0 * (y * z * sq - x * sc),
            0.0,

            2.0 * (x * z * sq - y * sc),
            2.0 * (y * z * sq + x * sc),
            1.0 - 2.0 * (xx + yy) * sq,
            0.0,

            0.0,
            0.0,
            0.0,
            1.0
        )
    }

    /// Create a 2d skew matrix
    pub fn create_skew(sx: f32, sy: f32) -> Matrix4 {
        Matrix4(1.0,  sx, 0.0, 0.0,
                 sy, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0)
    }

    /// Create a simple perspective projection matrix
    pub fn create_perspective(d: f32) -> Matrix4 {
        Matrix4(1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, -1.0 / d,
                0.0, 0.0, 0.0, 1.0)
    }
}

// TODO(gw): Move ortho and identity into static functions of the Matrix type.
pub fn ortho(left: f32, right: f32,
             bottom: f32, top: f32,
             near: f32, far: f32) -> Matrix4 {
    let tx = -((right + left) / (right - left));
    let ty = -((top + bottom) / (top - bottom));
    let tz = -((far + near) / (far - near));

    Matrix4(2.0 / (right - left),
            0.0,
            0.0,
            0.0,

            0.0,
            2.0 / (top - bottom),
            0.0,
            0.0,

            0.0,
            0.0,
            -2.0 / (far - near),
            0.0,

            tx,
            ty,
            tz,
            1.0)
}

pub fn identity() -> Matrix4 {
    Matrix4(1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
}

#[test]
pub fn test_ortho() {
    let (left, right, bottom, top) = (0.0f32, 1.0f32, 0.1f32, 1.0f32);
    let (near, far) = (-1.0f32, 1.0f32);
    let result = ortho(left, right, bottom, top, near, far);
    let expected = Matrix4(2.0,  0.0,         0.0,  0.0,
                           0.0,  2.22222222,  0.0,  0.0,
                           0.0,  0.0,         -1.0, 0.0,
                           -1.0, -1.22222222, -0.0, 1.0);
    debug!("result={:?} expected={:?}", result, expected);
    assert!(result.approx_eq(&expected));
}

