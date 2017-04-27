fn main() {
    let s = "hello";

    // println!("The first letter of s is {}", s[0]);
    
    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}", c);
    }
    println!("");
}
