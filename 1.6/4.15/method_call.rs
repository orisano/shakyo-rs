struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn reference(&self) {
        println!("taking self by reference!");
    }
    fn mutable_reference(&mut self) {
        println!("taking self by mutable reference!");
    }
    fn takes_ownership(self) {
        println!("taking ownership of self!");
    }
}
impl Circle {
    fn multi_impl(&self) {
        println!("taking self by reference!");
    }
}


fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}
