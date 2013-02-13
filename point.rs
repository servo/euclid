#[deriving_eq]
pub struct Point2D<T> {
    x: T,
    y: T
}

pub pure fn Point2D<T:Copy>(x: T, y: T) -> Point2D<T> {
    Point2D {x: x, y: y}
}


impl<T:Copy + Add<T,T>> Add<Point2D<T>, Point2D<T>> for Point2D<T> {
    pure fn add(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
}

impl<T:Copy + Sub<T,T>> Sub<Point2D<T>, Point2D<T>> for Point2D<T> {
    pure fn sub(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

