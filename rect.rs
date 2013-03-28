use point::Point2D;
use size::Size2D;
use core::cmp::{Eq, Ord};

#[deriving(Eq)]
pub struct Rect<T> {
    origin: Point2D<T>,
    size: Size2D<T>,
}

pub fn Rect<T:Copy + Ord + Add<T,T> + Sub<T,T>>(origin: Point2D<T>,
                                                size: Size2D<T>)
        -> Rect<T> {
    return Rect {
        origin: copy origin,
        size: copy size
    }
}

pub impl<T: Copy + Ord + Add<T,T> + Sub<T,T>> Rect<T> {
    fn intersects(&self, other: &Rect<T>) -> bool {
        self.origin.x < other.origin.x + other.size.width &&
       other.origin.x <  self.origin.x + self.size.width &&
        self.origin.y < other.origin.y + other.size.height &&
       other.origin.y <  self.origin.y + self.size.height
    }

    fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>> {
        if !self.intersects(other) {
            return None;
        }

        Some(Rect(Point2D(max(self.origin.x, other.origin.x),
                          max(self.origin.y, other.origin.y)),
                  Size2D(min(self.origin.x + self.size.width,
                             other.origin.x + other.size.width),
                         min(self.origin.y + self.size.height,
                             other.origin.y + other.size.height))))
    }

    fn union(&self, other: &Rect<T>) -> Rect<T> {
        let upper_left = Point2D(min(self.origin.x, other.origin.x),
                                 min(self.origin.y, other.origin.y));
        
        let lower_right = Point2D(max(self.origin.x + self.size.width,
                                      other.origin.x + other.size.width),
                                  max(self.origin.y + self.size.height,
                                      other.origin.y + other.size.height));
        
        Rect {
            origin: upper_left,
            size: Size2D(lower_right.x - upper_left.x, lower_right.y - upper_left.y)
        }
    }

    fn translate(&self, other: &Point2D<T>) -> Rect<T> {
        Rect {
            origin: Point2D(self.origin.x + other.x, self.origin.y + other.y),
            size: copy self.size
        }
    }
}

pub fn min<T:Copy + Ord>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

pub fn max<T:Copy + Ord>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}

#[test]
fn test_min_max() {
    fail_unless!(min(0, 1) == 0);
    fail_unless!(min(-1.0, 0.0) == -1.0);

    fail_unless!(max(0, 1) == 1);
    fail_unless!(max(-1.0, 0.0) == 0.0);
}

#[test]
fn test_translate() {
    let p = Rect(Point2D(0, 0), Size2D(50, 40));
    let pp = p.translate(&Point2D(10,15));

    fail_unless!(pp.size.width == 50);
    fail_unless!(pp.size.height == 40);
    fail_unless!(pp.origin.x == 10);
    fail_unless!(pp.origin.y == 15);


    let r = Rect(Point2D(-10, -5), Size2D(50, 40));
    let rr = r.translate(&Point2D(0,-10));

    fail_unless!(rr.size.width == 50);
    fail_unless!(rr.size.height == 40);
    fail_unless!(rr.origin.x == -10);
    fail_unless!(rr.origin.y == -15);
}

#[test]
fn test_union() {
    let p = Rect(Point2D(0,0), Size2D(50, 40));
    let q = Rect(Point2D(20,20), Size2D(5, 5));
    let r = Rect(Point2D(-15, -30), Size2D(200, 15));
    let s = Rect(Point2D(20, -15), Size2D(250, 200));

    let pq = p.union(&q);
    fail_unless!(pq.origin == Point2D(0, 0));
    fail_unless!(pq.size == Size2D(50, 40));

    let pr = p.union(&r);
    fail_unless!(pr.origin == Point2D(-15, -30));
    fail_unless!(pr.size == Size2D(200, 70));

    let ps = p.union(&s);
    fail_unless!(ps.origin == Point2D(0, -15));
    fail_unless!(ps.size == Size2D(270, 200));

}
