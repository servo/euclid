import num::num;

class Size2D<T:copy num> {
    let width: T;
    let height: T;

    new(width: T, height: T) {
        self.width = width;
        self.height = height;
    }

    fn area() -> T { self.width.mul(self.height) }
}

