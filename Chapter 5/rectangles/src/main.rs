fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width : dbg!(30*scale), 
        height : 50,
    };

    let rect2 = Rectangle {
        width : 40, 
        height : 70,
    };

    println!("The area of rect is {}", area(&rect1));
    // println!("rect1 is {}", rect1); // won't work, trait Display not implemented for Rectangle
    println!("rect1 is {:?}", rect1); // :? for pretty printing, but Debug is not implemented for Rect 
    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1); // useful when debuggin, actually printing values in nice way

    //adding methods to structs ---------------------------------------
    println!("The area of rect is {}", rect1.area());

    println!("Can rect1 hold rect2 inside ? {}", rect1.can_hold(&rect2));

    let _rect3 = Rectangle::square(20);
}

#[derive(Debug)] // enables trait Debug for rectangle
struct Rectangle {
    width : u32,
    height : u32,
}

//everything inside impl block is only for Rectangle struct
//methods can have same name as fields
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //functions without self, something like static functions
    fn square(size : u32) -> Self {
        Self {
            width : size, 
            height : size,
        }
    }
}

fn area(rectangle : &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}