struct Rect {
    width: usize,
    height: usize,
} impl Rect {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn perimeter(&self) -> usize {
        2 * (self.width + self.height)
    }
}

#[derive(Debug)]
struct Comp {
    re: f64,
    im: f64,
} impl Comp {
    fn new(re: f64, im: f64) -> Comp {
        Comp {
            re: re,
            im: im,
        }
    }
    fn conj(&self) -> Comp {
        Comp {
            re: self.re,
            im: -self.im,
        }
    }
}

fn main() {
    println!("henlo");
}