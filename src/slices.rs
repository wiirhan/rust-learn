pub fn run_slices() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
