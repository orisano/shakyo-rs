fn main() {
    let num = 5;

    let owns_num = move |x: i32| x + num;

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;

        add_num(5);
    }

    assert_eq!(5, num);
}
