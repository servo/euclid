use num::Num;
use cmp::Eq;

struct Size2D<T:Copy Num> {
    width: T,
    height: T
}

pure fn Size2D<T: Copy Num>(width: T, height: T) -> Size2D<T> {
    return Size2D {
        width: width,
        height: height
    }
}

impl<T:Copy Num> Size2D<T> {
    pure fn area() -> T { self.width.mul(self.height) }
}

impl<T: Copy Num Eq> Size2D<T>: Eq {
    pure fn eq(other: &Size2D<T>) -> bool {
        self.width == other.width && self.height == other.height
    }
    pure fn ne(other: &Size2D<T>) -> bool {
        !self.eq(other)
    }
}
