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
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
