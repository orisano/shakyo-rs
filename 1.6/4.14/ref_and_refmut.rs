fn main() {
    let x = 5;
    match x {
        ref r => println!("Got a reference to {}", r),
    }
    let mut x = 5;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}
