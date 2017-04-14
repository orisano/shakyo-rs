struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Inches(i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}
