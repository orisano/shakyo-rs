fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point {x: 0, y: 0};

    match origin {
        Point {x, y} => println!("({},{})", x, y),
    }

    match origin {
        Point {x: x1, y: y1} => println!("({},{})", x1, y1),
    }

    match origin {
        Point {x, .. } => println!("x is {}", x),
    }

    match origin {
        Point {y, .. } => println!("y is {}", y),
    }
}
