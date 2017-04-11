fn take(v: Vec<i32>) {
    // do something
}

fn main() {
    let v = vec![1, 2, 3];

    let v2 = v;

    println!("v[0] is: {}", v[0]); // use of moved value


    let v = vec![1, 2, 3];
    take(v);
    println!("v[0] is: {}", v[0]); // use of moved value
}

