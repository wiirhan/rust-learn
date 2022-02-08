pub fn run_structs() {
    let user1 = User {
        username: String::from("someone"),
        email: String::from("some@xx.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        username: String::from("another"),
        ..user1
    };

    print!("{}", user1.username);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let back = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    struct AlwaysEqual;
    let subject = AlwaysEqual;

    get_area();

    compare_area();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn get_area() {
    let width = 30;
    let height = 50;
    let rect = Rectangle { width, height };

    println!("Area is {}", area(width, height));
    println!("Area is {}", tu_area((width, height)));
    println!("Area is {}", struct_area(&rect));

    println!("rect is {:#?}", rect);
    println!("Area is {:#?}", rect.area());

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// 元组参数
fn tu_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//  结构体参数
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> bool {
        size > 20
    }
}

fn compare_area() {
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
    println!(" {}", Rectangle::square(2));
}
