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
use point::{TypedPoint2D, TypedPoint4D};
use matrix2d::TypedMatrix2D;
use length::UnknownUnit;
use scale_factor::ScaleFactor;
use num::{One, Zero};
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::marker::PhantomData;
use std::fmt;

define_matrix! {
    pub struct TypedMatrix4D<T, Src, Dst> {
        pub m11: T, pub m12: T, pub m13: T, pub m14: T,
        pub m21: T, pub m22: T, pub m23: T, pub m24: T,
        pub m31: T, pub m32: T, pub m33: T, pub m34: T,
        pub m41: T, pub m42: T, pub m43: T, pub m44: T,
    }
}

pub type Matrix4D<T> = TypedMatrix4D<T, UnknownUnit, UnknownUnit>;

impl<T, Src, Dst> TypedMatrix4D<T, Src, Dst> {
    #[inline]
    pub fn new(
            m11: T, m12: T, m13: T, m14: T,
            m21: T, m22: T, m23: T, m24: T,
            m31: T, m32: T, m33: T, m34: T,
            m41: T, m42: T, m43: T, m44: T)
         -> TypedMatrix4D<T, Src, Dst> {
        TypedMatrix4D {
            m11: m11, m12: m12, m13: m13, m14: m14,
            m21: m21, m22: m22, m23: m23, m24: m24,
            m31: m31, m32: m32, m33: m33, m34: m34,
            m41: m41, m42: m42, m43: m43, m44: m44,
            _unit: PhantomData,
        }
    }
}

