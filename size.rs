import num::Num;
import cmp::Eq;

struct Size2D<T:copy Num> {
    let width: T;
    let height: T;

    new(width: T, height: T) {
        self.width = width;
        self.height = height;
    }

    fn area() -> T { self.width.mul(self.height) }
}

impl<T: copy Num Eq> Size2D<T>: Eq {
    pure fn eq(&&other: Size2D<T>) -> bool {
        self.width == other.width && self.height == other.height
    }
}
