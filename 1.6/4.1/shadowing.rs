fn main() {
    let x: i32 = 8;
    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 42;
    println!("{}", x);

    let mut x: i32 = 1;
    x = 7;
    let x = x;

    let y = 4;
    let y = "I can also be bound to text!";
}
