#[deriving(Eq)]
pub struct Point2D<T> {
    x: T,
    y: T
}

pub fn Point2D<T:Copy>(x: T, y: T) -> Point2D<T> {
    Point2D {x: x, y: y}
}


impl<T:Copy + Add<T,T>> Add<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn add(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
}

impl<T:Copy + Sub<T,T>> Sub<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn sub(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

