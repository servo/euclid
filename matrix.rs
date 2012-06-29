import float::num;
import std::cmp::fuzzy_eq;
import Num = num::num;

class Matrix4<T:copy fuzzy_eq Num> {
    let m11: T; let m12: T; let m13: T; let m14: T;
    let m21: T; let m22: T; let m23: T; let m24: T;
    let m31: T; let m32: T; let m33: T; let m34: T;
    let m41: T; let m42: T; let m43: T; let m44: T;

    new(m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T) {

        self.m11 = m11; self.m12 = m12; self.m13 = m13; self.m14 = m14;
        self.m21 = m21; self.m22 = m22; self.m23 = m23; self.m24 = m24;
        self.m31 = m31; self.m32 = m32; self.m33 = m33; self.m34 = m34;
        self.m41 = m41; self.m42 = m42; self.m43 = m43; self.m44 = m44;
    }

    fn fuzzy_eq(&&other: Matrix4<T>) -> bool {
        self.m11.fuzzy_eq(other.m11) && self.m12.fuzzy_eq(other.m12) &&
        self.m13.fuzzy_eq(other.m13) && self.m14.fuzzy_eq(other.m14) &&
        self.m21.fuzzy_eq(other.m21) && self.m22.fuzzy_eq(other.m22) &&
        self.m23.fuzzy_eq(other.m23) && self.m24.fuzzy_eq(other.m24) &&
        self.m31.fuzzy_eq(other.m31) && self.m32.fuzzy_eq(other.m32) &&
        self.m33.fuzzy_eq(other.m33) && self.m34.fuzzy_eq(other.m34) &&
        self.m41.fuzzy_eq(other.m41) && self.m42.fuzzy_eq(other.m42) &&
        self.m43.fuzzy_eq(other.m43) && self.m44.fuzzy_eq(other.m44)
    }

    fn mul_s(&&x: T) -> Matrix4<T> {
        ret Matrix4(self.m11.mul(x), self.m12.mul(x), self.m13.mul(x), self.m14.mul(x),
                    self.m21.mul(x), self.m22.mul(x), self.m23.mul(x), self.m24.mul(x),
                    self.m31.mul(x), self.m32.mul(x), self.m33.mul(x), self.m34.mul(x),
                    self.m41.mul(x), self.m42.mul(x), self.m43.mul(x), self.m44.mul(x));
    }

    pure fn to_array() -> [T]/~ {
        ret [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]/~;
    }
}

fn ortho<T:copy fuzzy_eq Num>(left: T, right: T, bottom: T, top: T, near: T, far: T)
                           -> Matrix4<T> {

    let two = left.from_int(2);
    let one = left.from_int(1);
    let zero = left.from_int(0);
    let minus_two = left.from_int(-2);

    let tx = right.add(left).div(right.sub(left)).neg();
    let ty = top.add(bottom).div(top.sub(bottom)).neg();
    let tz = far.add(near).div(far.sub(near)).neg();

    ret Matrix4(two.div(right.sub(left)), zero, zero, zero,
                zero, two.div(top.sub(bottom)), zero, zero,
                zero, zero, minus_two.div(far.sub(near)), zero,
                tx, ty, tz, one);
}

fn identity<T:copy fuzzy_eq Num>(_0: T) -> Matrix4<T> {
    let _1 = _0.from_int(1);
    ret Matrix4(_1, _0, _0, _0,
                _0, _1, _0, _0,
                _0, _0, _1, _0,
                _0, _0, _0, _1);
}

#[test]
fn test_ortho() {
    let (left, right, bottom, top) = (0.0, 1.0, 0.1, 1.0);
    let (near, far) = (-1.0, 1.0);
    let result = ortho(left, right, bottom, top, near, far);
    let expected = Matrix4(2.0,  0.0,         0.0,  0.0,
                           0.0,  2.22222222,  0.0,  0.0,
                           0.0,  0.0,         -1.0, 0.0,
                           -1.0, -1.22222222, -0.0, 1.0);
    #debug("result=%? expected=%?", result, expected);
    assert result.fuzzy_eq(expected);
}

