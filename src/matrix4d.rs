// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use approxeq::ApproxEq;
use trig::Trig;
use point::{Point2D, Point4D};
use num::{One, Zero};
use std::ops::{Add, Mul, Sub, Div, Neg};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf, Deserialize, Serialize))]
pub struct Matrix4D<T> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
}

impl <T:Add<T, Output=T> +
       ApproxEq<T> +
       Copy +
       Clone +
       Div<T, Output=T> +
       Mul<T, Output=T> +
       Neg<Output=T> +
       One +
       PartialOrd +
       Sub<T, Output=T> +
       Trig +
       Zero> Matrix4D<T> {
    pub fn new(
            m11: T, m12: T, m13: T, m14: T,
            m21: T, m22: T, m23: T, m24: T,
            m31: T, m32: T, m33: T, m34: T,
            m41: T, m42: T, m43: T, m44: T)
         -> Matrix4D<T> {
        Matrix4D {
            m11: m11, m12: m12, m13: m13, m14: m14,
            m21: m21, m22: m22, m23: m23, m24: m24,
            m31: m31, m32: m32, m33: m33, m34: m34,
            m41: m41, m42: m42, m43: m43, m44: m44
        }
    }

    pub fn ortho(left: T, right: T,
                 bottom: T, top: T,
                 near: T, far: T) -> Matrix4D<T> {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;
        Matrix4D::new(_2 / (right - left),
                      _0,
                      _0,
                      _0,

                      _0,
                      _2 / (top - bottom),
                      _0,
                      _0,

                      _0,
                      _0,
                      -_2 / (far - near),
                      _0,

                       tx,
                       ty,
                       tz,
                      _1)
    }

    pub fn identity() -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4D::new(_1, _0, _0, _0,
                      _0, _1, _0, _0,
                      _0, _0, _1, _0,
                      _0, _0, _0, _1)
    }

    pub fn approx_eq(&self, other: &Matrix4D<T>) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m13.approx_eq(&other.m13) && self.m14.approx_eq(&other.m14) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m23.approx_eq(&other.m23) && self.m24.approx_eq(&other.m24) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32) &&
        self.m33.approx_eq(&other.m33) && self.m34.approx_eq(&other.m34) &&
        self.m41.approx_eq(&other.m41) && self.m42.approx_eq(&other.m42) &&
        self.m43.approx_eq(&other.m43) && self.m44.approx_eq(&other.m44)
    }

    pub fn mul(&self, m: &Matrix4D<T>) -> Matrix4D<T> {
        Matrix4D::new(m.m11*self.m11 + m.m12*self.m21 + m.m13*self.m31 + m.m14*self.m41,
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

    pub fn invert(&self) -> Matrix4D<T> {
        let det = self.determinant();

        if det == Zero::zero() {
            return Matrix4D::identity();
        }

        // todo(gw): this could be made faster by special casing
        // for simpler matrix types.
        let m = Matrix4D::new(
            self.m23*self.m34*self.m42 - self.m24*self.m33*self.m42 +
            self.m24*self.m32*self.m43 - self.m22*self.m34*self.m43 -
            self.m23*self.m32*self.m44 + self.m22*self.m33*self.m44,

            self.m14*self.m33*self.m42 - self.m13*self.m34*self.m42 -
            self.m14*self.m32*self.m43 + self.m12*self.m34*self.m43 +
            self.m13*self.m32*self.m44 - self.m12*self.m33*self.m44,

            self.m13*self.m24*self.m42 - self.m14*self.m23*self.m42 +
            self.m14*self.m22*self.m43 - self.m12*self.m24*self.m43 -
            self.m13*self.m22*self.m44 + self.m12*self.m23*self.m44,

            self.m14*self.m23*self.m32 - self.m13*self.m24*self.m32 -
            self.m14*self.m22*self.m33 + self.m12*self.m24*self.m33 +
            self.m13*self.m22*self.m34 - self.m12*self.m23*self.m34,

            self.m24*self.m33*self.m41 - self.m23*self.m34*self.m41 -
            self.m24*self.m31*self.m43 + self.m21*self.m34*self.m43 +
            self.m23*self.m31*self.m44 - self.m21*self.m33*self.m44,

            self.m13*self.m34*self.m41 - self.m14*self.m33*self.m41 +
            self.m14*self.m31*self.m43 - self.m11*self.m34*self.m43 -
            self.m13*self.m31*self.m44 + self.m11*self.m33*self.m44,

            self.m14*self.m23*self.m41 - self.m13*self.m24*self.m41 -
            self.m14*self.m21*self.m43 + self.m11*self.m24*self.m43 +
            self.m13*self.m21*self.m44 - self.m11*self.m23*self.m44,

            self.m13*self.m24*self.m31 - self.m14*self.m23*self.m31 +
            self.m14*self.m21*self.m33 - self.m11*self.m24*self.m33 -
            self.m13*self.m21*self.m34 + self.m11*self.m23*self.m34,

            self.m22*self.m34*self.m41 - self.m24*self.m32*self.m41 +
            self.m24*self.m31*self.m42 - self.m21*self.m34*self.m42 -
            self.m22*self.m31*self.m44 + self.m21*self.m32*self.m44,

            self.m14*self.m32*self.m41 - self.m12*self.m34*self.m41 -
            self.m14*self.m31*self.m42 + self.m11*self.m34*self.m42 +
            self.m12*self.m31*self.m44 - self.m11*self.m32*self.m44,

            self.m12*self.m24*self.m41 - self.m14*self.m22*self.m41 +
            self.m14*self.m21*self.m42 - self.m11*self.m24*self.m42 -
            self.m12*self.m21*self.m44 + self.m11*self.m22*self.m44,

            self.m14*self.m22*self.m31 - self.m12*self.m24*self.m31 -
            self.m14*self.m21*self.m32 + self.m11*self.m24*self.m32 +
            self.m12*self.m21*self.m34 - self.m11*self.m22*self.m34,

            self.m23*self.m32*self.m41 - self.m22*self.m33*self.m41 -
            self.m23*self.m31*self.m42 + self.m21*self.m33*self.m42 +
            self.m22*self.m31*self.m43 - self.m21*self.m32*self.m43,

            self.m12*self.m33*self.m41 - self.m13*self.m32*self.m41 +
            self.m13*self.m31*self.m42 - self.m11*self.m33*self.m42 -
            self.m12*self.m31*self.m43 + self.m11*self.m32*self.m43,

            self.m13*self.m22*self.m41 - self.m12*self.m23*self.m41 -
            self.m13*self.m21*self.m42 + self.m11*self.m23*self.m42 +
            self.m12*self.m21*self.m43 - self.m11*self.m22*self.m43,

            self.m12*self.m23*self.m31 - self.m13*self.m22*self.m31 +
            self.m13*self.m21*self.m32 - self.m11*self.m23*self.m32 -
            self.m12*self.m21*self.m33 + self.m11*self.m22*self.m33
        );

        let _1: T = One::one();
        m.mul_s(_1 / det)
    }

    pub fn determinant(&self) -> T {
        self.m14 * self.m23 * self.m32 * self.m41 -
        self.m13 * self.m24 * self.m32 * self.m41 -
        self.m14 * self.m22 * self.m33 * self.m41 +
        self.m12 * self.m24 * self.m33 * self.m41 +
        self.m13 * self.m22 * self.m34 * self.m41 -
        self.m12 * self.m23 * self.m34 * self.m41 -
        self.m14 * self.m23 * self.m31 * self.m42 +
        self.m13 * self.m24 * self.m31 * self.m42 +
        self.m14 * self.m21 * self.m33 * self.m42 -
        self.m11 * self.m24 * self.m33 * self.m42 -
        self.m13 * self.m21 * self.m34 * self.m42 +
        self.m11 * self.m23 * self.m34 * self.m42 +
        self.m14 * self.m22 * self.m31 * self.m43 -
        self.m12 * self.m24 * self.m31 * self.m43 -
        self.m14 * self.m21 * self.m32 * self.m43 +
        self.m11 * self.m24 * self.m32 * self.m43 +
        self.m12 * self.m21 * self.m34 * self.m43 -
        self.m11 * self.m22 * self.m34 * self.m43 -
        self.m13 * self.m22 * self.m31 * self.m44 +
        self.m12 * self.m23 * self.m31 * self.m44 +
        self.m13 * self.m21 * self.m32 * self.m44 -
        self.m11 * self.m23 * self.m32 * self.m44 -
        self.m12 * self.m21 * self.m33 * self.m44 +
        self.m11 * self.m22 * self.m33 * self.m44
    }

    pub fn mul_s(&self, x: T) -> Matrix4D<T> {
        Matrix4D::new(self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
                     self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
                     self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
                     self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x)
    }

    pub fn scale(&self, x: T, y: T, z: T) -> Matrix4D<T> {
        Matrix4D::new(self.m11 * x, self.m12,     self.m13,     self.m14,
                     self.m21    , self.m22 * y, self.m23,     self.m24,
                     self.m31    , self.m32,     self.m33 * z, self.m34,
                     self.m41    , self.m42,     self.m43,     self.m44)
    }

    /// Returns the given point transformed by this matrix.
    #[inline]
    pub fn transform_point(&self, p: &Point2D<T>) -> Point2D<T> {
        Point2D::new(p.x * self.m11 + p.y * self.m21 + self.m41,
                     p.x * self.m12 + p.y * self.m22 + self.m42)
    }

    #[inline]
    pub fn transform_point4d(&self, p: &Point4D<T>) -> Point4D<T> {
        let x = p.x * self.m11 + p.y * self.m21 + p.z * self.m31 + self.m41;
        let y = p.x * self.m12 + p.y * self.m22 + p.z * self.m32 + self.m42;
        let z = p.x * self.m13 + p.y * self.m23 + p.z * self.m33 + self.m43;
        let w = p.x * self.m14 + p.y * self.m24 + p.z * self.m34 + self.m44;
        Point4D::new(x, y, z, w)
    }

    pub fn to_array(&self) -> [T; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]
    }

    pub fn translate(&self, x: T, y: T, z: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let matrix = Matrix4D::new(_1, _0, _0, _0,
                                   _0, _1, _0, _0,
                                   _0, _0, _1, _0,
                                    x,  y,  z, _1);
        self.mul(&matrix)
    }

    /// Create a 3d translation matrix
    pub fn create_translation(x: T, y: T, z: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4D::new(_1, _0, _0, _0,
                      _0, _1, _0, _0,
                      _0, _0, _1, _0,
                       x,  y,  z, _1)
    }

    /// Create a 3d scale matrix
    pub fn create_scale(x: T, y: T, z: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4D::new( x, _0, _0, _0,
                      _0,  y, _0, _0,
                      _0, _0,  z, _0,
                      _0, _0, _0, _1)
    }

    /// Create a 3d rotation matrix from an angle / axis.
    /// The supplied axis must be normalized.
    pub fn create_rotation(x: T, y: T, z: T, theta: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta / _2;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        Matrix4D::new(
            _1 - _2 * (yy + zz) * sq,
            _2 * (x * y * sq - z * sc),
            _2 * (x * z * sq + y * sc),
            _0,

            _2 * (x * y * sq + z * sc),
            _1 - _2 * (xx + zz) * sq,
            _2 * (y * z * sq - x * sc),
            _0,

            _2 * (x * z * sq - y * sc),
            _2 * (y * z * sq + x * sc),
            _1 - _2 * (xx + yy) * sq,
            _0,

            _0,
            _0,
            _0,
            _1
        )
    }

    /// Create a 2d skew matrix.
    /// https://drafts.csswg.org/css-transforms/#funcdef-skew
    pub fn create_skew(alpha: T, beta: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let (sx, sy) = (beta.tan(), alpha.tan());
        Matrix4D::new(_1, sx, _0, _0,
                      sy, _1, _0, _0,
                      _0, _0, _1, _0,
                      _0, _0, _0, _1)
    }

    /// Create a simple perspective projection matrix
    pub fn create_perspective(d: T) -> Matrix4D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4D::new(_1, _0, _0, _0,
                      _0, _1, _0, _0,
                      _0, _0, _1, -_1 / d,
                      _0, _0, _0, _1)
    }
}
