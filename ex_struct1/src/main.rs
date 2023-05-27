//Refectoring with Structs: Adding more Meaning
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 35,
        height: 55,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);


    println!(
        "The area of the rectangle1 is {} square pixels,
        rectangle2 is {}",
        area(&rect1), area(&rect2)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}



/*//Refactoring with Tuples
fn main() {
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}*/

/*fn main() { //The basic way
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}*/
