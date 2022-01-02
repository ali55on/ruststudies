#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn has_width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Funções associadas não precisam da instancia 'self'
    // e são acessadas pelo operador de :: onde recebem o namespace
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\n-------------");
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.has_width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    // Ref
    println!("\n-------------");
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

    // Funções associadas
    println!("\n-------------");

    let my_square = Rectangle::square(10);
    println!("square width: {}", my_square.width);
    println!("square height: {}", my_square.height);
}