fn main() {
    let v = 1;
    let v2 = v;
    println!("v is: {}", v);

    main_int();
    main_bool();
}

fn main_int() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main_bool() {
    let a = true;

    let _y = change_truth(a);
    println!("{}", a);
}

fn change_truth(x: bool) -> bool {
    !x
}

