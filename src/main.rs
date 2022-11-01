#[derive(Debug)]
struct Rectangle {            // new rectangle struct
    width: u32,
    height: u32,
}

impl Rectangle { // this houses the methods related to the rectangle struct so we wont need the fn area at the bottom of this code, highlighted out
    fn area(&self) ->u32 {
        self.width * self.height
    }
}

fn main() {

    let rect1 = Rectangle {   // new rect1 using rectangle struct and adding dimensions
        width: 30,
        height: 70,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()                                           // was area(&rect1) when fn area was used below
    );

    println!("{:#?}", rect1); // this shows the dimensions of the rectangle but needs the debug above and  the :? between the curly braces
}

//fn area(rectangle: &Rectangle) -> u32 { //new area function to multiply struct together
//    rectangle.width * rectangle.height
//}

