fn main() {
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
    let a = [0; 20];

    println!("a has {} elements", a.len());

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);
}
