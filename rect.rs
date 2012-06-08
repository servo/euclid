import num::num;
import point::Point2D;
import size::Size2D;

class Rect<T:copy num> {
    let origin: Point2D<T>;
    let size: Size2D<T>;

    new(origin: Point2D<T>, size: Size2D<T>) {
        self.origin = copy origin;
        self.size = copy size;
    }
}

