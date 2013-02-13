use cmp::Eq;

#[deriving_eq]
pub struct Size2D<T> {
    width: T,
    height: T
}

pub pure fn Size2D<T: Copy>(width: T, height: T) -> Size2D<T> {
    return Size2D {
        width: width,
        height: height
    }
}

impl<T:Copy + Mul<T,T>> Size2D<T> {
    pure fn area() -> T { self.width * self.height }
}

