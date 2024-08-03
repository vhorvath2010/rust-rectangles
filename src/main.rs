struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Self {
            height: size,
            width: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    println!("The area of rect1 is {} square units.", rect1.area());

    let rect2 = Rectangle::square(25);

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect 2!")
    } else {
        println!("rect1 cannot hold rect 2!")
    }
}
