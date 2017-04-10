fn main() {
    let x = (1, "hello");
    let mut x = (1, 2);
    let y = (2, 3);

    x = y;

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);
}
