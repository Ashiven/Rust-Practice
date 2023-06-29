#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// we are not modifying self so &self will do
impl Rectangle {
    /*
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    /*
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    */
}
/*
fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }
}
*/
/*
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
*/
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}
//

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
