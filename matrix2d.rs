use std::cmp::FuzzyEq;
use num::Num;

pub struct Matrix2D<T> {
    m11: T, m12: T,
    m21: T, m22: T,
    m31: T, m32: T
}

pub impl<T:Copy + FuzzyEq<Matrix2D<T>> + Num> Matrix2D<T> {
    static pure fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Matrix2D<T> {
        Matrix2D {
            m11: move m11, m12: move m12,
            m21: move m21, m22: move m22,
            m31: move m31, m32: move m32
        }
    }

    pure fn translate(&self, x: &T, y: &T) -> Matrix2D<T> {
        let _0 = Num::from_int(0);
        let _1 = Num::from_int(1);
        let matrix = Matrix2D::new(_1, _0,
                                   _0, _1,
                                   *x, *y);
        return matrix;  // FIXME: This does not multiply yet!!
    }

    static fn identity() -> Matrix2D<T> {
        let _0 = Num::from_int(0);
        let _1 = Num::from_int(1);
        return Matrix2D::new(_1, _0,
                             _0, _1,
                             _0, _0);
    }
}

