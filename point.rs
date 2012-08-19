import num::Num;

struct Point2D<T:copy Num> {
    let x: T;
    let y: T;

    new(x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

