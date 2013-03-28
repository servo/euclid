use core::cmp::Eq;

#[deriving(Eq)]
pub struct Size2D<T> {
    width: T,
    height: T
}

pub fn Size2D<T: Copy>(width: T, height: T) -> Size2D<T> {
    return Size2D {
        width: width,
        height: height
    }
}

pub impl<T:Copy + Mul<T,T>> Size2D<T> {
    fn area(&self) -> T { self.width * self.height }
}

