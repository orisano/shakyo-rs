trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("this is silly");

        *self as f64
    }
}

fn main() {
    println!("{}", 5.area());

    use std::io::Write;

    let mut f = std::fs::File::open("foo.txt").expect("Couldn't open foo.txt");
    let buf = b"whatever";
    let result = f.write(buf);
}
