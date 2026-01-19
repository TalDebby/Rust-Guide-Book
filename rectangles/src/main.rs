#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        let self_diagonal = ((self.width.pow(2) + self.height.pow(2)) as f64).sqrt();
        let other_diagonal = ((other.width.pow(2) + other.height.pow(2)) as f64).sqrt();
        self_diagonal >= other_diagonal
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle { width: 30, height: 40 * scale };
    let rect2 = Rectangle { width: 20, height: 30 };
    let square = Rectangle::square(14);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold square: {}", rect1.can_hold(&square));
}
