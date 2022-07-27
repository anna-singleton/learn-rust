#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("The area of r1 is {} px^2", rect1.area());
    println!("Can r1 hold t2? {}", rect1.can_hold(&rect2));
}
