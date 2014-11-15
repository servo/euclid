// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use point::Point2D;
use rect::Rect;
use size::Size2D;

use std::num::{One, Zero};

pub struct Matrix2D<T> {
    m11: T, m12: T,
    m21: T, m22: T,
    m31: T, m32: T
}

impl<T> Matrix2D<T> where T: Add<T,T> + Clone + Mul<T,T> + One + PartialOrd + Sub<T,T> + Zero {
    pub fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Matrix2D<T> {
        Matrix2D {
            m11: m11, m12: m12,
            m21: m21, m22: m22,
            m31: m31, m32: m32
        }
    }

    pub fn mul(&self, m: &Matrix2D<T>) -> Matrix2D<T> {
        Matrix2D::new(m.m11*self.m11 + m.m12*self.m21,
                      m.m11*self.m12 + m.m12*self.m22,
                      m.m21*self.m11 + m.m22*self.m21,
                      m.m21*self.m12 + m.m22*self.m22,
                      m.m31*self.m11 + m.m32*self.m21 + self.m31,
                      m.m31*self.m12 + m.m32*self.m22 + self.m32)
    } 

    pub fn translate(&self, x: T, y: T) -> Matrix2D<T> {
         let (_0, _1): (T, T) = (Zero::zero(), One::one());
         let matrix = Matrix2D::new(_1.clone(), _0.clone(),
                                    _0.clone(), _1.clone(),
                                    x, y);
        return self.mul(&matrix);
    }

    pub fn scale(&self, x: T, y: T) -> Matrix2D<T> {
        Matrix2D::new(self.m11 * x,     self.m12.clone(),
                      self.m21.clone(), self.m22 * y,
                      self.m31.clone(), self.m32.clone())
    }

    pub fn identity() -> Matrix2D<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        return Matrix2D::new(_1.clone(), _0.clone(),
                             _0.clone(), _1.clone(),
                             _0.clone(), _0.clone());
    }

    pub fn to_array(&self) -> [T, ..6] {
        [
            self.m11.clone(), self.m12.clone(),
            self.m21.clone(), self.m22.clone(),
            self.m31.clone(), self.m32.clone()
        ]
    }

    #[inline]
    pub fn transform_point(&self, point: &Point2D<T>) -> Point2D<T> {
        Point2D(self.m11 * point.x + self.m21 * point.y + self.m31,
                self.m21 * point.x + self.m22 * point.y + self.m32)
    }

    #[inline]
    pub fn transform_rect(&self, rect: &Rect<T>) -> Rect<T> {
        let upper_left = self.transform_point(&rect.origin);
        let lower_right = self.transform_point(&rect.max_point());
        Rect(upper_left.clone(),
             Size2D(lower_right.x - upper_left.x, lower_right.y - upper_left.y))
    }
}

