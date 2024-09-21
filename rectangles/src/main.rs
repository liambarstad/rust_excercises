// this allows us to print the contents using {rect:?} and do more debugging stuff
// there are many other traits that can be derived by default
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let dimensions = (120, 30);

    let scale = 2;

    // debugs line and statement
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 20
    };

    println!(
        "the areas of the rectangles are {}, {}, {}",
        simple_area(width1, height1),
        tuple_area(dimensions),
        rect_area(&rectangle)
    );

    println!(
        "the rectangle is {rectangle:?}"
    );

    // debugs struct
    dbg!(&rectangle);
}

fn simple_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
