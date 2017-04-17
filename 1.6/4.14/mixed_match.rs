fn main() {
    struct Foo {
        x: Option<String>,
        y: Option<i32>,
    }

    let f = Foo { x: Some("hoge".to_string()), y: None };

    match f {
        Foo { x: Some(ref name), y: None } => println!("Hey {}", name),
        _ => println!("anything"),
    }
}