impl <T, Src, Dst> TypedMatrix4D<T, Src, Dst>
where T: Copy + Clone +
         Add<T, Output=T> +
         Sub<T, Output=T> +
         Mul<T, Output=T> +
         Div<T, Output=T> +
         Neg<Output=T> +
         ApproxEq<T> +
         PartialOrd +
         Trig +
         One + Zero {

    #[inline]
    pub fn new_2d(m11: T, m12: T, m21: T, m22: T, m41: T, m42: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        TypedMatrix4D::new(
            m11, m12, _0, _0,
            m21, m22, _0, _0,
             _0,  _0, _1, _0,
            m41, m42, _0, _1
       )
    }

    pub fn ortho(left: T, right: T,
                 bottom: T, top: T,
                 near: T, far: T) -> TypedMatrix4D<T, Src, Dst> {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;
        TypedMatrix4D::new(_2 / (right - left),
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

    #[inline]
    pub fn identity() -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        TypedMatrix4D::new(
            _1, _0, _0, _0,
            _0, _1, _0, _0,
            _0, _0, _1, _0,
            _0, _0, _0, _1
        )
    }


    // See https://drafts.csswg.org/css-transforms/#2d-matrix
    #[inline]
    pub fn is_2d(&self) -> bool {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        self.m31 == _0 && self.m32 == _0 &&
        self.m13 == _0 && self.m23 == _0 &&
        self.m43 == _0 && self.m14 == _0 &&
        self.m24 == _0 && self.m34 == _0 &&
        self.m33 == _1 && self.m44 == _1
    }

    pub fn to_2d(&self) -> TypedMatrix2D<T, Src, Dst> {
        TypedMatrix2D::new(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m41, self.m42
        )
    }

    pub fn approx_eq(&self, other: &TypedMatrix4D<T, Src, Dst>) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m13.approx_eq(&other.m13) && self.m14.approx_eq(&other.m14) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m23.approx_eq(&other.m23) && self.m24.approx_eq(&other.m24) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32) &&
        self.m33.approx_eq(&other.m33) && self.m34.approx_eq(&other.m34) &&
        self.m41.approx_eq(&other.m41) && self.m42.approx_eq(&other.m42) &&
        self.m43.approx_eq(&other.m43) && self.m44.approx_eq(&other.m44)
    }

    pub fn to<Destination>(&self) -> TypedMatrix4D<T, Src, Destination> {
        TypedMatrix4D::new(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }

    pub fn mul<NewSrc>(&self, mat: &TypedMatrix4D<T, NewSrc, Src>) -> TypedMatrix4D<T, NewSrc, Dst> {
        TypedMatrix4D::new(
            mat.m11*self.m11 + mat.m12*self.m21 + mat.m13*self.m31 + mat.m14*self.m41,
            mat.m11*self.m12 + mat.m12*self.m22 + mat.m13*self.m32 + mat.m14*self.m42,
            mat.m11*self.m13 + mat.m12*self.m23 + mat.m13*self.m33 + mat.m14*self.m43,
            mat.m11*self.m14 + mat.m12*self.m24 + mat.m13*self.m34 + mat.m14*self.m44,
            mat.m21*self.m11 + mat.m22*self.m21 + mat.m23*self.m31 + mat.m24*self.m41,
            mat.m21*self.m12 + mat.m22*self.m22 + mat.m23*self.m32 + mat.m24*self.m42,
            mat.m21*self.m13 + mat.m22*self.m23 + mat.m23*self.m33 + mat.m24*self.m43,
            mat.m21*self.m14 + mat.m22*self.m24 + mat.m23*self.m34 + mat.m24*self.m44,
            mat.m31*self.m11 + mat.m32*self.m21 + mat.m33*self.m31 + mat.m34*self.m41,
            mat.m31*self.m12 + mat.m32*self.m22 + mat.m33*self.m32 + mat.m34*self.m42,
            mat.m31*self.m13 + mat.m32*self.m23 + mat.m33*self.m33 + mat.m34*self.m43,
            mat.m31*self.m14 + mat.m32*self.m24 + mat.m33*self.m34 + mat.m34*self.m44,
            mat.m41*self.m11 + mat.m42*self.m21 + mat.m43*self.m31 + mat.m44*self.m41,
            mat.m41*self.m12 + mat.m42*self.m22 + mat.m43*self.m32 + mat.m44*self.m42,
            mat.m41*self.m13 + mat.m42*self.m23 + mat.m43*self.m33 + mat.m44*self.m43,
            mat.m41*self.m14 + mat.m42*self.m24 + mat.m43*self.m34 + mat.m44*self.m44
        )
    }

    pub fn invert(&self) -> TypedMatrix4D<T, Dst, Src> {
        let det = self.determinant();

        if det == Zero::zero() {
            return TypedMatrix4D::identity();
        }

        // todo(gw): this could be made faster by special casing
        // for simpler matrix types.
        let m = TypedMatrix4D::new(
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

    pub fn mul_s(&self, x: T) -> TypedMatrix4D<T, Src, Dst> {
        TypedMatrix4D::new(
            self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
            self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
            self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
            self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x
        )
    }

    pub fn from_scale_factor(scale: ScaleFactor<T, Src, Dst>) -> TypedMatrix4D<T, Src, Dst> {
        TypedMatrix4D::create_scale(scale.get(), scale.get(), scale.get())
    }

    pub fn scale(&self, x: T, y: T, z: T) -> TypedMatrix4D<T, Src, Dst> {
        TypedMatrix4D::new(
            self.m11 * x, self.m12,     self.m13,     self.m14,
            self.m21    , self.m22 * y, self.m23,     self.m24,
            self.m31    , self.m32,     self.m33 * z, self.m34,
            self.m41    , self.m42,     self.m43,     self.m44
        )
    }

    /// Returns the given point transformed by this matrix.
    #[inline]
    pub fn transform_point(&self, p: &TypedPoint2D<T, Src>) -> TypedPoint2D<T, Dst> {
        TypedPoint2D::new(p.x * self.m11 + p.y * self.m21 + self.m41,
                          p.x * self.m12 + p.y * self.m22 + self.m42)
    }

    #[inline]
    pub fn transform_point4d(&self, p: &TypedPoint4D<T, Src>) -> TypedPoint4D<T, Dst> {
        let x = p.x * self.m11 + p.y * self.m21 + p.z * self.m31 + self.m41;
        let y = p.x * self.m12 + p.y * self.m22 + p.z * self.m32 + self.m42;
        let z = p.x * self.m13 + p.y * self.m23 + p.z * self.m33 + self.m43;
        let w = p.x * self.m14 + p.y * self.m24 + p.z * self.m34 + self.m44;
        TypedPoint4D::new(x, y, z, w)
    }

    pub fn translate(&self, x: T, y: T, z: T) -> TypedMatrix4D<T, Src, Dst> {
        self.mul(&TypedMatrix4D::create_translation(x, y, z))
    }

    /// Create a 3d translation matrix
    pub fn create_translation(x: T, y: T, z: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        TypedMatrix4D::new(
            _1, _0, _0, _0,
            _0, _1, _0, _0,
            _0, _0, _1, _0,
             x,  y,  z, _1
        )
    }

    /// Create a 3d scale matrix
    pub fn create_scale(x: T, y: T, z: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        TypedMatrix4D::new(
             x, _0, _0, _0,
            _0,  y, _0, _0,
            _0, _0,  z, _0,
            _0, _0, _0, _1
        )
    }

    /// Create a 3d rotation matrix from an angle / axis.
    /// The supplied axis must be normalized.
    pub fn create_rotation(x: T, y: T, z: T, theta: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta / _2;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        TypedMatrix4D::new(
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
    pub fn create_skew(alpha: T, beta: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let (sx, sy) = (beta.tan(), alpha.tan());
        TypedMatrix4D::new(_1, sx, _0, _0,
                      sy, _1, _0, _0,
                      _0, _0, _1, _0,
                      _0, _0, _0, _1)
    }

    /// Create a simple perspective projection matrix
    pub fn create_perspective(d: T) -> TypedMatrix4D<T, Src, Dst> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        TypedMatrix4D::new(_1, _0, _0, _0,
                      _0, _1, _0, _0,
                      _0, _0, _1, -_1 / d,
                      _0, _0, _0, _1)
    }
}

impl<T: Copy, Src, Dst> TypedMatrix4D<T, Src, Dst> {
    pub fn to_array(&self) -> [T; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]
    }
}

impl<T: Copy + fmt::Debug, Src, Dst> fmt::Debug for TypedMatrix4D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_array().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use point::Point2D;
    use super::*;

    type Mf32 = Matrix4D<f32>;

    #[test]
    pub fn test_ortho() {
        let (left, right, bottom, top) = (0.0f32, 1.0f32, 0.1f32, 1.0f32);
        let (near, far) = (-1.0f32, 1.0f32);
        let result = Mf32::ortho(left, right, bottom, top, near, far);
        let expected = Mf32::new(2.0,  0.0,         0.0,  0.0,
                                 0.0,  2.22222222,  0.0,  0.0,
                                 0.0,  0.0,         -1.0, 0.0,
                                 -1.0, -1.22222222, -0.0, 1.0);
        debug!("result={:?} expected={:?}", result, expected);
        assert!(result.approx_eq(&expected));
    }

    #[test]
    pub fn test_is_2d() {
        assert!(Mf32::identity().is_2d());
        assert!(Mf32::create_rotation(0.0, 0.0, 1.0, 0.7854).is_2d());
        assert!(!Mf32::create_rotation(0.0, 1.0, 0.0, 0.7854).is_2d());
    }

    #[test]
    pub fn test_new_2d() {
        let m1 = Mf32::new_2d(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let m2 = Mf32::new(1.0, 2.0, 0.0, 0.0,
                           3.0, 4.0, 0.0, 0.0,
                           0.0, 0.0, 1.0, 0.0,
                           5.0, 6.0, 0.0, 1.0);
        assert_eq!(m1, m2);
    }

    #[test]
    pub fn test_invert_simple() {
        let m1 = Mf32::identity();
        let m2 = m1.invert();
        assert!(m1.approx_eq(&m2));
    }

    #[test]
    pub fn test_invert_scale() {
        let m1 = Mf32::create_scale(1.5, 0.3, 2.1);
        let m2 = m1.invert();
        assert!(m1.mul(&m2).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_invert_translate() {
        let m1 = Mf32::create_translation(-132.0, 0.3, 493.0);
        let m2 = m1.invert();
        assert!(m1.mul(&m2).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_invert_rotate() {
        let m1 = Mf32::create_rotation(0.0, 1.0, 0.0, 1.57);
        let m2 = m1.invert();
        assert!(m1.mul(&m2).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_invert_transform_point_2d() {
        let m1 = Mf32::create_translation(100.0, 200.0, 0.0);
        let m2 = m1.invert();
        assert!(m1.mul(&m2).approx_eq(&Mf32::identity()));

        let p1 = Point2D::new(1000.0, 2000.0);
        let p2 = m1.transform_point(&p1);
        assert!(p2.eq(&Point2D::new(1100.0, 2200.0)));

        let p3 = m2.transform_point(&p2);
        assert!(p3.eq(&p1));
    }
}
