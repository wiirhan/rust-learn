pub fn run_function() {
    println!("Hello, world!");

    another_function();

    param_function(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    println!("The value of y is {}", five());

    println!("The value of y is {}", plus_one(12));
}

fn another_function() {
    println!("Another function.");
}

fn param_function(x: i32) {
    println!("The value of x is :{}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
