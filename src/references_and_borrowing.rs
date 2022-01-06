#![allow(unused)]
pub fn run_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    mut_change(&mut s);
    // error_mut();
    // error_mut2();

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    // 不允许修改引用的值
    // some_string.push_str("test");
}

fn mut_change(some_string: &mut String) {
    some_string.push_str("test");
}

fn error_mut() {
    // 不允许这样的可变引用
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}

fn error_mut2() {
    // let mut s = String::from("hello");
    //
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // // 不可变引用在被可变引用使用后，不能再使用
    // println!("{}, {}, and {}", r1, r2, r3);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
