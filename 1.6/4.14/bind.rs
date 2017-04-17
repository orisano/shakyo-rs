fn main() {
    let x = 1;

    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

    let x = 5;

    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}
