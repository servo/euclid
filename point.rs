import num::num;

class Point2D<T:copy num> {
    let x: T;
    let y: T;

    new(x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

