#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}
fn main() {
    
    let rect1 = Rectangle{width:30, height:50};
    println!("The area of  the rect is  {:?} square pixels", rect1);
}

fn area(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}


