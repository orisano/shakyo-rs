fn main() {
    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = '💅';
    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }

}
