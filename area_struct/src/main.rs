#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
    let r1 = Rectangle {
    	width: 20,
	height: 50,
    };
    println!("r1 is a {:?}", r1);
    println!("Area of r1 = {}", area(&r1));
}

fn area(rectangle: &Rectangle) -> u32 {
	return rectangle.width * rectangle.height;
}

