import num::Num;
import cmp::Eq;

struct Point2D<T:copy Num> {
    let x: T;
    let y: T;

    new(x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

impl<T: copy Num Eq> Point2D<T>: Eq {
    pure fn eq(&&other: Point2D<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}