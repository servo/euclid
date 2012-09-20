use num::Num;
use point::Point2D;
use size::Size2D;
use cmp::{Eq, Ord};


struct Rect<T:Copy Num> {
    origin: Point2D<T>,
    size: Size2D<T>

}

pure fn Rect<T:Copy Num>(origin: Point2D<T>, size: Size2D<T>) -> Rect<T> {
    return Rect {
        origin: copy origin,
        size: copy size
    }
}

impl<T: Copy Num Ord> Rect<T> {
    pure fn intersects(&&other: &Rect<T>) -> bool {
        self.origin.x < other.origin.x.add(other.size.width) &&
       other.origin.x <  self.origin.x.add(self.size.width) &&
        self.origin.y < other.origin.y.add(other.size.height) &&
       other.origin.y <  self.origin.y.add(self.size.height)
    }
}

impl<T: Copy Num Ord> Rect<T> {
    pure fn intersection(other: &Rect<T>) -> Option<Rect<T>> {
        if !self.intersects(other) { return None }

        Some(Rect(Point2D(max(self.origin.x, other.origin.x),
                            max(self.origin.y, other.origin.y)),
                  Size2D(min(self.origin.x.add(self.size.width),
                            other.origin.x.add(other.size.width)),
                         min(self.origin.y.add(self.size.height),
                            other.origin.y.add(other.size.height)))
        ))
    }

    pure fn union(other: &Rect<T>) -> Rect<T> {
        Rect {
            origin: Point2D(min(self.origin.x, other.origin.x),
                            min(self.origin.y, other.origin.y)),
            size: Size2D(max(self.origin.x.add(self.size.width),
                            other.origin.x.add(other.size.width)),
                         max(self.origin.y.add(self.size.height),
                            other.origin.y.add(other.size.height)))
        }
    }
}

impl<T: Copy Num> Rect<T> {
    pure fn translate(other: &Point2D<T>) -> Rect<T> {
        Rect {
            origin: Point2D(self.origin.x.add(other.x),
                            self.origin.y.add(other.y)),
            size: copy self.size
        }
    }
}

impl<T: Copy Num Eq> Rect<T>: Eq {
    pure fn eq(other: &Rect<T>) -> bool {
        self.origin == other.origin && self.size == other.size
    }
    pure fn ne(other: &Rect<T>) -> bool {
        !self.eq(other)
    }
}

pure fn min<T: Copy Num Ord>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

pure fn max<T: Copy Num Ord>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}
