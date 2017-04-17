fn main() {
    let some_value: Result<i32, ()> = Ok(1);

    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),
    }

    fn coordinate() -> (i32, i32, i32) {
        (1, 2, 3)
    }

    let (x, _, z) = coordinate();

    println!("x is {}, z is {}", x, z);

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple"),
        OptionalTuple::Missing => println!("No such luck."),
    }
}
