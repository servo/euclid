import num::Num;
import cmp::Eq;

struct Point2D<T:Copy Num> {
    x: T,
    y: T
}

fn Point2D<T: Copy Num>(x: T, y: T) -> Point2D<T> {
    return Point2D {x: x, y: y}
}


impl<T: Copy Num Eq> Point2D<T>: Eq {
    pure fn eq(&&other: Point2D<T>) -> bool {
        self.x == other.x && self.y == other.y
    }

    pure fn ne(&&other: Point2D<T>) -> bool {
        !self.eq(other)
    }
}