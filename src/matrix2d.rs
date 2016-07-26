// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::UnknownUnit;
use num::{One, Zero};
use point::TypedPoint2D;
use rect::TypedRect;
use size::TypedSize2D;
use std::ops::{Add, Mul, Div, Sub};
use std::marker::PhantomData;
use approxeq::ApproxEq;

define_matrix! {
    /// A 2d transform stored as a 2 by 3 matrix in row-major order in memory,
    /// useful to represent 2d transformations.
    ///
    /// Matrices can be parametrized over the source and destination units, to describe a
    /// transformation from a space to another.
    /// For example, TypedMatrix2D<f32, WordSpace, ScreenSpace>::transform_point4d
    /// takes a TypedPoint2D<f32, WordSpace> and returns a TypedPoint2D<f32, ScreenSpace>.
    ///
    /// Matrices expose a set of convenience methods for pre- and post-transformations.
    /// A pre-transformation corresponds to adding an operation that is applied before
    /// the rest of the transformation, while a post-transformation adds an operation
    /// that is appled after.
    pub struct TypedMatrix2D<T, Src, Dst> {
        pub m11: T, pub m12: T,
        pub m21: T, pub m22: T,
        pub m31: T, pub m32: T,
    }
}

/// The default 2d matrix type with no units.
pub type Matrix2D<T> = TypedMatrix2D<T, UnknownUnit, UnknownUnit>;

