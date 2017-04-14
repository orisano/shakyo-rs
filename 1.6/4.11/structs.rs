struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    let point = point;

    println!("The origin is at ({}, {})", point.x, point.y);
}
