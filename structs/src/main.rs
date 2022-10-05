// Allows for automatic printing of object members.
#[derive(Debug)]

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn struct_area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    // Using primitive datatypes.
    let height = 50;
    let width = 30;
    println!("The values are: {} and {}", height, width);
    println!("The area is {} pixels.", prim_area(height, width));
    // Using a tuple.
    let tup_rect = (50, 30);
    // {:?} is the single line pretty print flag.
    println!("The tuple is {:?}", tup_rect);
    println!("The area is {} pixels.", tup_area(tup_rect));
    // Using a struct.
    let struct_rect = Rectangle {
        height: 50,
        width: 30,
    };
    // {:#?} is the multi-line pretty print flag.
    println!("struct_rect is {:#?}", struct_rect);
    println!("The area is {} pixels.", struct_rect.struct_area());
    let other_rect = Rectangle {
        height: 20,
        width: 10,
    };
    if struct_rect.can_hold(&other_rect) {
        println!("The other rectangle will fit inside.");
    } else {
        println!("The other rectangle will not fit inside.");
    }
}

fn prim_area(height: u32, width: u32) -> u32 {
    height * width
}

fn tup_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
