use num::Num;
use cmp::Eq;

struct Point2D<T:Copy Num> {
    x: T,
    y: T
}

pure fn Point2D<T: Copy Num>(x: T, y: T) -> Point2D<T> {
    return Point2D {x: x, y: y}
}


impl<T: Copy Num> Point2D<T> {
    pure fn add(&&other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
    pure fn sub(&&other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

impl<T: Copy Num Eq> Point2D<T>: Eq {
    pure fn eq(&&other: Point2D<T>) -> bool {
        self.x == other.x && self.y == other.y
    }

    pure fn ne(&&other: Point2D<T>) -> bool {
        !self.eq(other)
    }
}