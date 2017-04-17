fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 'x';
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c)
    }

    println!("x: {}", x);
}
