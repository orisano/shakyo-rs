fn foo(x: &str) {
    println!("foo {}", x);
}

fn bar() {
    println!("bar");
}

fn main() {
    let option = Some("hoge");
    // let option = None;
    match option {
        Some(x) => { foo(x) },
        None => {},
    }

    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    if let Some(x) = option {
        foo(x);
    }

    if let Some(x) = option {
        foo(x);
    } else {
        bar();
    }
}
