pub fn run_ownership() {
    // 数据放在堆上
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    // 数据移动，只移动栈数据、不移动堆数据
    let s1 = String::from("hello");
    let s2 = s1;
    // error
    // print!("{}", s1);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{},{}", s3, s4);

    // 数据放在栈上
    let x = 5;
    let y = x;
    print!("{},{}", x, y);

    let str = String::from("hello"); // str 进入作用域

    takes_ownership(str); // str 的值移动到函数里 ...
                          // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
