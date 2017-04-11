fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    42
}

fn bar(v: &Vec<i32>) {
    v.push(5); // borrowing object is immutable
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2);
}
