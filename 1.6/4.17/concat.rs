fn main() {
    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
    println!("{}", hello_world);

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;
    println!("{}", hello_world);
}
