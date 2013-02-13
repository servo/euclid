use std::cmp::FuzzyEq;
use num::{One, Zero};

pub struct Matrix2D<T> {
    m11: T, m12: T,
    m21: T, m22: T,
    m31: T, m32: T
}

pub impl<T:Copy + One + Zero> Matrix2D<T> {
    static pure fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Matrix2D<T> {
        Matrix2D {
            m11: m11, m12: m12,
            m21: m21, m22: m22,
            m31: m31, m32: m32
        }
    }

    pure fn translate(&self, x: &T, y: &T) -> Matrix2D<T> {
        let (_0, _1) = (Zero::zero(), One::one());
        let matrix = Matrix2D::new(_1, _0,
                                   _0, _1,
                                   *x, *y);
        return matrix;  // FIXME: This does not multiply yet!!
    }

    static fn identity() -> Matrix2D<T> {
        let (_0, _1) = (Zero::zero(), One::one());
        return Matrix2D::new(_1, _0,
                             _0, _1,
                             _0, _0);
    }
}

