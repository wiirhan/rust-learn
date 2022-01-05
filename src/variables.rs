use std::io;

pub fn run_variables() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is {}, y is {}", x, y);

    let sum = 5 + 10;
    println!("The value of sum is {}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);
    let product = 4 * 30;
    println!("The value of product is {}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {}", quotient);
    let remainder = 43 % 5;
    println!("The value of remainder is {}", remainder);

    let tup = (400, 6.4, 1);
    println!("The value of tup[1] is {}", tup.1);
    let (a, b, c) = tup;
    println!("The value of a,b,c is {},{},{}", a, b, c);

    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
