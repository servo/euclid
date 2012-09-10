import num::Num;
import point::Point2D;
import size::Size2D;
import cmp::Eq;

struct Rect<T:Copy Num> {
    origin: Point2D<T>,
    size: Size2D<T>

}

fn Rect<T:Copy Num>(origin: Point2D<T>, size: Size2D<T>) -> Rect<T> {
    return Rect {
        origin: copy origin,
        size: copy size
    }
}

impl<T: Copy Num Eq> Rect<T>: Eq {
    pure fn eq(&&other: Rect<T>) -> bool {
        self.origin == other.origin && self.size == other.size
    }
    pure fn ne(&&other: Rect<T>) -> bool {
        !self.eq(other)
    }
}