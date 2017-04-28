fn main() {
    let dog = "hachiko";
    let hachi = &dog[0..5];
    println!("{}", hachi);

    let dog = "忠犬ハチ公";
    let hachi = &dog[0..2];
    println!("{}", hachi);
}