impl<T, Src, Dst> TypedMatrix2D<T, Src, Dst> {
    /// Create a matrix specifying its components in row-major order.
    pub fn row_major(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> TypedMatrix2D<T, Src, Dst> {
        TypedMatrix2D {
            m11: m11, m12: m12,
            m21: m21, m22: m22,
            m31: m31, m32: m32,
            _unit: PhantomData,
        }
    }

    /// Create a matrix specifying its components in column-major order.
    pub fn colum_major(m11: T, m21: T, m31: T, m12: T, m22: T, m32: T) -> TypedMatrix2D<T, Src, Dst> {
        TypedMatrix2D {
            m11: m11, m12: m12,
            m21: m21, m22: m22,
            m31: m31, m32: m32,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, Src, Dst> TypedMatrix2D<T, Src, Dst> {
    /// Returns an array containing this matrix's terms in row-major order (the order
    /// in which the matrix is actually laid out in memory).
    pub fn to_row_major_array(&self) -> [T; 6] {
        [
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32
        ]
    }

    /// Returns an array containing this matrix's terms in column-major order.
    pub fn to_column_major_array(&self) -> [T; 6] {
        [
            self.m11, self.m21, self.m31,
            self.m12, self.m22, self.m32
        ]
    }
}

impl<T, Src, Dst> TypedMatrix2D<T, Src, Dst>
where T: Copy + Clone +
         Add<T, Output=T> +
         Mul<T, Output=T> +
         Div<T, Output=T> +
         Sub<T, Output=T> +
         PartialOrd +
         One + Zero  {

    pub fn identity() -> TypedMatrix2D<T, Src, Dst> {
        let (_0, _1) = (Zero::zero(), One::one());
        TypedMatrix2D::row_major(
           _1, _0,
           _0, _1,
           _0, _0
        )
    }

    /// Returns the multiplication of the two matrices such that mat's transformation
    /// applies after self's transformation.
    pub fn post_mul<NewDst>(&self, mat: &TypedMatrix2D<T, Dst, NewDst>) -> TypedMatrix2D<T, Src, NewDst> {
        TypedMatrix2D::row_major(
            self.m11 * mat.m11 + self.m12 * mat.m21,
            self.m11 * mat.m12 + self.m12 * mat.m22,
            self.m21 * mat.m11 + self.m22 * mat.m21,
            self.m21 * mat.m12 + self.m22 * mat.m22,
            self.m31 * mat.m11 + self.m32 * mat.m21 + mat.m31,
            self.m31 * mat.m12 + self.m32 * mat.m22 + mat.m32,
        )
    }

    /// Returns the multiplication of the two matrices such that mat's transformation
    /// applies before self's transformation.
    pub fn pre_mul<NewSrc>(&self, mat: &TypedMatrix2D<T, NewSrc, Src>) -> TypedMatrix2D<T, NewSrc, Dst> {
        mat.post_mul(self)
    }

    pub fn create_translation(x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
         let (_0, _1): (T, T) = (Zero::zero(), One::one());
         TypedMatrix2D::row_major(
            _1, _0,
            _0, _1,
             x,  y
        )
    }

    /// Applies a translation after self's transformation and returns the resulting matrix.
    pub fn post_translated(&self, x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
        self.post_mul(&TypedMatrix2D::create_translation(x, y))
    }

    /// Applies a translation before self's transformation and returns the resulting matrix.
    pub fn pre_translated(&self, x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
        self.pre_mul(&TypedMatrix2D::create_translation(x, y))
    }

    pub fn create_scale(x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
        let _0 = Zero::zero();
        TypedMatrix2D::row_major(
             x, _0,
            _0,  y,
            _0, _0
        )
    }

    /// Applies a scale after self's transformation and returns the resulting matrix.
    pub fn post_scaled(&self, x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
        self.post_mul(&TypedMatrix2D::create_scale(x, y))
    }

    /// Applies a scale before self's transformation and returns the resulting matrix.
    pub fn pre_scaled(&self, x: T, y: T) -> TypedMatrix2D<T, Src, Dst> {
        TypedMatrix2D::row_major(
            self.m11 * x, self.m12,
            self.m21,     self.m22 * y,
            self.m31,     self.m32
        )
    }

    /// Returns the given point transformed by this matrix.
    #[inline]
    pub fn transform_point(&self, point: &TypedPoint2D<T, Src>) -> TypedPoint2D<T, Dst> {
        TypedPoint2D::new(point.x * self.m11 + point.y * self.m21 + self.m31,
                          point.x * self.m12 + point.y * self.m22 + self.m32)
    }

    /// Returns a rectangle that encompasses the result of transforming the given rectangle by this
    /// matrix.
    #[inline]
    pub fn transform_rect(&self, rect: &TypedRect<T, Src>) -> TypedRect<T, Dst> {
        let top_left = self.transform_point(&rect.origin);
        let top_right = self.transform_point(&rect.top_right());
        let bottom_left = self.transform_point(&rect.bottom_left());
        let bottom_right = self.transform_point(&rect.bottom_right());
        let (mut min_x, mut min_y) = (top_left.x, top_left.y);
        let (mut max_x, mut max_y) = (min_x, min_y);
        for point in &[top_right, bottom_left, bottom_right] {
            if point.x < min_x {
                min_x = point.x
            }
            if point.x > max_x {
                max_x = point.x
            }
            if point.y < min_y {
                min_y = point.y
            }
            if point.y > max_y {
                max_y = point.y
            }
        }
        TypedRect::new(TypedPoint2D::new(min_x, min_y),
                       TypedSize2D::new(max_x - min_x, max_y - min_y))
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    /// Returns the inverse matrix if possible.
    pub fn inverse(&self) -> Option<TypedMatrix2D<T, Dst, Src>> {
        let det = self.determinant();

        let _0: T = Zero::zero();
        let _1: T = One::one();

        if det == _0 {
          return None;
        }

        let inv_det = _1 / det;
        Some(TypedMatrix2D::row_major(
            inv_det * self.m22,
            inv_det * (_0 - self.m12),
            inv_det * (_0 - self.m21),
            inv_det * self.m11,
            inv_det * (self.m21 * self.m32 - self.m22 * self.m31),
            inv_det * (self.m31 * self.m12 - self.m11 * self.m32),
        ))
    }

    /// Returns the same matrix with a different destination unit.
    #[inline]
    pub fn with_destination<NewDst>(&self) -> TypedMatrix2D<T, Src, NewDst> {
        TypedMatrix2D::row_major(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }

    /// Returns the same matrix with a different source unit.
    #[inline]
    pub fn with_source<NewSrc>(&self) -> TypedMatrix2D<T, NewSrc, Dst> {
        TypedMatrix2D::row_major(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }
}

impl<T: ApproxEq<T>, Src, Dst> TypedMatrix2D<T, Src, Dst> {
    pub fn approx_eq(&self, other: &Self) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32)
    }
}

// Convenient aliases for TypedPoint2D with typed units
impl<T: Copy, Src, Dst> TypedMatrix2D<T, Src, Dst> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Matrix2D<T> {
        Matrix2D::row_major(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32
        )
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Matrix2D<T>) -> TypedMatrix2D<T, Src, Dst> {
        TypedMatrix2D::row_major(
            p.m11, p.m12,
            p.m21, p.m22,
            p.m31, p.m32
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Mat = Matrix2D<f32>;

    #[test]
    pub fn test_inverse_simple() {
        let m1 = Mat::identity();
        let m2 = m1.inverse().unwrap();
        assert!(m1.approx_eq(&m2));
    }

    #[test]
    pub fn test_inverse_scale() {
        let m1 = Mat::create_scale(1.5, 0.3);
        let m2 = m1.inverse().unwrap();
        assert!(m1.pre_mul(&m2).approx_eq(&Mat::identity()));
    }

    #[test]
    pub fn test_inverse_translate() {
        let m1 = Mat::create_translation(-132.0, 0.3);
        let m2 = m1.inverse().unwrap();
        assert!(m1.pre_mul(&m2).approx_eq(&Mat::identity()));
    }

    #[test]
    pub fn test_pre_post() {
        let m1 = Matrix2D::identity().post_scaled(1.0, 2.0).post_translated(1.0, 2.0);
        let m2 = Matrix2D::identity().pre_translated(1.0, 2.0).pre_scaled(1.0, 2.0);
        assert!(m1.approx_eq(&m2));
    }
}