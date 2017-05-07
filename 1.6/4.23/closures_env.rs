fn main() {
    let num = 5;
    let plus_num = |x: i32| x + num;

    assert_eq!(10, plus_num(5));

    let mut num = 5;
    {
        let plus_num = |x: i32| x + num;
    }
    let y = &mut num;

    /*
    let nums = vec![1, 2, 3];
    let takes_nums = || nums;

    println!("{:?}", nums);
    */
}
