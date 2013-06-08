// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::num::{One, Zero};

pub fn Matrix2D<T:Add<T,T> + Copy + Mul<T,T> + One + Zero>(
        m11: T, m12: T,
        m21: T, m22: T,
        m31: T, m32: T)
    -> Matrix2D<T> {
    Matrix2D {
        m11: m11, m12: m12,
        m21: m21, m22: m22,
        m31: m31, m32: m32
    }
}

pub struct Matrix2D<T> {
    m11: T, m12: T,
    m21: T, m22: T,
    m31: T, m32: T
}

pub impl<T:Add<T,T> + Copy + Mul<T,T> + One + Zero> Matrix2D<T> {
    fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Matrix2D<T> {
        Matrix2D {
            m11: m11, m12: m12,
            m21: m21, m22: m22,
            m31: m31, m32: m32
        }
    }

    fn mul(&self, m: &Matrix2D<T>) -> Matrix2D<T> {
        Matrix2D(m.m11*self.m11 + m.m12*self.m21,
                 m.m11*self.m12 + m.m12*self.m22,
                 m.m21*self.m11 + m.m22*self.m21,
                 m.m21*self.m12 + m.m22*self.m22,
                 m.m31*self.m11 + m.m32*self.m21 + self.m31,
                 m.m31*self.m12 + m.m32*self.m22 + self.m32)
    }
                 

    fn translate(&self, x: T, y: T) -> Matrix2D<T> {
        let (_0, _1) = (Zero::zero(), One::one());
        let matrix = Matrix2D::new(_1, _0,
                                   _0, _1,
                                   x, y);
        return self.mul(&matrix);
    }


    fn scale(&self, x: T, y: T) -> Matrix2D<T> {
        Matrix2D(self.m11 * x, self.m12,
                 self.m21,     self.m22 * y,
                 self.m31,     self.m32)
    }

    fn identity() -> Matrix2D<T> {
        let (_0, _1) = (Zero::zero(), One::one());
        return Matrix2D::new(_1, _0,
                             _0, _1,
                             _0, _0);
    }

    fn to_array(&self) -> [T, ..6] {
        [
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32
        ]
    }
}

