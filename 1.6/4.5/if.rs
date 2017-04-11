fn main() {
    let x = 5;

    if x == 5 {
        println!("xは5です!");
    } else if x == 6 {
        println!("xは6です!");
    } else {
        println!("xは5でも6でもありません :(");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };
    let y = if x == 5 { 10 } else { 15 };
}
