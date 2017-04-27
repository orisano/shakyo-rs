fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

fn main() {
    let greeting = "Hello there."; // greeting: &'static str
    let s = "foo
        bar";

    assert_eq!("foo\n        bar", s);

    let s = "foo\
             bar";
    assert_eq!("foobar", s);

    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    let s = "Hello".to_string();
    takes_slice(&s);

    use std::net::TcpStream;

    TcpStream::connect("192.168.0.1:3000");

    let addr_string = "192.168.0.1:3000".to_string();
    TcpStream::connect(&*addr_string);
}
