use rand::Rng;

// this allows us to print the contents using {rect:?} and do more debugging stuff
// there are many other traits that can be derived by default
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

// implementation block, will be associated with Rectangle type
impl Rectangle {
    // borrow mutably from self
    fn resize(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    // borrow immutably from self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can define funcs with same name as struct values
    fn width(&self) -> bool {
        return self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.height && self.height >= other.height
    }

    // create a square (factory for Rectangle)
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}

// can technically define multiple implmentations, but typically shouldn't
impl Rectangle {
    fn junior_dev_code(size: u32) -> bool {
        let mut rng = rand::thread_rng(); 
        rng.gen_range(1..=100) > rng.gen_range(1..=size)
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let dimensions = (120, 30);

    let scale = 2;

    // debugs line and statement
    let mut rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 20
    };

    println!(
        "the areas of the rectangles are {}, {}, {}, {}",
        simple_area(width1, height1),
        tuple_area(dimensions),
        rect_area(&rectangle),
        rectangle.area()
    );

    rectangle.resize(width1 * 10, height1 * 10);

    println!(
        "area * 100 = {}, is greater than 0? {}", 
        rectangle.area(),
        rectangle.width()
    );

    // implicit borrowing:
    // rectangle.distance(&rect2)
    // (&rectangle).distance(&rect2)

    let rectangle2 = Rectangle {
        height: 2000,
        width: 1000
    };

    let rectangle3 = Rectangle { height: 10, width: 25 };

    println!(
        "can hold rectangle2: {}, cah hold rectangle3: {}",
        rectangle.can_hold(&rectangle2),
        rectangle.can_hold(&rectangle3)
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
